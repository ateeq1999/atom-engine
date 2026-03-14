use serde_json::Value;
use std::collections::HashMap;

use super::FilterResult;

pub fn escape_html(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
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

pub fn safe(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    Ok(value.clone())
}
