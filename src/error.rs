use crate::parser::lexer::Span;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ParseError {
    #[error("Parse error: {0}")]
    Unknown(String),

    #[error("Unexpected token: expected {expected}, found {found}")]
    UnexpectedToken { expected: String, found: String },

    #[error("Unclosed block directive: {directive}")]
    UnclosedBlock { directive: String },

    #[error("Expression error: {message}")]
    ExprError { message: String },

    #[error("Unknown directive: {name}")]
    UnknownDirective { name: String },

    #[error("@elseif without matching @if")]
    ElseifWithoutIf,

    #[error("@else without matching @if")]
    ElseWithoutIf,

    #[error("@fill directive outside of @use component")]
    FillOutsideComponent,

    #[error("@slot directive outside of component")]
    SlotOutsideComponent,

    #[error("Cannot @set prop - props are immutable")]
    SetTargetingProp { name: String },

    #[error("Cannot reassign const variable")]
    ConstReassignment { name: String },

    #[error("Duplicate prop declaration")]
    DuplicatePropDeclaration { name: String },

    #[error("Missing required argument '{arg}' for @{directive}")]
    MissingRequiredArg { directive: String, arg: String },

    #[error("Invalid slot name: {name}")]
    InvalidSlotName { name: String },

    #[error("Multiple @extends directives")]
    MultipleExtends,

    #[error("@extends must be the first directive")]
    ExtendsNotFirst,
}

impl From<crate::parser::parser::ParseError> for ParseError {
    fn from(e: crate::parser::parser::ParseError) -> Self {
        match e {
            crate::parser::parser::ParseError::UnexpectedToken(_, expected, found) => {
                ParseError::UnexpectedToken { expected, found }
            }
            crate::parser::parser::ParseError::UnclosedBlock { directive, .. } => {
                ParseError::UnclosedBlock { directive }
            }
            crate::parser::parser::ParseError::ExprError(_, message) => {
                ParseError::ExprError { message }
            }
            crate::parser::parser::ParseError::UnknownDirective { name, .. } => {
                ParseError::UnknownDirective { name }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub file: String,
    pub span: Span,
    pub note: Option<String>,
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

    #[error("Undefined variable `{name}`")]
    UndefinedVariable { name: String },

    #[error("Cannot set undeclared variable `{name}`")]
    SetUndeclaredVariable { name: String },

    #[error("Template not found: `{path}`")]
    TemplateNotFound { path: String },

    #[error("Maximum loop iterations ({limit}) exceeded")]
    MaxLoopIterationsExceeded { limit: usize },

    #[error("Helper error in `{name}`: {message}")]
    HelperError { name: String, message: String },

    #[error("Transform error in `{name}`: {message}")]
    TransformError { name: String, message: String },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Cannot @inject key - no matching @provide")]
    InjectWithoutProvide { key: String },

    #[error("Required slot not filled in component")]
    RequiredSlotUnfilled { name: String, component: String },

    #[error("Circular include: {a} -> {b}")]
    CircularInclude { a: String, b: String },

    #[error("Render error: {0}")]
    Unknown(String),
}

impl From<ParseError> for RenderError {
    fn from(e: ParseError) -> Self {
        RenderError::Unknown(e.to_string())
    }
}

pub type Result<T, E = ParseError> = std::result::Result<T, E>;
pub type RenderResult<T, E = RenderError> = std::result::Result<T, E>;
