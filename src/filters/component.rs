use serde_json::{json, Value};
use std::collections::HashMap;
use tera::Function;

use super::FilterResult;

pub fn slot_filter(name: &Value, _: &HashMap<String, Value>) -> FilterResult {
    let slot_name = name.as_str().unwrap_or("default").trim_start_matches('$');
    let slot_key = format!("__slot_{}", slot_name);
    if let Some(slot_content) = name.get(&slot_key) {
        return Ok(slot_content.clone());
    }
    Ok(Value::String(String::new()))
}

pub fn has_slot_filter(name: &Value, _: &HashMap<String, Value>) -> FilterResult {
    let slot_name = name.as_str().unwrap_or("default").trim_start_matches('$');
    let slot_key = format!("__slot_{}", slot_name);
    Ok(Value::Bool(name.get(&slot_key).is_some()))
}

pub fn stack_filter(name: &Value, _: &HashMap<String, Value>) -> FilterResult {
    let stack_name = name.as_str().unwrap_or("");
    let stack_key = format!("__stack_{}", stack_name);
    Ok(name
        .get(&stack_key)
        .cloned()
        .unwrap_or(Value::String(String::new())))
}

pub struct PushFn;

impl Function for PushFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let stack_name = args
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("default");
        let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");
        let stack_key = format!("__stack_{}", stack_name);
        Ok(Value::String(format!("{}__push__ {}", stack_key, content)))
    }
}

pub struct PrependFn;

impl Function for PrependFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
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

pub struct SetSlotFn;

impl Function for SetSlotFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
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

pub struct OnceFn;

impl Function for OnceFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let key = args
            .get("key")
            .and_then(|v| v.as_str())
            .unwrap_or("default");
        let hash = simple_hash(key);
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
