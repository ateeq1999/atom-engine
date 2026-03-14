use std::collections::HashMap;

#[derive(Clone)]
pub struct DirectiveRegistry {
    directives: HashMap<String, Box<dyn Directive>>,
}

pub trait Directive: Send + Sync {
    fn name(&self) -> &str;
    fn render(&self, args: &HashMap<String, serde_json::Value>) -> Result<String, String>;
}

impl DirectiveRegistry {
    pub fn new() -> Self {
        DirectiveRegistry {
            directives: HashMap::new(),
        }
    }

    pub fn register<D: Directive + 'static>(&mut self, directive: D) {
        self.directives
            .insert(directive.name().to_string(), Box::new(directive));
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Directive>> {
        self.directives.get(name)
    }
}

impl Default for DirectiveRegistry {
    fn default() -> Self {
        Self::new()
    }
}
