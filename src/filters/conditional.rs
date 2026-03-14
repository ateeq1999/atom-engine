use serde_json::Value;
use std::collections::HashMap;

use super::FilterResult;

pub fn when(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let condition = value.as_bool().unwrap_or(false);
    let then_val = args.get("then").cloned().unwrap_or(Value::Null);
    let else_val = args.get("else").cloned().unwrap_or(Value::Null);
    Ok(if condition { then_val } else { else_val })
}

pub fn default_filter(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let is_falsy = value.is_null()
        || (value.as_bool() == Some(false))
        || (value.as_array().map(|a| a.is_empty()).unwrap_or(false))
        || (value.as_object().map(|o| o.is_empty()).unwrap_or(false));

    if is_falsy {
        Ok(args.get("value").cloned().unwrap_or(Value::Null))
    } else {
        Ok(value.clone())
    }
}

pub fn coalesce(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    if !value.is_null() {
        return Ok(value.clone());
    }

    for i in 0.. {
        if let Some(v) = args.get(&format!("{}", i)) {
            if !v.is_null() {
                return Ok(v.clone());
            }
        } else {
            break;
        }
    }

    Ok(Value::Null)
}

pub fn defined(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    Ok(Value::Bool(!value.is_null()))
}

pub fn undefined(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    Ok(Value::Bool(value.is_null()))
}

pub fn empty(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    let is_empty = match value {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
        _ => false,
    };
    Ok(Value::Bool(is_empty))
}

pub fn not_empty(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    let is_empty = match value {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
        _ => false,
    };
    Ok(Value::Bool(!is_empty))
}
