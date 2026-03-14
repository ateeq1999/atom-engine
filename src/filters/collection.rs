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
