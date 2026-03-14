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

pub struct RepeatFn;

impl Function for RepeatFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let count = args.get("count").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
        let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");
        let separator = args.get("separator").and_then(|v| v.as_str()).unwrap_or("");

        let result: Vec<String> = (0..count).map(|_| content.to_string()).collect();
        Ok(Value::String(result.join(separator)))
    }
}

pub struct TimesFn;

impl Function for TimesFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let times = args.get("times").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
        let start = args.get("start").and_then(|v| v.as_i64()).unwrap_or(1);
        let step = args.get("step").and_then(|v| v.as_i64()).unwrap_or(1);

        let result: Vec<Value> = (0..times)
            .map(|i| json!(start + (i as i64) * step))
            .collect();
        Ok(Value::Array(result))
    }
}

pub struct LoopFn;

impl Function for LoopFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let from = args.get("from").and_then(|v| v.as_i64()).unwrap_or(0);
        let to = args.get("to").and_then(|v| v.as_i64()).unwrap_or(10);
        let step = args.get("step").and_then(|v| v.as_i64()).unwrap_or(1);
        let include = args
            .get("inclusive")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let end = if include { to + 1 } else { to };

        let result: Vec<Value> = (from..end)
            .step_by(step as usize)
            .map(|i| json!({"index": i, "value": i}))
            .collect();
        Ok(Value::Array(result))
    }
}

pub struct IterateFn;

impl Function for IterateFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let array = args
            .get("array")
            .ok_or_else(|| Error::msg("Missing array"))?;
        let arr = array
            .as_array()
            .ok_or_else(|| Error::msg("Expected array"))?;
        let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
        let skip = args.get("skip").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

        let iter: Vec<Value> = arr
            .iter()
            .skip(skip)
            .take(if limit > 0 { limit } else { arr.len() })
            .enumerate()
            .map(|(i, v)| serde_json::json!({"index": i, "value": v, "key": i}))
            .collect();
        Ok(Value::Array(iter))
    }
}

pub struct ObjectFn;

impl Function for ObjectFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let keys = args
            .get("keys")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let values = args
            .get("values")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut obj = serde_json::Map::new();
        for (i, key) in keys.iter().enumerate() {
            if let Some(k) = key.as_str() {
                let v = values.get(i).cloned().unwrap_or(Value::Null);
                obj.insert(k.to_string(), v);
            }
        }
        Ok(Value::Object(obj))
    }
}

pub struct MergeFn;

impl Function for MergeFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let arr1 = args
            .get("array1")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let arr2 = args
            .get("array2")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut result = arr1;
        result.extend(arr2);
        Ok(Value::Array(result))
    }
}

pub struct ChunkFn;

impl Function for ChunkFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let array = args
            .get("array")
            .ok_or_else(|| Error::msg("Missing array"))?;
        let arr = array
            .as_array()
            .ok_or_else(|| Error::msg("Expected array"))?;
        let size = args.get("size").and_then(|v| v.as_u64()).unwrap_or(2) as usize;

        let chunks: Vec<Value> = arr
            .chunks(size)
            .map(|chunk| Value::Array(chunk.to_vec()))
            .collect();
        Ok(Value::Array(chunks))
    }
}

pub struct ZipFn;

impl Function for ZipFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let arrays = args
            .get("arrays")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        if arrays.is_empty() {
            return Ok(Value::Array(Vec::new()));
        }

        let max_len = arrays
            .iter()
            .map(|a| a.as_array().map(|a| a.len()).unwrap_or(0))
            .max()
            .unwrap_or(0);

        let result: Vec<Value> = (0..max_len)
            .map(|i| {
                let items: Vec<Value> = arrays
                    .iter()
                    .filter_map(|a| a.as_array().and_then(|arr| arr.get(i).cloned()))
                    .collect();
                Value::Array(items)
            })
            .collect();
        Ok(Value::Array(result))
    }
}

pub struct CompactFn;

impl Function for CompactFn {
    fn call(&self, args: &HashMap<String, Value>) -> FilterResult {
        let array = args
            .get("array")
            .ok_or_else(|| Error::msg("Missing array"))?;
        let arr = array
            .as_array()
            .ok_or_else(|| Error::msg("Expected array"))?;

        let result: Vec<Value> = arr
            .iter()
            .filter(|v| !v.is_null() && !v.as_array().map(|a| a.is_empty()).unwrap_or(false))
            .cloned()
            .collect();
        Ok(Value::Array(result))
    }
}
