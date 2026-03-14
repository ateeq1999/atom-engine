use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

use super::FilterResult;

pub trait Filter: Send + Sync {
    fn filter(&self, value: &Value, args: &HashMap<String, Value>) -> FilterResult;
}

pub fn json_encode(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    Ok(Value::String(
        serde_json::to_string(value).unwrap_or_default(),
    ))
}

pub fn upper(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        Ok(Value::String(s.to_uppercase()))
    } else {
        Ok(value.clone())
    }
}

pub fn lower(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        Ok(Value::String(s.to_lowercase()))
    } else {
        Ok(value.clone())
    }
}

pub fn capitalize(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
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

pub fn title(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
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

pub fn camel_case(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        Ok(Value::String(heck::ToLowerCamelCase::to_lower_camel_case(
            s,
        )))
    } else {
        Ok(value.clone())
    }
}

pub fn pascal_case(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        Ok(Value::String(heck::ToLowerCamelCase::to_lower_camel_case(
            s,
        )))
    } else {
        Ok(value.clone())
    }
}

pub fn snake_case(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        Ok(Value::String(heck::ToSnakeCase::to_snake_case(s)))
    } else {
        Ok(value.clone())
    }
}

pub fn kebab_case(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        Ok(Value::String(heck::ToKebabCase::to_kebab_case(s)))
    } else {
        Ok(value.clone())
    }
}

pub fn truncate(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let s = value
        .as_str()
        .ok_or_else(|| Error::msg("Expected string"))?;
    let length = args.get("length").and_then(|v| v.as_u64()).unwrap_or(255) as usize;
    let suffix = args.get("end").and_then(|v| v.as_str()).unwrap_or("...");

    if s.len() <= length {
        return Ok(Value::String(s.to_string()));
    }

    let truncated = s.chars().take(length).collect::<String>();
    Ok(Value::String(format!("{}{}", truncated, suffix)))
}

pub fn slugify(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
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

pub fn pluralize(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let n = value.as_i64().unwrap_or(0);
    let singular = args.get("singular").and_then(|v| v.as_str()).unwrap_or("");
    let plural = args.get("plural").and_then(|v| v.as_str()).unwrap_or("s");

    Ok(Value::String(if n.abs() == 1 {
        singular.to_string()
    } else {
        plural.to_string()
    }))
}

pub fn replace(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let s = value
        .as_str()
        .ok_or_else(|| Error::msg("Expected string"))?;
    let old = args.get("old").and_then(|v| v.as_str()).unwrap_or("");
    let new = args.get("new").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::String(s.replace(old, new)))
}

pub fn remove(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let s = value
        .as_str()
        .ok_or_else(|| Error::msg("Expected string"))?;
    let to_remove = args.get("string").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::String(s.replace(to_remove, "")))
}

pub fn prepend(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let s = value
        .as_str()
        .ok_or_else(|| Error::msg("Expected string"))?;
    let prefix = args.get("string").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::String(format!("{}{}", prefix, s)))
}

pub fn append(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let s = value
        .as_str()
        .ok_or_else(|| Error::msg("Expected string"))?;
    let suffix = args.get("string").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::String(format!("{}{}", s, suffix)))
}

pub fn strip(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        Ok(Value::String(s.trim().to_string()))
    } else {
        Ok(value.clone())
    }
}

pub fn nl2br(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        let escaped = s
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('\n', "<br>\n");
        Ok(Value::String(escaped))
    } else {
        Ok(value.clone())
    }
}

pub fn word_count(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        let count = s.split_whitespace().count();
        Ok(Value::Number(count.into()))
    } else {
        Ok(Value::Number(0.into()))
    }
}

pub fn char_count(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        let include_spaces = args
            .get("include_spaces")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let count = if include_spaces {
            s.chars().count()
        } else {
            s.replace(' ', "").chars().count()
        };
        Ok(Value::Number(count.into()))
    } else {
        Ok(Value::Number(0.into()))
    }
}

pub fn starts_with(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let s = value
        .as_str()
        .ok_or_else(|| Error::msg("Expected string"))?;
    let prefix = args.get("prefix").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::Bool(s.starts_with(prefix)))
}

pub fn ends_with(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let s = value
        .as_str()
        .ok_or_else(|| Error::msg("Expected string"))?;
    let suffix = args.get("suffix").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::Bool(s.ends_with(suffix)))
}

pub fn contains(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let s = value
        .as_str()
        .ok_or_else(|| Error::msg("Expected string"))?;
    let substring = args.get("substring").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::Bool(s.contains(substring)))
}
