use crate::types::value::Value;
use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct ContextChain {
    layers: Vec<IndexMap<String, Value>>,
}

impl ContextChain {
    pub fn new() -> Self {
        ContextChain {
            layers: vec![IndexMap::new()],
        }
    }

    pub fn push_layer(&mut self) {
        self.layers.push(IndexMap::new());
    }

    pub fn pop_layer(&mut self) {
        if self.layers.len() > 1 {
            self.layers.pop();
        }
    }

    pub fn provide(&mut self, key: &str, value: Value) {
        if let Some(layer) = self.layers.last_mut() {
            layer.insert(key.to_string(), value);
        }
    }

    pub fn inject(&self, key: &str) -> Option<&Value> {
        for layer in self.layers.iter().rev() {
            if let Some(value) = layer.get(key) {
                return Some(value);
            }
        }
        None
    }

    pub fn has_key(&self, key: &str) -> bool {
        for layer in self.layers.iter().rev() {
            if layer.contains_key(key) {
                return true;
            }
        }
        false
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn get_inner(&self) -> Option<&IndexMap<String, Value>> {
        self.layers.last()
    }
}

impl Default for ContextChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provide_and_inject() {
        let mut chain = ContextChain::new();
        chain.provide("theme", Value::Str("dark".to_string()));
        assert_eq!(chain.inject("theme"), Some(&Value::Str("dark".to_string())));
    }

    #[test]
    fn test_inject_not_found() {
        let chain = ContextChain::new();
        assert_eq!(chain.inject("missing"), None);
    }

    #[test]
    fn test_layer_propagation() {
        let mut chain = ContextChain::new();
        chain.provide("a", Value::Num(1.0));

        chain.push_layer();
        chain.provide("b", Value::Num(2.0));

        assert_eq!(chain.inject("a"), Some(&Value::Num(1.0)));
        assert_eq!(chain.inject("b"), Some(&Value::Num(2.0)));

        chain.pop_layer();

        assert_eq!(chain.inject("a"), Some(&Value::Num(1.0)));
        assert_eq!(chain.inject("b"), None);
    }
}
