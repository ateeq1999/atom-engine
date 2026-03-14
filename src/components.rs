use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use indexmap::IndexMap;
use serde_json::Value;

use crate::error::Result;

pub fn compute_props_hash(props: &Value) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let json_str = serde_json::to_string(props).unwrap_or_default();
    json_str.hash(&mut hasher);
    hasher.finish()
}

pub fn compute_cache_key(path: &str, props_hash: u64) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    path.hash(&mut hasher);
    props_hash.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    Any,
}

impl PropType {
    #[allow(clippy::should_implement_trait)]
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

    pub fn matches(&self, value: &Value) -> bool {
        match self {
            PropType::Any => true,
            PropType::String => value.is_string(),
            PropType::Number => value.is_number(),
            PropType::Boolean => value.is_boolean(),
            PropType::Array => value.is_array(),
            PropType::Object => value.is_object(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropDef {
    pub name: String,
    pub prop_type: PropType,
    pub required: bool,
    pub default: Option<Value>,
}

impl PropDef {
    pub fn validate(&self, value: &Value) -> std::result::Result<(), String> {
        if value.is_null() && self.required {
            return Err(format!("Required prop '{}' is null", self.name));
        }
        if !self.prop_type.matches(value) {
            return Err(format!(
                "Prop '{}' type mismatch: expected {:?}, got {}",
                self.name,
                self.prop_type,
                match value {
                    Value::Null => "null",
                    Value::Bool(_) => "boolean",
                    Value::Number(_) => "number",
                    Value::String(_) => "string",
                    Value::Array(_) => "array",
                    Value::Object(_) => "object",
                }
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Component {
    pub path: String,
    pub props: Vec<PropDef>,
    pub template: String,
    pub slots: Vec<String>,
    pub optional_slots: Vec<String>,
    pub scoped_slots: Vec<ScopedSlotDef>,
}

#[derive(Debug, Clone)]
pub struct ScopedSlotDef {
    pub name: String,
    pub props: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SlotData {
    pub fills: IndexMap<String, String>,
    pub default: Option<String>,
    pub scoped_data: HashMap<String, Value>,
}

#[derive(Clone, Default)]
pub struct ComponentRenderer {
    stack_buffers: HashMap<String, Vec<String>>,
    slot_data: HashMap<String, SlotData>,
    once_rendered: std::collections::HashSet<u64>,
}

impl ComponentRenderer {
    pub fn new() -> Self {
        ComponentRenderer {
            stack_buffers: HashMap::new(),
            slot_data: HashMap::new(),
            once_rendered: std::collections::HashSet::new(),
        }
    }

    pub fn push(&mut self, name: &str, content: String) {
        self.stack_buffers
            .entry(name.to_string())
            .or_default()
            .push(content);
    }

    pub fn prepend(&mut self, name: &str, content: String) {
        self.stack_buffers
            .entry(name.to_string())
            .or_default()
            .insert(0, content);
    }

    pub fn drain(&mut self, name: &str) -> String {
        self.stack_buffers
            .remove(name)
            .map(|v| v.join("\n"))
            .unwrap_or_default()
    }

    pub fn peek(&self, name: &str) -> Option<String> {
        self.stack_buffers.get(name).map(|v| v.join("\n"))
    }

    pub fn set_slot_fill(&mut self, slot_name: &str, content: String) {
        let slot_name = slot_name.trim_start_matches('$').to_string();
        let slot = self.slot_data.entry(slot_name.clone()).or_default();
        if slot_name == "default" || slot_name.is_empty() {
            slot.default = Some(content);
        } else {
            slot.fills.insert(slot_name, content);
        }
    }

    pub fn get_slot(&self, name: &str) -> Option<String> {
        let name = name.trim_start_matches('$').to_string();
        if name == "default" || name.is_empty() {
            self.slot_data
                .get("default")
                .and_then(|s| s.default.clone())
        } else {
            self.slot_data
                .get(&name)
                .and_then(|s| s.fills.get(&name).cloned())
        }
    }

    pub fn has_slot(&self, name: &str) -> bool {
        let name = name.trim_start_matches('$').to_string();
        if name == "default" || name.is_empty() {
            self.slot_data
                .get("default")
                .map(|s| s.default.is_some())
                .unwrap_or(false)
        } else {
            self.slot_data
                .get(&name)
                .map(|s| s.fills.contains_key(&name))
                .unwrap_or(false)
        }
    }

    pub fn set_scoped_data(&mut self, slot_name: &str, key: &str, value: Value) {
        let slot_name = slot_name.trim_start_matches('$').to_string();
        let slot = self.slot_data.entry(slot_name).or_default();
        slot.scoped_data.insert(key.to_string(), value);
    }

    pub fn get_scoped_data(&self, slot_name: &str) -> Option<HashMap<String, Value>> {
        let name = slot_name.trim_start_matches('$').to_string();
        self.slot_data.get(&name).map(|s| s.scoped_data.clone())
    }

    pub fn once(&mut self, key: u64) -> bool {
        if self.once_rendered.contains(&key) {
            return false;
        }
        self.once_rendered.insert(key);
        true
    }

    pub fn reset(&mut self) {
        self.stack_buffers.clear();
        self.slot_data.clear();
    }
}

#[derive(Clone, Default)]
pub struct ComponentRegistry {
    components: HashMap<String, Component>,
    cache: Arc<std::sync::RwLock<ComponentCache>>,
    cache_enabled: bool,
}

#[derive(Default)]
pub struct ComponentCache {
    entries: HashMap<u64, CachedRender>,
}

#[derive(Clone)]
pub struct CachedRender {
    pub html: String,
    #[allow(dead_code)]
    pub props_hash: u64,
}

impl ComponentCache {
    pub fn new() -> Self {
        ComponentCache {
            entries: HashMap::new(),
        }
    }

    pub fn get(&self, key: &u64) -> Option<String> {
        self.entries.get(key).map(|c| c.html.clone())
    }

    pub fn insert(&mut self, key: u64, html: String, props_hash: u64) {
        self.entries.insert(key, CachedRender { html, props_hash });
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl ComponentRegistry {
    pub fn new() -> Self {
        ComponentRegistry {
            components: HashMap::new(),
            cache: Arc::new(std::sync::RwLock::new(ComponentCache::new())),
            cache_enabled: false,
        }
    }

    pub fn register(&mut self, path: &str, template: &str) -> Result<()> {
        let props = Self::parse_props(template);
        let (slots, optional_slots) = Self::parse_slots(template);
        let scoped_slots = Self::parse_scoped_slots(template);

        self.components.insert(
            path.to_string(),
            Component {
                path: path.to_string(),
                props,
                template: template.to_string(),
                slots,
                optional_slots,
                scoped_slots,
            },
        );

        Ok(())
    }

    pub fn get(&self, path: &str) -> Option<&Component> {
        self.components.get(path)
    }

    pub fn resolve_tag(&self, tag: &str) -> Option<String> {
        if self.components.contains_key(tag) {
            return Some(tag.to_string());
        }

        let with_prefix = format!("components/{}", tag);
        if self.components.contains_key(&with_prefix) {
            return Some(with_prefix);
        }

        let with_ext = format!("{}.html", tag);
        if self.components.contains_key(&with_ext) {
            return Some(with_ext);
        }

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

    pub fn validate_props(
        &self,
        path: &str,
        provided: &Value,
    ) -> std::result::Result<HashMap<String, Value>, String> {
        let component = self
            .components
            .get(path)
            .ok_or_else(|| format!("Component '{}' not found", path))?;

        let mut result = HashMap::new();
        let provided_obj = provided.as_object().ok_or("Props must be an object")?;

        for prop_def in &component.props {
            if let Some(value) = provided_obj.get(&prop_def.name) {
                prop_def.validate(value)?;
                result.insert(prop_def.name.clone(), value.clone());
            } else if let Some(default) = &prop_def.default {
                result.insert(prop_def.name.clone(), default.clone());
            } else if prop_def.required {
                return Err(format!("Required prop '{}' not provided", prop_def.name));
            }
        }

        Ok(result)
    }

    pub fn enable_cache(&mut self, enabled: bool) {
        self.cache_enabled = enabled;
    }

    pub fn is_cache_enabled(&self) -> bool {
        self.cache_enabled
    }

    pub fn get_cached(&self, key: u64) -> Option<String> {
        if !self.cache_enabled {
            return None;
        }
        self.cache.read().ok()?.get(&key)
    }

    pub fn set_cached(&self, key: u64, html: String, props_hash: u64) {
        if !self.cache_enabled {
            return;
        }
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(key, html, props_hash);
        }
    }

    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }

    pub fn cache_len(&self) -> usize {
        self.cache.read().map(|c| c.len()).unwrap_or(0)
    }

    fn parse_scoped_slots(template: &str) -> Vec<ScopedSlotDef> {
        let mut scoped_slots = Vec::new();

        if let Some(start) = template.find("{%-- atom:") {
            if let Some(end) = template[start..].find("--%}") {
                let atom_block = &template[start + 11..start + end];
                if let Some(slots_start) = atom_block.find("@scoped_slots(") {
                    let slots_end = atom_block[slots_start..]
                        .find(')')
                        .map(|i| slots_start + i + 1);
                    if let Some(end_idx) = slots_end {
                        let slots_str = &atom_block[slots_start + 14..end_idx];
                        for part in slots_str.split(',') {
                            let part = part.trim();
                            if part.is_empty() {
                                continue;
                            }
                            let parts: Vec<&str> = part.split(':').collect();
                            let name = parts[0].trim().to_string();
                            let props = if parts.len() > 1 {
                                parts[1]
                                    .trim()
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .collect()
                            } else {
                                vec![]
                            };
                            scoped_slots.push(ScopedSlotDef { name, props });
                        }
                    }
                }
            }
        }

        scoped_slots
    }

    fn parse_props(template: &str) -> Vec<PropDef> {
        let mut props = Vec::new();

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

            let mut name = parts[0].trim().to_string();
            let prop_type = if parts.len() > 1 {
                PropType::from_str(parts[1].trim())
            } else {
                PropType::Any
            };

            let optional = name.ends_with('?');
            if optional {
                name = name.trim_end_matches('?').to_string();
            }

            let (required, default) = if let Some(eq_pos) = name.find('=') {
                let default_str = &name[eq_pos + 1..];
                let default = serde_json::Value::from(default_str.trim());
                (false, Some(default))
            } else {
                (true, None)
            };

            name = name.split('=').next().unwrap_or(&name).trim().to_string();

            let required = required && !optional;

            props.push(PropDef {
                name,
                prop_type,
                required,
                default,
            });
        }

        props
    }

    fn parse_slots(template: &str) -> (Vec<String>, Vec<String>) {
        let mut slots = Vec::new();
        let mut optional_slots = Vec::new();

        let mut search = template;
        while let Some(start) = search.find("slot_") {
            search = &search[start + 5..];
            if let Some(end) = search.find("()") {
                let raw_name = &search[..end];
                if raw_name.is_empty() {
                    continue;
                }
                let is_optional = raw_name.ends_with('?');
                let name = if is_optional {
                    raw_name.trim_end_matches('?').to_string()
                } else {
                    raw_name.to_string()
                };

                if is_optional {
                    if !optional_slots.contains(&name) {
                        optional_slots.push(name);
                    }
                } else if !slots.contains(&name) {
                    slots.push(name);
                }
            }
        }

        (slots, optional_slots)
    }
}
