use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

use super::FilterResult;

pub fn first(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(arr) = value.as_array() {
        Ok(arr.first().cloned().unwrap_or(Value::Null))
    } else {
        Ok(value.clone())
    }
}

pub fn last(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(arr) = value.as_array() {
        Ok(arr.last().cloned().unwrap_or(Value::Null))
    } else {
        Ok(value.clone())
    }
}

pub fn length(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    let len = match value {
        Value::Array(arr) => arr.len(),
        Value::Object(obj) => obj.len(),
        Value::String(s) => s.chars().count(),
        _ => 0,
    };
    Ok(Value::Number(len.into()))
}

pub fn reverse(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(arr) = value.as_array().cloned() {
        let mut rev = arr;
        rev.reverse();
        Ok(Value::Array(rev))
    } else if let Some(s) = value.as_str() {
        Ok(Value::String(s.chars().rev().collect()))
    } else {
        Ok(value.clone())
    }
}

pub fn sort(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(mut arr) = value.as_array().cloned() {
        arr.sort_by(|a, b| {
            let a_str = serde_json::to_string(a).unwrap_or_default();
            let b_str = serde_json::to_string(b).unwrap_or_default();
            a_str.cmp(&b_str)
        });
        Ok(Value::Array(arr))
    } else {
        Ok(value.clone())
    }
}

pub fn group_by(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let arr = value
        .as_array()
        .ok_or_else(|| Error::msg("Expected array"))?;
    let key = args
        .get("attribute")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("Missing attribute"))?;

    let mut groups: HashMap<String, Vec<Value>> = HashMap::new();
    for item in arr {
        let group_key = item
            .get(key)
            .map(|v| serde_json::to_string(v).unwrap_or_default())
            .unwrap_or_default();
        groups.entry(group_key).or_default().push(item.clone());
    }

    Ok(Value::Object(
        groups
            .into_iter()
            .map(|(k, v)| (k, serde_json::json!(v)))
            .collect(),
    ))
}

pub fn where_filter(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let arr = value
        .as_array()
        .ok_or_else(|| Error::msg("Expected array"))?;
    let key = args.get("attribute").and_then(|v| v.as_str());
    let filter_value = args.get("value").cloned();

    let result: Vec<Value> = arr
        .iter()
        .filter(|item| {
            if let (Some(key), Some(fv)) = (key, &filter_value) {
                item.get(key) == Some(fv)
            } else {
                item.is_object() && !item.as_object().map(|o| o.is_empty()).unwrap_or(true)
            }
        })
        .cloned()
        .collect();

    Ok(Value::Array(result))
}

pub fn pluck(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let arr = value
        .as_array()
        .ok_or_else(|| Error::msg("Expected array"))?;
    let key = args
        .get("attribute")
        .and_then(|v| v.as_str())
        .ok_or_else(|| Error::msg("Missing attribute"))?;

    let result: Vec<Value> = arr
        .iter()
        .filter_map(|item| item.get(key).cloned())
        .collect();
    Ok(Value::Array(result))
}

pub fn join(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let sep = args
        .get("separator")
        .and_then(|v| v.as_str())
        .unwrap_or(",");
    if let Some(arr) = value.as_array() {
        let joined = arr
            .iter()
            .map(|v| v.as_str().unwrap_or(""))
            .collect::<Vec<_>>()
            .join(sep);
        Ok(Value::String(joined))
    } else {
        Ok(value.clone())
    }
}

pub fn slice(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let start = args.get("start").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let length = args.get("length").and_then(|v| v.as_u64()).unwrap_or(1) as usize;

    if let Some(arr) = value.as_array() {
        let slice: Vec<Value> = arr.iter().skip(start).take(length).cloned().collect();
        Ok(Value::Array(slice))
    } else if let Some(s) = value.as_str() {
        let chars: String = s.chars().skip(start).take(length).collect();
        Ok(Value::String(chars))
    } else {
        Ok(value.clone())
    }
}

pub fn uniq(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(arr) = value.as_array() {
        let mut seen = std::collections::HashSet::new();
        let unique: Vec<Value> = arr
            .iter()
            .filter(|v| seen.insert(v.to_string()))
            .cloned()
            .collect();
        Ok(Value::Array(unique))
    } else {
        Ok(value.clone())
    }
}

pub fn shuffle(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    if let Some(arr) = value.as_array() {
        let mut shuffled = arr.clone();
        let mut rng = thread_rng();
        shuffled.shuffle(&mut rng);
        Ok(Value::Array(shuffled))
    } else {
        Ok(value.clone())
    }
}

