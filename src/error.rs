//! Error types for Atom Engine.
//!
//! This module defines all error types that can occur during
//! template loading, parsing, rendering, and component operations.

use thiserror::Error;

/// All error types that can occur in Atom Engine.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to load templates from {path}: {message}")]
    TemplateLoad { path: String, message: String },

    #[error("Failed to parse template {name}: {message}")]
    TemplateParse { name: String, message: String },

    #[error("Template render error in {template}: {message}")]
    Render { template: String, message: String },

    #[error("Context error: {message}")]
    Context { message: String },

    #[error("Component error: {message}")]
    Component { message: String },

    #[error("Slot error: {message}")]
    Slot { message: String },

    #[error("Props error: {message}")]
    Props { message: String },

    #[error("Filter error: {message}")]
    Filter { message: String },

    #[error("Function error: {message}")]
    Function { message: String },
}

pub type Result<T> = std::result::Result<T, Error>;
