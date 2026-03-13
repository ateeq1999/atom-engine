use crate::types::value::Value;
use indexmap::IndexMap;

pub fn array_len(arr: &[Value]) -> Value {
    Value::Num(arr.len() as f64)
}

pub fn array_is_empty(arr: &[Value]) -> Value {
    Value::Bool(arr.is_empty())
}

pub fn array_first(arr: &[Value]) -> Value {
    arr.first().cloned().unwrap_or(Value::Null)
}

pub fn array_last(arr: &[Value]) -> Value {
    arr.last().cloned().unwrap_or(Value::Null)
}

pub fn array_first_n(arr: &[Value], n: usize) -> Value {
    Value::Array(arr.iter().take(n).cloned().collect())
}

pub fn array_last_n(arr: &[Value], n: usize) -> Value {
    let start = arr.len().saturating_sub(n);
    Value::Array(arr[start..].to_vec())
}

pub fn array_nth(arr: &[Value], i: usize) -> Value {
    arr.get(i).cloned().unwrap_or(Value::Null)
}

pub fn array_slice(arr: &[Value], start: usize, end: Option<usize>) -> Value {
    let e = end.unwrap_or(arr.len());
    Value::Array(arr[start..e].to_vec())
}

pub fn array_take(arr: &[Value], n: usize) -> Value {
    array_first_n(arr, n)
}

pub fn array_skip(arr: &[Value], n: usize) -> Value {
    Value::Array(arr.iter().skip(n).cloned().collect())
}

pub fn array_includes(arr: &[Value], val: &Value) -> Value {
    Value::Bool(arr.contains(val))
}

pub fn array_index_of(arr: &[Value], val: &Value) -> Value {
    Value::Num(
        arr.iter()
            .position(|v| v == val)
            .map(|i| i as f64)
            .unwrap_or(-1.0),
    )
}

pub fn array_push(arr: &[Value], val: Value) -> Value {
    let mut new_arr = arr.to_vec();
    new_arr.push(val);
    Value::Array(new_arr)
}

pub fn array_prepend(arr: &[Value], val: Value) -> Value {
    let mut new_arr = vec![val];
    new_arr.extend(arr.iter().cloned());
    Value::Array(new_arr)
}

pub fn array_concat(arr: &[Value], other: &[Value]) -> Value {
    let mut new_arr = arr.to_vec();
    new_arr.extend(other.iter().cloned());
    Value::Array(new_arr)
}

pub fn array_flatten(arr: &[Value]) -> Value {
    let mut result = Vec::new();
    for item in arr {
        if let Value::Array(inner) = item {
            for i in inner {
                result.push(i.clone());
            }
        } else {
            result.push(item.clone());
        }
    }
    Value::Array(result)
}

pub fn array_unique(arr: &[Value]) -> Value {
    let mut result = Vec::new();
    for item in arr {
        if !result.contains(item) {
            result.push(item.clone());
        }
    }
    Value::Array(result)
}

pub fn array_unique_by(arr: &[Value], _key: &str) -> Value {
    // Simplified - just unique for now
    array_unique(arr)
}

pub fn array_compact(arr: &[Value]) -> Value {
    Value::Array(arr.iter().filter(|v| !v.is_truthy()).cloned().collect())
}

pub fn array_sort(arr: &[Value]) -> Value {
    let mut sorted = arr.to_vec();
    sorted.sort_by(|a, b| a.coerce_str().cmp(&b.coerce_str()));
    Value::Array(sorted)
}

pub fn array_reverse(arr: &[Value]) -> Value {
    let mut rev = arr.to_vec();
    rev.reverse();
    Value::Array(rev)
}

pub fn array_join(arr: &[Value], sep: &str) -> Value {
    Value::Str(
        arr.iter()
            .map(|v| v.coerce_str())
            .collect::<Vec<_>>()
            .join(sep),
    )
}

pub fn array_join_with(arr: &[Value], sep: &str, last_sep: Option<&str>) -> Value {
    if arr.len() <= 1 {
        return Value::Str(arr.first().map(|v| v.coerce_str()).unwrap_or_default());
    }
    let last = last_sep.unwrap_or(sep);
    let first = &arr[..arr.len() - 1];
    let last_elem = arr.last().unwrap();
    Value::Str(format!(
        "{}{}{}",
        first
            .iter()
            .map(|v| v.coerce_str())
            .collect::<Vec<_>>()
            .join(sep),
        last,
        last_elem.coerce_str()
    ))
}

pub fn array_sum(arr: &[Value]) -> Value {
    let sum: f64 = arr.iter().filter_map(|v| v.as_f64()).sum();
    Value::Num(sum)
}

pub fn array_sum_key(arr: &[Value], _key: &str) -> Value {
    // Simplified - sum all
    array_sum(arr)
}

