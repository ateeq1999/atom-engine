use serde_json::Value;
use std::collections::HashMap;
use tera::Error;

use super::FilterResult;

pub fn json_decode(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    let s = value
        .as_str()
        .ok_or_else(|| Error::msg("Expected string"))?;
    serde_json::from_str(s).map_err(|e| Error::msg(e.to_string()))
}

pub fn urlescape(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        Ok(Value::String(urlencoding::encode(s).to_string()))
    } else {
        Ok(value.clone())
    }
}

pub fn urlunescape(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        Ok(Value::String(
            urlencoding::decode(s)
                .map_err(|e| Error::msg(e.to_string()))?
                .to_string(),
        ))
    } else {
        Ok(value.clone())
    }
}

pub fn strip_tags(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        let re = regex::Regex::new(r"<[^>]*>").unwrap();
        Ok(Value::String(re.replace_all(s, "").to_string()))
    } else {
        Ok(value.clone())
    }
}

pub fn base64_encode(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        Ok(Value::String(base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            s,
        )))
    } else {
        Ok(value.clone())
    }
}

pub fn base64_decode(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(s) = value.as_str() {
        let decoded = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, s)
            .map_err(|e| Error::msg(e.to_string()))?;
        Ok(Value::String(
            String::from_utf8(decoded).map_err(|e| Error::msg(e.to_string()))?,
        ))
    } else {
        Ok(value.clone())
    }
}
