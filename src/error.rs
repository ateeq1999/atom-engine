use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Parse error: {0}")]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("Missing required prop `{name}` in template `{template}`")]
    MissingRequiredProp { name: String, template: String },

    #[error("Prop type mismatch for `{name}`: expected {expected}, found {found}")]
    PropTypeMismatch {
        name: String,
        expected: String,
        found: String,
    },

    #[error("Render error: {0}")]
    Unknown(String),
}
