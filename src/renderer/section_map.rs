use crate::parser::parser::Node;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SectionMap {
    sections: HashMap<String, Vec<Node>>,
}

impl SectionMap {
    pub fn new() -> Self {
        SectionMap {
            sections: HashMap::new(),
        }
    }

    pub fn from_template(nodes: &[Node]) -> Self {
        let mut sections = HashMap::new();

        for node in nodes {
            if let Node::Section { name, body } = node {
                sections.insert(name.clone(), body.clone());
            }
        }

        SectionMap { sections }
    }

    pub fn get(&self, name: &str) -> Option<&[Node]> {
        self.sections.get(name).map(|v| v.as_slice())
    }

    pub fn has(&self, name: &str) -> bool {
        self.sections.contains_key(name)
    }

    pub fn insert(&mut self, name: String, body: Vec<Node>) {
        self.sections.insert(name, body);
    }

    pub fn len(&self) -> usize {
        self.sections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.sections.keys()
    }
}

impl Default for SectionMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_template() {
        let nodes = vec![
            Node::Section {
                name: "title".to_string(),
                body: vec![Node::Text("Hello".to_string())],
            },
            Node::Section {
                name: "content".to_string(),
                body: vec![Node::Text("World".to_string())],
            },
        ];

        let map = SectionMap::from_template(&nodes);

        assert!(map.has("title"));
        assert!(map.has("content"));
        assert!(!map.has("missing"));
    }

    #[test]
    fn test_get() {
        let nodes = vec![Node::Section {
            name: "title".to_string(),
            body: vec![Node::Text("Hello".to_string())],
        }];

        let map = SectionMap::from_template(&nodes);
        let section = map.get("title").unwrap();

        assert_eq!(section.len(), 1);
    }

    #[test]
    fn test_get_missing() {
        let map = SectionMap::new();
        assert!(map.get("missing").is_none());
    }
}
