use crate::types::value::Value;
use indexmap::IndexMap;

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Bool(self)
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Num(self)
    }
}

impl IntoValue for i64 {
    fn into_value(self) -> Value {
        Value::Num(self as f64)
    }
}

impl IntoValue for i32 {
    fn into_value(self) -> Value {
        Value::Num(self as f64)
    }
}

impl IntoValue for usize {
    fn into_value(self) -> Value {
        Value::Num(self as f64)
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::Str(self)
    }
}

impl IntoValue for &str {
    fn into_value(self) -> Value {
        Value::Str(self.to_string())
    }
}

impl IntoValue for Vec<Value> {
    fn into_value(self) -> Value {
        Value::Array(self)
    }
}

impl IntoValue for IndexMap<String, Value> {
    fn into_value(self) -> Value {
        Value::Object(self)
    }
}

impl<T: IntoValue> IntoValue for Option<T> {
    fn into_value(self) -> Value {
        match self {
            Some(v) => v.into_value(),
            None => Value::Null,
        }
    }
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Num(n) => Some(*n),
            Value::Str(s) => s.parse().ok(),
            Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            Value::Num(n) => Some(*n != 0.0),
            Value::Str(s) => match s.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => Some(true),
                "false" | "0" | "no" | "off" | "" => Some(false),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&IndexMap<String, Value>> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn coerce_str(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Num(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    (*n as i64).to_string()
                } else {
                    n.to_string()
                }
            }
            Value::Str(s) => s.clone(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.coerce_str()).collect();
                items.join(", ")
            }
            Value::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.coerce_str()))
                    .collect();
                items.join(", ")
            }
            Value::Date(d) => d.to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_value_primitives() {
        assert_eq!(IntoValue::into_value(true), Value::Bool(true));
        assert_eq!(IntoValue::into_value(42i64), Value::Num(42.0));
        assert_eq!(
            IntoValue::into_value("hello"),
            Value::Str("hello".to_string())
        );
    }

    #[test]
    fn test_as_str() {
        assert_eq!(Value::Str("test".to_string()).as_str(), Some("test"));
        assert_eq!(Value::Num(1.0).as_str(), None);
    }

    #[test]
    fn test_as_f64() {
        assert_eq!(Value::Num(42.0).as_f64(), Some(42.0));
        assert_eq!(Value::Str("42".to_string()).as_f64(), Some(42.0));
        assert_eq!(Value::Bool(true).as_f64(), Some(1.0));
        assert_eq!(Value::Null.as_f64(), None);
    }

    #[test]
    fn test_as_bool() {
        assert_eq!(Value::Bool(true).as_bool(), Some(true));
        assert_eq!(Value::Num(1.0).as_bool(), Some(true));
        assert_eq!(Value::Num(0.0).as_bool(), Some(false));
        assert_eq!(Value::Str("true".to_string()).as_bool(), Some(true));
        assert_eq!(Value::Str("false".to_string()).as_bool(), Some(false));
        assert_eq!(Value::Null.as_bool(), None);
    }

    #[test]
    fn test_coerce_str() {
        assert_eq!(Value::Null.coerce_str(), "null");
        assert_eq!(Value::Bool(false).coerce_str(), "false");
        assert_eq!(Value::Num(42.0).coerce_str(), "42");
        assert_eq!(Value::Str("hello".to_string()).coerce_str(), "hello");
    }
}
