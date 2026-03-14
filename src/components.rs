use std::collections::HashMap;

use crate::error::Result;

#[derive(Debug, Clone)]
pub enum PropType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    Any,
}

impl PropType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "string" => PropType::String,
            "number" => PropType::Number,
            "boolean" => PropType::Boolean,
            "array" => PropType::Array,
            "object" => PropType::Object,
            _ => PropType::Any,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropDef {
    pub name: String,
    pub prop_type: PropType,
    pub required: bool,
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct Component {
    pub path: String,
    pub props: Vec<PropDef>,
    pub template: String,
    pub slots: Vec<String>,
}

#[derive(Clone)]
pub struct ComponentRegistry {
    components: HashMap<String, Component>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        ComponentRegistry {
            components: HashMap::new(),
        }
    }

    pub fn register(&mut self, path: &str, template: &str) -> Result<()> {
        let props = Self::parse_props(template);
        let slots = Self::parse_slots(template);

        self.components.insert(
            path.to_string(),
            Component {
                path: path.to_string(),
                props,
                template: template.to_string(),
                slots,
            },
        );

        Ok(())
    }

    pub fn get(&self, path: &str) -> Option<&Component> {
        self.components.get(path)
    }

    pub fn resolve_tag(&self, tag: &str) -> Option<String> {
        // Try direct component path
        if self.components.contains_key(tag) {
            return Some(tag.to_string());
        }

        // Try components/ prefix
        let with_prefix = format!("components/{}", tag);
        if self.components.contains_key(&with_prefix) {
            return Some(with_prefix);
        }

        // Try with file extension
        let with_ext = format!("{}.html", tag);
        if self.components.contains_key(&with_ext) {
            return Some(with_ext);
        }

        // Try nested: form.input -> components/form/input
        let parts: Vec<&str> = tag.split('.').collect();
        if parts.len() >= 2 {
            let nested = format!("components/{}.html", parts.join("/"));
            if self.components.contains_key(&nested) {
                return Some(nested);
            }
        }

        None
    }

    pub fn list_components(&self) -> Vec<String> {
        self.components.keys().cloned().collect()
    }

    fn parse_props(template: &str) -> Vec<PropDef> {
        let mut props = Vec::new();

        // Look for {%-- atom: @props({ ... }) --%}
        if let Some(start) = template.find("{%-- atom:") {
            if let Some(end) = template[start..].find("--%}") {
                let atom_block = &template[start + 11..start + end];
                if let Some(props_start) = atom_block.find("@props(") {
                    let props_end = atom_block[props_start..]
                        .find(')')
                        .map(|i| props_start + i + 1);
                    if let Some(end_idx) = props_end {
                        let props_str = &atom_block[props_start + 7..end_idx];
                        props = Self::parse_props_string(props_str);
                    }
                }
            }
        }

        props
    }

    fn parse_props_string(s: &str) -> Vec<PropDef> {
        let mut props = Vec::new();
        let s = s.trim();
        let s = s.trim_start_matches('{').trim_end_matches('}');

        for part in s.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            let parts: Vec<&str> = part.split(':').collect();
            if parts.is_empty() {
                continue;
            }

            let name = parts[0].trim().to_string();
            let prop_type = if parts.len() > 1 {
                PropType::from_str(parts[1].trim())
            } else {
                PropType::Any
            };

            let (required, default) = if let Some(eq_pos) = name.find('=') {
                let default_str = &name[eq_pos + 1..];
                let default = serde_json::Value::from(default_str.trim());
                (false, Some(default))
            } else {
                (true, None)
            };

            let name = name.split('=').next().unwrap_or(&name).trim().to_string();

            props.push(PropDef {
                name,
                prop_type,
                required,
                default,
            });
        }

        props
    }

    fn parse_slots(template: &str) -> Vec<String> {
        let mut slots = Vec::new();

        // Look for slot_default(), slot_header(), etc.
        let mut search = template;
        while let Some(start) = search.find("slot_") {
            search = &search[start + 5..];
            if let Some(end) = search.find("()") {
                let slot_name = &search[..end];
                if !slot_name.is_empty() && !slots.contains(&slot_name.to_string()) {
                    slots.push(slot_name.to_string());
                }
            }
        }

        slots
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
