use std::collections::HashMap;

use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase};
use serde_json::{json, Value};
use tera::Function;

pub fn json_encode(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    Ok(Value::String(
        serde_json::to_string(value).unwrap_or_default(),
    ))
}

pub fn upper(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        Ok(Value::String(s.to_uppercase()))
    } else {
        Ok(value.clone())
    }
}

pub fn lower(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        Ok(Value::String(s.to_lowercase()))
    } else {
        Ok(value.clone())
    }
}

pub fn capitalize(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        let mut chars = s.chars();
        match chars.next() {
            None => Ok(Value::String(String::new())),
            Some(f) => Ok(Value::String(f.to_uppercase().chain(chars).collect())),
        }
    } else {
        Ok(value.clone())
    }
}

pub fn title(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        Ok(Value::String(
            s.split_whitespace()
                .map(|word| {
                    let mut chars = word.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().chain(chars).collect(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" "),
        ))
    } else {
        Ok(value.clone())
    }
}

pub fn camel_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        Ok(Value::String(s.to_lower_camel_case()))
    } else {
        Ok(value.clone())
    }
}

pub fn pascal_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        Ok(Value::String(s.to_lower_camel_case()))
    } else {
        Ok(value.clone())
    }
}

pub fn snake_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        Ok(Value::String(s.to_snake_case()))
    } else {
        Ok(value.clone())
    }
}

pub fn kebab_case(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        Ok(Value::String(s.to_kebab_case()))
    } else {
        Ok(value.clone())
    }
}

pub fn truncate(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("Expected string"))?;
    let length = args.get("length").and_then(|v| v.as_u64()).unwrap_or(255) as usize;
    let suffix = args.get("end").and_then(|v| v.as_str()).unwrap_or("...");

    if s.len() <= length {
        return Ok(Value::String(s.to_string()));
    }

    let truncated = s
        .chars()
        .take(length.saturating_sub(suffix.len()))
        .collect::<String>();
    Ok(Value::String(format!("{}{}", truncated, suffix)))
}

pub fn slugify(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        let slug = s
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-");
        Ok(Value::String(slug))
    } else {
        Ok(value.clone())
    }
}

pub fn pluralize(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let n = value.as_i64().unwrap_or(0);
    let singular = args.get("singular").and_then(|v| v.as_str()).unwrap_or("");
    let plural = args.get("plural").and_then(|v| v.as_str()).unwrap_or("s");

    Ok(Value::String(if n.abs() == 1 {
        singular.to_string()
    } else {
        plural.to_string()
    }))
}

pub fn first(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(arr) = value.as_array() {
        Ok(arr.first().cloned().unwrap_or(Value::Null))
    } else {
        Ok(value.clone())
    }
}

pub fn last(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(arr) = value.as_array() {
        Ok(arr.last().cloned().unwrap_or(Value::Null))
    } else {
        Ok(value.clone())
    }
}

pub fn length(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let len = match value {
        Value::Array(arr) => arr.len(),
        Value::Object(obj) => obj.len(),
        Value::String(s) => s.chars().count(),
        _ => 0,
    };
    Ok(Value::Number(len.into()))
}

