use crate::error::RenderError;
use crate::types::value::Value;
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
pub enum PropType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    Function,
    Any,
}

impl PropType {
    pub fn matches(&self, value: &Value) -> bool {
        match self {
            PropType::Any => true,
            PropType::String => matches!(value, Value::Str(_)),
            PropType::Number => matches!(value, Value::Num(_)),
            PropType::Boolean => matches!(value, Value::Bool(_)),
            PropType::Array => matches!(value, Value::Array(_)),
            PropType::Object => matches!(value, Value::Object(_)),
            PropType::Function => matches!(value, Value::Null), // Functions not implemented yet
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            PropType::String => "string",
            PropType::Number => "number",
            PropType::Boolean => "boolean",
            PropType::Array => "array",
            PropType::Object => "object",
            PropType::Function => "function",
            PropType::Any => "any",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropDecl {
    pub name: String,
    pub ty: PropType,
    pub default: Option<Value>,
    pub optional: bool,
}

impl PropDecl {
    pub fn new(name: &str, ty: PropType) -> Self {
        PropDecl {
            name: name.to_string(),
            ty,
            default: None,
            optional: false,
        }
    }

    pub fn with_default(mut self, default: Value) -> Self {
        self.default = Some(default);
        self.optional = true;
        self
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Props {
    pub declared: IndexMap<String, PropDecl>,
    pub provided: IndexMap<String, Value>,
    pub extra: IndexMap<String, Value>,
}

impl Props {
    pub fn new() -> Self {
        Props {
            declared: IndexMap::new(),
            provided: IndexMap::new(),
            extra: IndexMap::new(),
        }
    }

    pub fn validate(&self) -> Result<(), RenderError> {
        for (name, decl) in &self.declared {
            if !decl.optional && !self.provided.contains_key(name) && decl.default.is_none() {
                return Err(RenderError::MissingRequiredProp {
                    name: name.clone(),
                    template: String::new(),
                });
            }

            if let Some(value) = self.provided.get(name) {
                if !decl.ty.matches(value) {
                    return Err(RenderError::PropTypeMismatch {
                        name: name.clone(),
                        expected: decl.ty.name().to_string(),
                        found: value.type_name().to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.provided.get(name).or_else(|| {
            self.declared
                .get(name)
                .and_then(|decl| decl.default.as_ref())
        })
    }

    pub fn has(&self, name: &str) -> bool {
        self.provided.contains_key(name)
    }

    pub fn only(&self, keys: &[&str]) -> Props {
        let mut result = Props::new();
        for key in keys {
            if let Some(value) = self.provided.get(*key) {
                result
                    .provided
                    .insert(key.to_string(), value.clone() as Value);
            }
        }
        result
    }

    pub fn except(&self, keys: &[&str]) -> Props {
        let mut result = Props::new();
        let keys_set: std::collections::HashSet<&str> = keys.iter().cloned().collect();
        for (key, value) in &self.provided {
            if !keys_set.contains(key.as_str()) {
                result.provided.insert(key.clone(), value.clone() as Value);
            }
        }
        result
    }

    pub fn merge(&self, other: &Props) -> Props {
        let mut result = self.clone();
        for (key, value) in &other.provided {
            result.provided.insert(key.clone(), value.clone() as Value);
        }
        for (key, value) in &other.extra {
            result.extra.insert(key.clone(), value.clone() as Value);
        }
        result
    }

    pub fn to_attrs(&self) -> String {
        let mut attrs = Vec::new();
        for (key, value) in &self.provided {
            let attr_value: String = match value {
                Value::Str(s) => s.clone(),
                Value::Bool(b) => b.to_string(),
                Value::Num(n) => n.to_string(),
                _ => value.coerce_str(),
            };
            attrs.push(format!(
                r#"{}="{}""#,
                key,
                attr_value.replace('"', "&quot;")
            ));
        }
        attrs.join(" ")
    }
}

impl Default for Props {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prop_type_matches() {
        assert!(PropType::String.matches(&Value::Str("test".to_string())));
        assert!(PropType::Number.matches(&Value::Num(42.0)));
        assert!(PropType::Boolean.matches(&Value::Bool(true)));
        assert!(PropType::Array.matches(&Value::Array(vec![])));
        assert!(PropType::Object.matches(&Value::Object(IndexMap::new())));
        assert!(PropType::Any.matches(&Value::Null));
    }

    #[test]
    fn test_prop_decl_with_default() {
        let decl = PropDecl::new("count", PropType::Number).with_default(Value::Num(0.0));
        assert!(decl.optional);
        assert_eq!(decl.default, Some(Value::Num(0.0)));
    }

    #[test]
    fn test_props_validate_required_missing() {
        let mut props = Props::new();
        props
            .declared
            .insert("name".to_string(), PropDecl::new("name", PropType::String));

        let result = props.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_props_validate_optional_ok() {
        let mut props = Props::new();
        props.declared.insert(
            "name".to_string(),
            PropDecl::new("name", PropType::String).optional(),
        );

        assert!(props.validate().is_ok());
    }

    #[test]
    fn test_props_validate_type_mismatch() {
        let mut props = Props::new();
        props.declared.insert(
            "count".to_string(),
            PropDecl::new("count", PropType::Number),
        );
        props
            .provided
            .insert("count".to_string(), Value::Str("not a number".to_string()));

        let result = props.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_props_get_with_default() {
        let mut props = Props::new();
        props.declared.insert(
            "count".to_string(),
            PropDecl::new("count", PropType::Number).with_default(Value::Num(10.0)),
        );

        assert_eq!(props.get("count"), Some(&Value::Num(10.0)));
    }

    #[test]
    fn test_props_to_attrs() {
        let mut props = Props::new();
        props
            .provided
            .insert("class".to_string(), Value::Str("btn".to_string()));
        props
            .provided
            .insert("disabled".to_string(), Value::Bool(true));

        let attrs = props.to_attrs();
        assert!(attrs.contains("class=\"btn\""));
        assert!(attrs.contains("disabled=\"true\""));
    }

    #[test]
    fn test_props_only() {
        let mut props = Props::new();
        props.provided.insert("a".to_string(), Value::Num(1.0));
        props.provided.insert("b".to_string(), Value::Num(2.0));
        props.provided.insert("c".to_string(), Value::Num(3.0));

        let filtered = props.only(&["a", "c"]);
        assert!(filtered.provided.contains_key("a"));
        assert!(!filtered.provided.contains_key("b"));
        assert!(filtered.provided.contains_key("c"));
    }

    #[test]
    fn test_props_except() {
        let mut props = Props::new();
        props.provided.insert("a".to_string(), Value::Num(1.0));
        props.provided.insert("b".to_string(), Value::Num(2.0));
        props.provided.insert("c".to_string(), Value::Num(3.0));

        let filtered = props.except(&["b"]);
        assert!(filtered.provided.contains_key("a"));
        assert!(!filtered.provided.contains_key("b"));
        assert!(filtered.provided.contains_key("c"));
    }
}