pub fn array_min(arr: &[Value]) -> Value {
    arr.iter()
        .filter_map(|v| v.as_f64())
        .fold(Value::Null, |acc, n| match acc {
            Value::Null => Value::Num(n),
            Value::Num(m) => Value::Num(m.min(n)),
            _ => Value::Null,
        })
}

pub fn array_max(arr: &[Value]) -> Value {
    arr.iter()
        .filter_map(|v| v.as_f64())
        .fold(Value::Null, |acc, n| match acc {
            Value::Null => Value::Num(n),
            Value::Num(m) => Value::Num(m.max(n)),
            _ => Value::Null,
        })
}

pub fn array_avg(arr: &[Value]) -> Value {
    let nums: Vec<f64> = arr.iter().filter_map(|v| v.as_f64()).collect();
    if nums.is_empty() {
        Value::Num(0.0)
    } else {
        Value::Num(nums.iter().sum::<f64>() / nums.len() as f64)
    }
}

pub fn array_chunk(arr: &[Value], size: usize) -> Value {
    let chunks: Vec<Value> = arr.chunks(size).map(|c| Value::Array(c.to_vec())).collect();
    Value::Array(chunks)
}

pub fn array_zip(arr: &[Value], other: &[Value]) -> Value {
    let len = arr.len().min(other.len());
    let result: Vec<Value> = (0..len)
        .map(|i| Value::Array(vec![arr[i].clone(), other[i].clone()]))
        .collect();
    Value::Array(result)
}

pub fn object_keys(obj: &IndexMap<String, Value>) -> Value {
    Value::Array(obj.keys().map(|k| Value::Str(k.clone())).collect())
}

pub fn object_values(obj: &IndexMap<String, Value>) -> Value {
    Value::Array(obj.values().cloned().collect())
}

pub fn object_entries(obj: &IndexMap<String, Value>) -> Value {
    Value::Array(
        obj.iter()
            .map(|(k, v)| Value::Array(vec![Value::Str(k.clone()), v.clone()]))
            .collect(),
    )
}

pub fn object_has(obj: &IndexMap<String, Value>, key: &str) -> Value {
    Value::Bool(obj.contains_key(key))
}

pub fn object_get(obj: &IndexMap<String, Value>, key: &str, default: Option<Value>) -> Value {
    obj.get(key)
        .cloned()
        .unwrap_or(default.unwrap_or(Value::Null))
}

pub fn object_merge(obj: &IndexMap<String, Value>, other: &IndexMap<String, Value>) -> Value {
    let mut new_obj = obj.clone();
    for (k, v) in other {
        new_obj.insert(k.clone(), v.clone());
    }
    Value::Object(new_obj)
}

pub fn object_pick(obj: &IndexMap<String, Value>, keys: &[Value]) -> Value {
    let mut result = IndexMap::new();
    for key in keys {
        if let Value::Str(k) = key {
            if let Some(v) = obj.get(k) {
                result.insert(k.clone(), v.clone());
            }
        }
    }
    Value::Object(result)
}

pub fn object_omit(obj: &IndexMap<String, Value>, keys: &[Value]) -> Value {
    let omit_keys: Vec<String> = keys
        .iter()
        .filter_map(|k| {
            if let Value::Str(s) = k {
                Some(s.clone())
            } else {
                None
            }
        })
        .collect();
    let result = obj
        .iter()
        .filter(|(k, _)| !omit_keys.contains(k))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    Value::Object(result)
}

pub fn object_len(obj: &IndexMap<String, Value>) -> Value {
    Value::Num(obj.len() as f64)
}

pub fn object_is_empty(obj: &IndexMap<String, Value>) -> Value {
    Value::Bool(obj.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arr(values: Vec<Value>) -> Vec<Value> {
        values
    }

    #[test]
    fn test_array_len() {
        assert_eq!(
            array_len(&arr(vec![Value::Num(1.0), Value::Num(2.0)])),
            Value::Num(2.0)
        );
    }

    #[test]
    fn test_array_first() {
        assert_eq!(
            array_first(&arr(vec![Value::Num(1.0), Value::Num(2.0)])),
            Value::Num(1.0)
        );
    }

    #[test]
    fn test_array_join() {
        let arr = arr(vec![
            Value::Str("a".to_string()),
            Value::Str("b".to_string()),
        ]);
        assert_eq!(array_join(&arr, ", "), Value::Str("a, b".to_string()));
    }

    #[test]
    fn test_object_keys() {
        let mut obj = IndexMap::new();
        obj.insert("a".to_string(), Value::Num(1.0));
        obj.insert("b".to_string(), Value::Num(2.0));
        let result = object_keys(&obj);
        assert!(matches!(result, Value::Array(arr) if arr.len() == 2));
    }
}
