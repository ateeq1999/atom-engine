use crate::types::value::Value;
use serde_json;

pub fn is_defined(v: &Value) -> Value {
    Value::Bool(!matches!(v, Value::Null))
}

pub fn is_null(v: &Value) -> Value {
    Value::Bool(matches!(v, Value::Null))
}

pub fn is_empty(v: &Value) -> Value {
    Value::Bool(match v {
        Value::Null => true,
        Value::Str(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
        _ => false,
    })
}

pub fn is_string(v: &Value) -> Value {
    Value::Bool(matches!(v, Value::Str(_)))
}

pub fn is_number(v: &Value) -> Value {
    Value::Bool(matches!(v, Value::Num(_)))
}

pub fn is_bool(v: &Value) -> Value {
    Value::Bool(matches!(v, Value::Bool(_)))
}

pub fn is_array(v: &Value) -> Value {
    Value::Bool(matches!(v, Value::Array(_)))
}

pub fn is_object(v: &Value) -> Value {
    Value::Bool(matches!(v, Value::Object(_)))
}

pub fn default_val(v: &Value, fallback: Value) -> Value {
    if is_defined(v).is_truthy() && !is_null(v).is_truthy() {
        v.clone()
    } else {
        fallback
    }
}

pub fn coalesce(values: &[Value]) -> Value {
    for v in values {
        if is_defined(v).is_truthy() && !is_null(v).is_truthy() {
            return v.clone();
        }
    }
    Value::Null
}

pub fn str_val(v: &Value) -> Value {
    Value::Str(v.coerce_str())
}

pub fn int_val(v: &Value) -> Value {
    if let Some(n) = v.as_f64() {
        Value::Num(n.trunc())
    } else {
        Value::Num(0.0)
    }
}

pub fn float_val(v: &Value) -> Value {
    v.as_f64().map(Value::Num).unwrap_or(Value::Num(0.0))
}

pub fn bool_val(v: &Value) -> Value {
    Value::Bool(v.is_truthy())
}

pub fn json_val(v: &Value) -> Value {
    match serde_json::to_string(v) {
        Ok(s) => Value::Str(s),
        Err(_) => Value::Null,
    }
}

pub fn parse_json(s: &str) -> Value {
    match serde_json::from_str(s) {
        Ok(v) => v,
        Err(_) => Value::Null,
    }
}

pub fn inspect(v: &Value) -> Value {
    Value::Str(format!("{:#?}", v))
}

pub fn type_of(v: &Value) -> Value {
    Value::Str(v.type_name().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_defined() {
        assert!(is_defined(&Value::Num(1.0)).is_truthy());
        assert!(!is_defined(&Value::Null).is_truthy());
    }

    #[test]
    fn test_is_null() {
        assert!(is_null(&Value::Null).is_truthy());
        assert!(!is_null(&Value::Num(1.0)).is_truthy());
    }

    #[test]
    fn test_is_empty() {
        assert!(is_empty(&Value::Str("".to_string())).is_truthy());
        assert!(is_empty(&Value::Array(vec![])).is_truthy());
        assert!(!is_empty(&Value::Str("x".to_string())).is_truthy());
    }

    #[test]
    fn test_type_of() {
        assert_eq!(
            type_of(&Value::Str("hi".to_string())),
            Value::Str("string".to_string())
        );
        assert_eq!(type_of(&Value::Num(1.0)), Value::Str("number".to_string()));
    }

    #[test]
    fn test_coalesce() {
        let values = vec![Value::Null, Value::Null, Value::Str("found".to_string())];
        assert_eq!(coalesce(&values), Value::Str("found".to_string()));
    }

    #[test]
    fn test_json_roundtrip() {
        let original = Value::Object(
            vec![("key".to_string(), Value::Str("value".to_string()))]
                .into_iter()
                .collect(),
        );
        let json_str = json_val(&original);
        if let Value::Str(s) = json_str {
            let parsed = parse_json(&s);
            assert_eq!(original, parsed);
        } else {
            panic!("Expected string");
        }
    }
}
