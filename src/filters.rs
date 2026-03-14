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

    let truncated = s.chars().take(length).collect::<String>();
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

// Slot helper - returns slot content from context
pub fn slot_filter(name: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let slot_name = name.as_str().unwrap_or("default").trim_start_matches('$');

    // Look for slot content in context
    let slot_key = format!("__slot_{}", slot_name);
    if let Some(slot_content) = name.get(&slot_key) {
        return Ok(slot_content.clone());
    }

    Ok(Value::String(String::new()))
}

pub fn has_slot_filter(name: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let slot_name = name.as_str().unwrap_or("default").trim_start_matches('$');

    let slot_key = format!("__slot_{}", slot_name);
    Ok(Value::Bool(name.get(&slot_key).is_some()))
}

// Stack helper - retrieves accumulated stack content
pub fn stack_filter(name: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let stack_name = name.as_str().unwrap_or("");
    let stack_key = format!("__stack_{}", stack_name);

    Ok(name
        .get(&stack_key)
        .cloned()
        .unwrap_or(Value::String(String::new())))
}

// Push function - adds content to a stack
pub struct PushFn;

impl Function for PushFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let stack_name = args
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("default");
        let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");

        let stack_key = format!("__stack_{}", stack_name);
        Ok(Value::String(format!("{}__push__ {}", stack_key, content)))
    }
}

// Prepend function - adds content to beginning of stack
pub struct PrependFn;

impl Function for PrependFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let stack_name = args
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("default");
        let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");

        let stack_key = format!("__stack_{}", stack_name);
        Ok(Value::String(format!(
            "{}__prepend__ {}",
            stack_key, content
        )))
    }
}

// Set slot function - defines slot content
pub struct SetSlotFn;

impl Function for SetSlotFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let slot_name = args
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("default")
            .trim_start_matches('$');
        let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");

        let slot_key = format!("__slot_{}", slot_name);
        Ok(Value::String(format!("{} {}", slot_key, content)))
    }
}

// Once function - renders content only once
pub struct OnceFn;

impl Function for OnceFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let key = args
            .get("key")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        // Simple hash for deduplication
        let hash = simple_hash(key);

        // In a real implementation, we'd check against a stored set
        // For now, always return the content to render
        let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");

        Ok(json!({ "key": hash, "content": content }))
    }
}

fn simple_hash(s: &str) -> u64 {
    let mut hash: u64 = 0;
    for (i, c) in s.bytes().enumerate() {
        hash = hash.wrapping_add((c as u64).wrapping_mul(31_u64.wrapping_pow(i as u32)));
    }
    hash
}

// Conditional filter - returns value when condition is truthy, otherwise returns default
pub fn when(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let condition = value.as_bool().unwrap_or(false);
    let then_val = args.get("then").cloned().unwrap_or(Value::Null);
    let else_val = args.get("else").cloned().unwrap_or(Value::Null);

    Ok(if condition { then_val } else { else_val })
}

// Default filter - returns value if truthy, otherwise returns default
pub fn default_filter(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
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

// Coalesce filter - returns first non-null value
pub fn coalesce(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
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

// Defined test - returns true if value is not null
pub fn defined(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    Ok(Value::Bool(!value.is_null()))
}

// Undefined test - returns true if value is null
pub fn undefined(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    Ok(Value::Bool(value.is_null()))
}

// Empty test - returns true if value is null, empty string, empty array, or empty object
pub fn empty(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let is_empty = match value {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
        _ => false,
    };
    Ok(Value::Bool(is_empty))
}

// Not empty test
pub fn not_empty(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let is_empty = match value {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(arr) => arr.is_empty(),
        Value::Object(obj) => obj.is_empty(),
        _ => false,
    };
    Ok(Value::Bool(!is_empty))
}

// ===== Phase 9: Additional String Filters =====

pub fn replace(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("Expected string"))?;
    let old = args.get("old").and_then(|v| v.as_str()).unwrap_or("");
    let new = args.get("new").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::String(s.replace(old, new)))
}

pub fn remove(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("Expected string"))?;
    let to_remove = args.get("string").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::String(s.replace(to_remove, "")))
}

pub fn prepend(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("Expected string"))?;
    let prefix = args.get("string").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::String(format!("{}{}", prefix, s)))
}

pub fn append(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("Expected string"))?;
    let suffix = args.get("string").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::String(format!("{}{}", s, suffix)))
}

pub fn strip(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        Ok(Value::String(s.trim().to_string()))
    } else {
        Ok(value.clone())
    }
}

pub fn nl2br(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
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

pub fn word_count(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        let count = s.split_whitespace().count();
        Ok(Value::Number(count.into()))
    } else {
        Ok(Value::Number(0.into()))
    }
}

pub fn char_count(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
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

pub fn starts_with(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("Expected string"))?;
    let prefix = args.get("prefix").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::Bool(s.starts_with(prefix)))
}

pub fn ends_with(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("Expected string"))?;
    let suffix = args.get("suffix").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::Bool(s.ends_with(suffix)))
}

pub fn contains(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("Expected string"))?;
    let substring = args.get("substring").and_then(|v| v.as_str()).unwrap_or("");
    Ok(Value::Bool(s.contains(substring)))
}

// ===== Collection Filters =====

