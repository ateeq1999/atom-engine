use crate::error::RenderError;
use crate::types::value::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub trait HelperFn: Send + Sync {
    fn call(
        &self,
        receiver: Option<Value>,
        args: Vec<Value>,
        ctx: &HelperCtx,
    ) -> Result<Value, RenderError>;
}

pub struct HelperCtx<'r> {
    pub debug: bool,
    _marker: std::marker::PhantomData<&'r ()>,
}

impl<'r> HelperCtx<'r> {
    pub fn new() -> Self {
        HelperCtx {
            debug: false,
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct HelperRegistry {
    helpers: HashMap<String, Arc<dyn HelperFn>>,
}

impl HelperRegistry {
    pub fn new() -> Self {
        let mut helpers = HashMap::new();

        // Register basic helpers that will be implemented in Phase 9
        // For now, we'll add string helpers and collection helpers stubs

        HelperRegistry { helpers }
    }

    pub fn register(&mut self, name: String, helper: Arc<dyn HelperFn>) {
        self.helpers.insert(name, helper);
    }

    pub fn get(&self, name: &str) -> Option<&Arc<dyn HelperFn>> {
        self.helpers.get(name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.helpers.contains_key(name)
    }

    pub fn call_method(
        &self,
        receiver: Option<Value>,
        method: &str,
        args: Vec<Value>,
        ctx: &HelperCtx,
    ) -> Result<Value, RenderError> {
        let key = format!(
            "{}.{}",
            receiver
                .as_ref()
                .map(|v| v.type_name())
                .unwrap_or("unknown"),
            method
        );

        if let Some(helper) = self.helpers.get(&key) {
            helper.call(receiver, args, ctx)
        } else {
            Err(RenderError::HelperError {
                name: method.to_string(),
                message: format!("Unknown method '{}'", method),
            })
        }
    }

    pub fn call_global(
        &self,
        name: &str,
        args: Vec<Value>,
        ctx: &HelperCtx,
    ) -> Result<Value, RenderError> {
        if let Some(helper) = self.helpers.get(name) {
            helper.call(None, args, ctx)
        } else {
            Err(RenderError::HelperError {
                name: name.to_string(),
                message: format!("Unknown helper '{}'", name),
            })
        }
    }
}

impl Default for HelperRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Simple helper implementation for testing
pub struct SimpleHelper<F> {
    func: F,
}

impl<F> SimpleHelper<F>
where
    F: Fn(Option<Value>, Vec<Value>, &HelperCtx) -> Result<Value, RenderError>
        + Send
        + Sync
        + 'static,
{
    pub fn new(func: F) -> Arc<dyn HelperFn> {
        Arc::new(SimpleHelper { func })
    }
}

impl<F> HelperFn for SimpleHelper<F>
where
    F: Fn(Option<Value>, Vec<Value>, &HelperCtx) -> Result<Value, RenderError>
        + Send
        + Sync
        + 'static,
{
    fn call(
        &self,
        receiver: Option<Value>,
        args: Vec<Value>,
        ctx: &HelperCtx,
    ) -> Result<Value, RenderError> {
        (self.func)(receiver, args, ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_new() {
        let registry = HelperRegistry::new();
        assert!(!registry.has("any"));
    }

    #[test]
    fn test_call_unknown_helper() {
        let registry = HelperRegistry::new();
        let ctx = HelperCtx::new();
        let result = registry.call_global("unknown", vec![], &ctx);
        assert!(result.is_err());
    }
}
