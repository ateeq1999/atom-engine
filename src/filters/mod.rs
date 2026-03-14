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
