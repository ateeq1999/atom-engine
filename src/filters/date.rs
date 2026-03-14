use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

use super::FilterResult;

pub fn date_format(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
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
