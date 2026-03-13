use crate::error::RenderError;

pub trait TransformFn: Send + Sync {
    fn apply(&self, content: &str) -> Result<String, RenderError>;
}

impl<F> TransformFn for F
where
    F: Fn(&str) -> Result<String, RenderError> + Send + Sync + 'static,
{
    fn apply(&self, content: &str) -> Result<String, RenderError> {
        self(content)
    }
}