pub fn map_filter(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let arr = value
        .as_array()
        .ok_or_else(|| Error::msg("Expected array"))?;
    let prop = args.get("prop").and_then(|v| v.as_str()).unwrap_or("value");
    let transform = args.get("transform").and_then(|v| v.as_str());

    #[allow(clippy::needless_borrows_for_generic_args)]
    let result: Vec<Value> = arr
        .iter()
        .map(|item| {
            if let Some(transform_fn) = transform {
                match transform_fn {
                    "upper" => {
                        if let Some(v) = item.get(&prop) {
                            Value::String(v.as_str().unwrap_or("").to_uppercase())
                        } else {
                            item.clone()
                        }
                    }
                    "lower" => {
                        if let Some(v) = item.get(&prop) {
                            Value::String(v.as_str().unwrap_or("").to_lowercase())
                        } else {
                            item.clone()
                        }
                    }
                    "length" => {
                        if let Some(v) = item.get(&prop) {
                            Value::Number(v.as_array().map(|a| a.len()).unwrap_or(0).into())
                        } else {
                            Value::Number(0.into())
                        }
                    }
                    _ => item.get(&prop).cloned().unwrap_or(Value::Null),
                }
            } else {
                item.get(&prop).cloned().unwrap_or(Value::Null)
            }
        })
        .collect();

    Ok(Value::Array(result))
}

pub fn filter_filter(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let arr = value
        .as_array()
        .ok_or_else(|| Error::msg("Expected array"))?;
    let key = args.get("key").and_then(|v| v.as_str());
    let value_filter = args.get("value").cloned();
    let op = args.get("op").and_then(|v| v.as_str()).unwrap_or("eq");

    let result: Vec<Value> = arr
        .iter()
        .filter(|item| {
            if let (Some(k), Some(fv)) = (key, &value_filter) {
                let item_val = item.get(k);
                match op {
                    "eq" => item_val == Some(fv),
                    "ne" => item_val != Some(fv),
                    "gt" => {
                        if let (Some(iv), Some(f)) =
                            (item_val.and_then(|v| v.as_f64()), fv.as_f64())
                        {
                            iv > f
                        } else {
                            false
                        }
                    }
                    "gte" => {
                        if let (Some(iv), Some(f)) =
                            (item_val.and_then(|v| v.as_f64()), fv.as_f64())
                        {
                            iv >= f
                        } else {
                            false
                        }
                    }
                    "lt" => {
                        if let (Some(iv), Some(f)) =
                            (item_val.and_then(|v| v.as_f64()), fv.as_f64())
                        {
                            iv < f
                        } else {
                            false
                        }
                    }
                    "lte" => {
                        if let (Some(iv), Some(f)) =
                            (item_val.and_then(|v| v.as_f64()), fv.as_f64())
                        {
                            iv <= f
                        } else {
                            false
                        }
                    }
                    "contains" => {
                        if let Some(iv) = item_val {
                            iv.to_string().contains(&fv.to_string())
                        } else {
                            false
                        }
                    }
                    "exists" => item.is_object() && item.get(k).is_some(),
                    _ => item_val == Some(fv),
                }
            } else {
                !item.is_null() && !item.as_array().map(|a| a.is_empty()).unwrap_or(false)
            }
        })
        .cloned()
        .collect();

    Ok(Value::Array(result))
}

pub fn each_filter(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let arr = value
        .as_array()
        .ok_or_else(|| Error::msg("Expected array"))?;
    let include_index = args.get("index").and_then(|v| v.as_bool()).unwrap_or(false);

    if include_index {
        let result: Vec<Value> = arr
            .iter()
            .enumerate()
            .map(|(i, v)| serde_json::json!({"index": i, "value": v, "first": i == 0, "last": i == arr.len() - 1}))
            .collect();
        Ok(Value::Array(result))
    } else {
        Ok(Value::Array(arr.clone()))
    }
}

pub fn reduce_filter(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let arr = value
        .as_array()
        .ok_or_else(|| Error::msg("Expected array"))?;
    let initial = args.get("initial").cloned().unwrap_or(Value::Null);
    let prop = args.get("prop").and_then(|v| v.as_str());

    let result = arr.iter().fold(initial, |acc, item| {
        if let Some(p) = prop {
            if let Some(v) = item.get(p) {
                if let (Some(a), Some(b)) = (acc.as_f64(), v.as_f64()) {
                    return serde_json::json!(a + b);
                }
            }
        }
        acc
    });

    Ok(result)
}

pub fn flatten_filter(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(arr) = value.as_array() {
        let mut result = Vec::new();
        for item in arr {
            if let Some(inner) = item.as_array() {
                for i in inner {
                    result.push(i.clone());
                }
            } else {
                result.push(item.clone());
            }
        }
        Ok(Value::Array(result))
    } else {
        Ok(value.clone())
    }
}

pub fn partition_filter(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let arr = value
        .as_array()
        .ok_or_else(|| Error::msg("Expected array"))?;
    let key = args.get("key").and_then(|v| v.as_str());
    let value_filter = args.get("value").cloned();

    let (matched, rest): (Vec<&Value>, Vec<&Value>) = arr.iter().partition(|item| {
        if let (Some(k), Some(fv)) = (key, &value_filter) {
            item.get(k) == Some(fv)
        } else {
            !item.is_null()
        }
    });

    Ok(serde_json::json!({
        "matched": matched.iter().map(|v| (*v).clone()).collect::<Vec<_>>(),
        "rest": rest.iter().map(|v| (*v).clone()).collect::<Vec<_>>()
    }))
}
