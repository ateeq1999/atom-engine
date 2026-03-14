//! Filter modules for Atom Engine.
//!
//! This module re-exports all filter categories:
//! - `string` - String manipulation filters
//! - `collection` - Array/object manipulation filters
//! - `number` - Number formatting filters
//! - `date` - Date formatting filters
//! - `html` - HTML processing filters
//! - `encoding` - Encoding/decoding filters
//! - `conditional` - Conditional filters
//! - `system` - System utility filters
//! - `component` - Component-specific filters

pub mod collection;
pub mod component;
pub mod conditional;
pub mod date;
pub mod encoding;
pub mod html;
pub mod number;
pub mod string;
pub mod system;

pub type FilterResult = Result<serde_json::Value, tera::Error>;

pub use collection::*;
pub use component::*;
pub use conditional::*;
pub use date::*;
pub use encoding::*;
pub use html::*;
pub use number::*;
pub use string::*;
pub use system::*;

/// Container for filter utilities.
/// Container for filter utilities.
pub struct Filters;

impl Filters {
    /// Creates a new Filters instance.
    pub fn new() -> Self {
        Filters
    }
}

impl Default for Filters {
    fn default() -> Self {
        Self::new()
    }
}
