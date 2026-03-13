use crate::parser::parser::Node;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SlotResolver {
    named: HashMap<String, Vec<Node>>,
    main: Vec<Node>,
}

impl SlotResolver {
    pub fn from_component_body(nodes: &[Node]) -> Self {
        let mut named = HashMap::new();
        let mut main = Vec::new();

        let mut i = 0;
        while i < nodes.len() {
            match &nodes[i] {
                Node::Directive { name, body, .. } if name == "fill" => {
                    // Extract slot name from @fill(name) - this would come from args
                    // For now, store under "default" if no name provided
                    if let Some(fill_body) = body {
                        // Store the body as fill content
                        // In real implementation, parse @fill(name) to get slot name
                        named.insert("default".to_string(), fill_body.clone());
                    }
                    i += 1;
                }
                _ => {
                    // Non-fill content goes to main slot
                    main.push(nodes[i].clone());
                    i += 1;
                }
            }
        }

        SlotResolver { named, main }
    }

    pub fn resolve(&self, name: Option<&str>) -> &[Node] {
        match name {
            Some(slot_name) => self
                .named
                .get(slot_name)
                .map(|v| v.as_slice())
                .unwrap_or(&[]),
            None => &self.main,
        }
    }

    pub fn has_slot(&self, name: &str) -> bool {
        self.named.contains_key(name)
    }

    pub fn has_main(&self) -> bool {
        !self.main.is_empty()
    }

    pub fn named_slots(&self) -> impl Iterator<Item = &String> {
        self.named.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_main_slot() {
        let resolver = SlotResolver {
            named: HashMap::new(),
            main: vec![Node::Text("main content".to_string())],
        };

        let result = resolver.resolve(None);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_resolve_named_slot() {
        let mut named = HashMap::new();
        named.insert(
            "header".to_string(),
            vec![Node::Text("header content".to_string())],
        );

        let resolver = SlotResolver {
            named,
            main: vec![],
        };

        let result = resolver.resolve(Some("header"));
        assert!(!result.is_empty());
    }

    #[test]
    fn test_resolve_missing_slot() {
        let resolver = SlotResolver {
            named: HashMap::new(),
            main: vec![],
        };

        let result = resolver.resolve(Some("missing"));
        assert!(result.is_empty());
    }
}
