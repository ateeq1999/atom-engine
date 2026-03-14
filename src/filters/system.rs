use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use tera::{Error, Function};

use super::FilterResult;

pub struct DumpFn;

impl Function for DumpFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        for (key, value) in args {
            eprintln!("[dump] {} = {:?}", key, value);
        }
        Ok(Value::Null)
    }
}

pub struct LogFn;

impl Function for LogFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        for (key, value) in args {
            eprintln!("[log] {} = {:?}", key, value);
        }
        Ok(Value::Null)
    }
}

pub struct RangeFn;

impl Function for RangeFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let end = args
            .get("end")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| Error::msg("Missing 'end' argument"))?;
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
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
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

impl Default for CycleFn {
    fn default() -> Self {
        Self::new()
    }
}

impl Function for CycleFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
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
    fn call(&self, _: &HashMap<String, Value>) -> FilterResult {
        Ok(Value::String(uuid::Uuid::new_v4().to_string()))
    }
}

pub struct RandomFn;

impl Function for RandomFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
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
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        let array = args
            .get("array")
            .ok_or_else(|| Error::msg("Missing array"))?;
        let arr = array
            .as_array()
            .ok_or_else(|| Error::msg("Expected array"))?;
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
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or("");
        Ok(Value::Bool(std::path::Path::new(path).exists()))
    }
}

pub struct EnvFn;

impl Function for EnvFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let key = args.get("key").and_then(|v| v.as_str()).unwrap_or("");
        Ok(Value::String(std::env::var(key).unwrap_or_default()))
    }
}

pub struct Md5Fn;

impl Function for Md5Fn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        use md5::{Digest, Md5};
        let input = args.get("string").and_then(|v| v.as_str()).unwrap_or("");
        let result = Md5::digest(input);
        Ok(Value::String(hex::encode(result)))
    }
}

pub struct Sha256Fn;

impl Function for Sha256Fn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        use sha2::{Digest, Sha256};
        let input = args.get("string").and_then(|v| v.as_str()).unwrap_or("");
        let result = Sha256::digest(input);
        Ok(Value::String(hex::encode(result)))
    }
}
