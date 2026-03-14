use serde_json::{json, Value};
use std::collections::HashMap;
use tera::Error;

use super::FilterResult;

pub fn round(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let n = value
        .as_f64()
        .ok_or_else(|| Error::msg("Expected number"))?;
    let precision = args.get("precision").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let multiplier = 10_f64.powi(precision as i32);
    let result = (n * multiplier).round() / multiplier;
    Ok(json!(result))
}

pub fn abs(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    let n = value
        .as_f64()
        .ok_or_else(|| Error::msg("Expected number"))?;
    Ok(json!(n.abs()))
}

pub fn format_number(value: &Value, args: &HashMap<String, Value>) -> FilterResult {
    let n = value
        .as_f64()
        .ok_or_else(|| Error::msg("Expected number"))?;
    let format = args.get("format").and_then(|v| v.as_str()).unwrap_or(",");

    let s = format!("{}", n);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let dec_part = parts.get(1);

    let formatted_int = int_part
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            if i > 0 && i % 3 == 0 {
                format!("{}{}", format, c)
            } else {
                c.to_string()
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

pub fn min_filter(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
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

pub fn max_filter(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
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

pub fn sum(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(arr) = value.as_array() {
        let total: f64 = arr.iter().filter_map(|v| v.as_f64()).sum();
        Ok(Value::Number(
            serde_json::Number::from_f64(total).unwrap_or(serde_json::Number::from(0)),
        ))
    } else {
        Ok(value.clone())
    }
}

pub fn avg(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
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

pub fn ceil(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(n) = value.as_f64() {
        Ok(Value::Number(
            serde_json::Number::from_f64(n.ceil()).unwrap_or(serde_json::Number::from(0)),
        ))
    } else {
        Ok(value.clone())
    }
}

pub fn floor(value: &Value, _: &HashMap<String, Value>) -> FilterResult {
    if let Some(n) = value.as_f64() {
        Ok(Value::Number(
            serde_json::Number::from_f64(n.floor()).unwrap_or(serde_json::Number::from(0)),
        ))
    } else {
        Ok(value.clone())
    }
}
