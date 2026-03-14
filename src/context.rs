//! Context chain for provide/inject pattern.
//!
//! This module implements a hierarchical context system similar to Vue.js
//! where values can be provided at any level and injected by descendants.

use serde_json::Value;
use std::collections::HashMap;

/// A chain of context layers supporting provide/inject pattern.
///
/// Context values are searched from innermost (most recent) to outermost.
#[derive(Clone)]
pub struct ContextChain {
    layers: Vec<HashMap<String, Value>>,
}

impl ContextChain {
    /// Creates a new ContextChain with a single empty layer.
    pub fn new() -> Self {
        ContextChain {
            layers: vec![HashMap::new()],
        }
    }

    /// Provides a value to the current (innermost) context layer.
    pub fn provide(&mut self, key: &str, value: Value) {
        if let Some(last) = self.layers.last_mut() {
            last.insert(key.to_string(), value);
        }
    }

    /// Injects a value from the context chain.
    ///
    /// Searches from innermost to outermost layer and returns the first match.
    pub fn inject(&self, key: &str) -> Option<&Value> {
        // Search from innermost to outermost
        for layer in self.layers.iter().rev() {
            if let Some(value) = layer.get(key) {
                return Some(value);
            }
        }
        None
    }

    /// Pushes a new context layer onto the chain.
    pub fn push_layer(&mut self) {
        self.layers.push(HashMap::new());
    }

    /// Pops the outermost context layer from the chain.
    ///
    /// Ensures at least one layer remains.
    pub fn pop_layer(&mut self) {
        if self.layers.len() > 1 {
            self.layers.pop();
        }
    }

    /// Returns all context values from all layers merged into a single map.
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