pub fn join(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
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

pub fn slice(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
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

pub fn uniq(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
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

pub fn shuffle(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
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

// ===== Encoding Filters =====

pub fn json_decode(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let s = value
        .as_str()
        .ok_or_else(|| tera::Error::msg("Expected string"))?;
    serde_json::from_str(s).map_err(|e| tera::Error::msg(e.to_string()))
}

pub fn urlescape(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        Ok(Value::String(urlencoding::encode(s).to_string()))
    } else {
        Ok(value.clone())
    }
}

pub fn urlunescape(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        Ok(Value::String(
            urlencoding::decode(s)
                .map_err(|e| tera::Error::msg(e.to_string()))?
                .to_string(),
        ))
    } else {
        Ok(value.clone())
    }
}

pub fn strip_tags(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        let re = regex::Regex::new(r"<[^>]*>").unwrap();
        Ok(Value::String(re.replace_all(s, "").to_string()))
    } else {
        Ok(value.clone())
    }
}

pub fn base64_encode(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        Ok(Value::String(base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            s,
        )))
    } else {
        Ok(value.clone())
    }
}

pub fn base64_decode(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(s) = value.as_str() {
        let decoded = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, s)
            .map_err(|e| tera::Error::msg(e.to_string()))?;
        Ok(Value::String(
            String::from_utf8(decoded).map_err(|e| tera::Error::msg(e.to_string()))?,
        ))
    } else {
        Ok(value.clone())
    }
}

// ===== Math Filters =====

pub fn min_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(arr) = value.as_array() {
        let min = arr
            .iter()
            .filter_map(|v| v.as_f64())
            .fold(f64::INFINITY, f64::min);
        Ok(Value::Number(
            serde_json::Number::from_f64(min).unwrap_or(serde_json::Number::from(0)),
        ))
    } else {
        Ok(value.clone())
    }
}

pub fn max_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(arr) = value.as_array() {
        let max = arr
            .iter()
            .filter_map(|v| v.as_f64())
            .fold(f64::NEG_INFINITY, f64::max);
        Ok(Value::Number(
            serde_json::Number::from_f64(max).unwrap_or(serde_json::Number::from(0)),
        ))
    } else {
        Ok(value.clone())
    }
}

pub fn sum(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(arr) = value.as_array() {
        let total: f64 = arr.iter().filter_map(|v| v.as_f64()).sum();
        Ok(Value::Number(
            serde_json::Number::from_f64(total).unwrap_or(serde_json::Number::from(0)),
        ))
    } else {
        Ok(value.clone())
    }
}

pub fn avg(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(arr) = value.as_array() {
        let nums: Vec<f64> = arr.iter().filter_map(|v| v.as_f64()).collect();
        let avg = if nums.is_empty() {
            0.0
        } else {
            nums.iter().sum::<f64>() / nums.len() as f64
        };
        Ok(Value::Number(
            serde_json::Number::from_f64(avg).unwrap_or(serde_json::Number::from(0)),
        ))
    } else {
        Ok(value.clone())
    }
}

pub fn ceil(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(n) = value.as_f64() {
        Ok(Value::Number(
            serde_json::Number::from_f64(n.ceil()).unwrap_or(serde_json::Number::from(0)),
        ))
    } else {
        Ok(value.clone())
    }
}

pub fn floor(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    if let Some(n) = value.as_f64() {
        Ok(Value::Number(
            serde_json::Number::from_f64(n.floor()).unwrap_or(serde_json::Number::from(0)),
        ))
    } else {
        Ok(value.clone())
    }
}

// ===== Phase 10: Additional Global Functions =====

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct CycleFn {
    index: AtomicUsize,
}

impl CycleFn {
    pub fn new() -> Self {
        CycleFn {
            index: AtomicUsize::new(0),
        }
    }
}

impl Function for CycleFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let values: Vec<Value> = (0..)
            .map(|i| args.get(&format!("{}", i)))
            .take_while(|v| v.is_some())
            .map(|v| v.unwrap().clone())
            .collect();

        if values.is_empty() {
            return Ok(Value::Null);
        }

        let idx = self.index.fetch_add(1, Ordering::SeqCst) % values.len();
        Ok(values[idx].clone())
    }
}

pub struct UuidFn;

impl Function for UuidFn {
    fn call(&self, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let uuid = uuid::Uuid::new_v4().to_string();
        Ok(Value::String(uuid))
    }
}

pub struct RandomFn;

impl Function for RandomFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let min = args.get("min").and_then(|v| v.as_i64()).unwrap_or(0);
        let max = args.get("max").and_then(|v| v.as_i64()).unwrap_or(100);

        let result = rng.gen_range(min..=max);
        Ok(Value::Number(result.into()))
    }
}

pub struct ChoiceFn;

impl Function for ChoiceFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let array = args
            .get("array")
            .ok_or_else(|| tera::Error::msg("Missing array"))?;
        let arr = array
            .as_array()
            .ok_or_else(|| tera::Error::msg("Expected array"))?;

        if arr.is_empty() {
            return Ok(Value::Null);
        }

        let mut rng = thread_rng();
        let choice = arr.choose(&mut rng).cloned();
        Ok(choice.unwrap_or(Value::Null))
    }
}

pub struct FileExistsFn;

impl Function for FileExistsFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
        let exists = std::path::Path::new(path).exists();
        Ok(Value::Bool(exists))
    }
}

pub struct EnvFn;

impl Function for EnvFn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let key = args.get("key").and_then(|v| v.as_str()).unwrap_or("");
        let value = std::env::var(key).unwrap_or_default();
        Ok(Value::String(value))
    }
}

pub struct Md5Fn;

impl Function for Md5Fn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        use md5::{Digest, Md5};

        let input = args.get("string").and_then(|v| v.as_str()).unwrap_or("");
        let result = Md5::digest(input);
        Ok(Value::String(hex::encode(result)))
    }
}

pub struct Sha256Fn;

impl Function for Sha256Fn {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        use sha2::{Digest, Sha256};

        let input = args.get("string").and_then(|v| v.as_str()).unwrap_or("");
        let result = Sha256::digest(input);
        Ok(Value::String(hex::encode(result)))
    }
}