pub fn reverse(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
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

pub fn sort(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(mut arr) = value.as_array().cloned() {
        // Simple sort for JSON values - convert to strings for comparison
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

pub fn group_by(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let arr = value
        .as_array()
        .ok_or_else(|| tera::Error::msg("Expected array"))?;
    let key = args
        .get("attribute")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("Missing attribute"))?;

    let mut groups: HashMap<String, Vec<Value>> = HashMap::new();
    for item in arr {
        let group_key = item
            .get(key)
            .map(|v| serde_json::to_string(v).unwrap_or_default())
            .unwrap_or_default();
        groups.entry(group_key).or_default().push(item.clone());
    }

    Ok(Value::Object(
        groups.into_iter().map(|(k, v)| (k, json!(v))).collect(),
    ))
}

pub fn where_filter(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let arr = value
        .as_array()
        .ok_or_else(|| tera::Error::msg("Expected array"))?;

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

pub fn pluck(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let arr = value
        .as_array()
        .ok_or_else(|| tera::Error::msg("Expected array"))?;
    let key = args
        .get("attribute")
        .and_then(|v| v.as_str())
        .ok_or_else(|| tera::Error::msg("Missing attribute"))?;

    let result: Vec<Value> = arr
        .iter()
        .filter_map(|item| item.get(key).cloned())
        .collect();

    Ok(Value::Array(result))
}

pub fn round(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let n = value
        .as_f64()
        .ok_or_else(|| tera::Error::msg("Expected number"))?;
    let precision = args.get("precision").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

    let multiplier = 10_f64.powi(precision as i32);
    let result = (n * multiplier).round() / multiplier;

    Ok(json!(result))
}

pub fn abs(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let n = value
        .as_f64()
        .ok_or_else(|| tera::Error::msg("Expected number"))?;
    Ok(json!(n.abs()))
}

pub fn format_number(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let n = value
        .as_f64()
        .ok_or_else(|| tera::Error::msg("Expected number"))?;
    let format = args.get("format").and_then(|v| v.as_str()).unwrap_or(",");

    let s = format!("{}", n);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let dec_part = parts.get(1);

    let formatted_int = int_part
        .chars()
        .rev()
        .enumerate()
        .filter_map(|(i, c)| {
            if i > 0 && i % 3 == 0 {
                Some(format!("{}{}", format, c))
            } else {
                Some(c.to_string())
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<String>();

    let result = match dec_part {
        Some(d) => format!("{}.{}", formatted_int, d),
        None => formatted_int,
    };

    Ok(Value::String(result))
}

pub fn date_format(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    use chrono::{DateTime, Utc};

    let fmt = args
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("%Y-%m-%d");

    let result = if let Some(s) = value.as_str() {
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            dt.format(fmt).to_string()
        } else {
            s.to_string()
        }
    } else if let Some(n) = value.as_i64() {
        if let Some(dt) = DateTime::<Utc>::from_timestamp(n, 0) {
            dt.format(fmt).to_string()
        } else {
            n.to_string()
        }
    } else {
        return Ok(value.clone());
    };

    Ok(Value::String(result))
}

pub fn escape_html(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        let escaped = s
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
            .replace('/', "&#x2F;");
        Ok(Value::String(escaped))
    } else {
        Ok(value.clone())
    }
}

pub fn safe(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    Ok(value.clone())
}

// Global functions

pub struct DumpFn;

impl Function for DumpFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        for (key, value) in args {
            eprintln!("[dump] {} = {:?}", key, value);
        }
        Ok(Value::Null)
    }
}

pub struct LogFn;

impl Function for LogFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        for (key, value) in args {
            eprintln!("[log] {} = {:?}", key, value);
        }
        Ok(Value::Null)
    }
}

pub struct RangeFn;

impl Function for RangeFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let end = args
            .get("end")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| tera::Error::msg("Missing 'end' argument"))?;
        let start = args.get("start").and_then(|v| v.as_i64()).unwrap_or(0);
        let step = args.get("step_by").and_then(|v| v.as_i64()).unwrap_or(1);

        let result: Vec<Value> = (start..end)
            .step_by(step as usize)
            .map(|i| json!(i))
            .collect();

        Ok(Value::Array(result))
    }
}

pub struct NowFn;

impl Function for NowFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        use chrono::Utc;

        let _utc = args.get("utc").and_then(|v| v.as_bool()).unwrap_or(false);
        let timestamp = args
            .get("timestamp")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if timestamp {
            let now = Utc::now();
            Ok(json!(now.timestamp()))
        } else {
            let now = Utc::now();
            Ok(Value::String(now.to_rfc3339()))
        }
    }
}
