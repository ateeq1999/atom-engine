use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ContextChain {
    layers: Vec<HashMap<String, Value>>,
}

impl ContextChain {
    pub fn new() -> Self {
        ContextChain {
            layers: vec![HashMap::new()],
        }
    }

    pub fn provide(&mut self, key: &str, value: Value) {
        if let Some(last) = self.layers.last_mut() {
            last.insert(key.to_string(), value);
        }
    }

    pub fn inject(&self, key: &str) -> Option<&Value> {
        // Search from innermost to outermost
        for layer in self.layers.iter().rev() {
            if let Some(value) = layer.get(key) {
                return Some(value);
            }
        }
        None
    }

    pub fn push_layer(&mut self) {
        self.layers.push(HashMap::new());
    }

    pub fn pop_layer(&mut self) {
        if self.layers.len() > 1 {
            self.layers.pop();
        }
    }

    pub fn all(&self) -> HashMap<String, Value> {
        let mut result = HashMap::new();
        for layer in &self.layers {
            for (k, v) in layer {
                result.insert(k.clone(), v.clone());
            }
        }
        result
    }
}

impl Default for ContextChain {
    fn default() -> Self {
        Self::new()
    }
}
