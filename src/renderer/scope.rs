use crate::types::value::Value;
use indexmap::IndexMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Frame {
    pub vars: IndexMap<String, Value>,
    pub is_props: bool,
    pub is_const: HashSet<String>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            vars: IndexMap::new(),
            is_props: false,
            is_const: HashSet::new(),
        }
    }

    pub fn new_props() -> Self {
        Frame {
            vars: IndexMap::new(),
            is_props: true,
            is_const: HashSet::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.vars.get(name)
    }

    pub fn insert(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    frames: Vec<Frame>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            frames: vec![Frame::new()],
        }
    }

    pub fn push_frame(&mut self) {
        self.frames.push(Frame::new());
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        if self.frames.len() > 1 {
            self.frames.pop()
        } else {
            None
        }
    }

    pub fn push_props(&mut self, props: IndexMap<String, Value>) {
        let mut frame = Frame::new_props();
        for (k, v) in props {
            frame.vars.insert(k, v);
        }
        self.frames.push(frame);
    }

    pub fn declare(&mut self, name: &str, value: Value) -> Result<(), String> {
        let current = self.frames.last_mut().ok_or("No frames")?;

        if current.vars.contains_key(name) {
            return Err(format!(
                "Variable '{}' already declared in current scope",
                name
            ));
        }

        current.vars.insert(name.to_string(), value);
        Ok(())
    }

    pub fn declare_const(&mut self, name: &str, value: Value) -> Result<(), String> {
        let current = self.frames.last_mut().ok_or("No frames")?;

        if current.vars.contains_key(name) {
            return Err(format!(
                "Variable '{}' already declared in current scope",
                name
            ));
        }

        current.vars.insert(name.to_string(), value);
        current.is_const.insert(name.to_string());
        Ok(())
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Walk from innermost to outermost
        for frame in self.frames.iter_mut().rev() {
            if frame.vars.contains_key(name) {
                if frame.is_props {
                    return Err(format!(
                        "Cannot assign to prop '{}' - props are immutable",
                        name
                    ));
                }
                if frame.is_const.contains(name) {
                    return Err(format!("Cannot reassign const variable '{}'", name));
                }
                frame.vars.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(format!("Variable '{}' not found", name))
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        for frame in self.frames.iter().rev() {
            if let Some(v) = frame.vars.get(name) {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn has(&self, name: &str) -> bool {
        for frame in self.frames.iter().rev() {
            if frame.vars.contains_key(name) {
                return true;
            }
        }
        false
    }

    pub fn inject_loop_vars(&mut self, index: usize, total: usize) {
        if let Some(frame) = self.frames.last_mut() {
            frame
                .vars
                .insert("$index".to_string(), Value::Num(index as f64));
            frame
                .vars
                .insert("$number".to_string(), Value::Num((index + 1) as f64));
            frame
                .vars
                .insert("$first".to_string(), Value::Bool(index == 0));
            frame.vars.insert(
                "$last".to_string(),
                Value::Bool(index == total.saturating_sub(1)),
            );
            frame
                .vars
                .insert("$even".to_string(), Value::Bool(index % 2 == 0));
            frame
                .vars
                .insert("$odd".to_string(), Value::Bool(index % 2 == 1));
            frame
                .vars
                .insert("$total".to_string(), Value::Num(total as f64));
        }
    }

    pub fn current_frame(&self) -> Option<&Frame> {
        self.frames.last()
    }

    pub fn depth(&self) -> usize {
        self.frames.len()
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_declare_and_get() {
        let mut scope = Scope::new();
        scope.declare("x", Value::Num(42.0)).unwrap();
        assert_eq!(scope.get("x"), Some(Value::Num(42.0)));
    }

    #[test]
    fn test_declare_error_on_duplicate() {
        let mut scope = Scope::new();
        scope.declare("x", Value::Num(1.0)).unwrap();
        let result = scope.declare("x", Value::Num(2.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_assign() {
        let mut scope = Scope::new();
        scope.declare("x", Value::Num(1.0)).unwrap();
        scope.assign("x", Value::Num(2.0)).unwrap();
        assert_eq!(scope.get("x"), Some(Value::Num(2.0)));
    }

    #[test]
    fn test_assign_not_found() {
        let mut scope = Scope::new();
        let result = scope.assign("x", Value::Num(1.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_scope_shadow() {
        let mut scope = Scope::new();
        scope.declare("x", Value::Num(1.0)).unwrap();
        scope.push_frame();
        scope.declare("x", Value::Num(2.0)).unwrap();
        // Inner scope should have 2
        assert_eq!(scope.get("x"), Some(Value::Num(2.0)));
        scope.pop_frame();
        // Outer scope should still have 1
        assert_eq!(scope.get("x"), Some(Value::Num(1.0)));
    }

    #[test]
    fn test_props_immutable() {
        let mut scope = Scope::new();
        scope.push_props(vec![].into_iter().collect());
        let result = scope.assign("anything", Value::Num(1.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_loop_vars() {
        let mut scope = Scope::new();
        scope.declare("x", Value::Num(0.0)).unwrap();
        scope.inject_loop_vars(2, 5);
        assert_eq!(scope.get("$index"), Some(Value::Num(2.0)));
        assert_eq!(scope.get("$number"), Some(Value::Num(3.0)));
        assert_eq!(scope.get("$first"), Some(Value::Bool(false)));
        assert_eq!(scope.get("$last"), Some(Value::Bool(false)));
        assert_eq!(scope.get("$even"), Some(Value::Bool(true)));
        assert_eq!(scope.get("$odd"), Some(Value::Bool(false)));
        assert_eq!(scope.get("$total"), Some(Value::Num(5.0)));
    }
}
