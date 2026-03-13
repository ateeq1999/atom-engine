pub mod directives;
pub mod error;
pub mod helpers;
pub mod parser;
pub mod renderer;
pub mod types;

pub use error::{ParseError, RenderError};
pub use parser::parser::Template;
pub use renderer::helper_eval::{HelperFn, HelperRegistry};
pub use renderer::renderer::TemplateRegistry;
pub use types::value::Value;

use parser::lexer::Lexer;
use parser::parser::Parser;
use renderer::renderer::Renderer;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub escape_html: bool,
    pub max_loop_iter: usize,
    pub debug: bool,
    pub strict_props: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        EngineConfig {
            escape_html: true,
            max_loop_iter: 1000,
            debug: false,
            strict_props: false,
        }
    }
}

#[derive(Clone)]
pub struct Engine {
    registry: TemplateRegistry,
    helpers: HelperRegistry,
    transforms: TransformRegistry,
    config: EngineConfig,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            registry: TemplateRegistry::new(),
            helpers: HelperRegistry::new(),
            transforms: TransformRegistry::new(),
            config: EngineConfig::default(),
        }
    }

    pub fn with_config(config: EngineConfig) -> Self {
        Engine {
            registry: TemplateRegistry::new(),
            helpers: HelperRegistry::new(),
            transforms: TransformRegistry::new(),
            config,
        }
    }

    pub fn add_disk(&mut self, name: &str, templates: HashMap<String, Arc<Template>>) {
        self.registry.add_disk(name, templates);
    }

    pub fn add_embedded(&mut self, templates: HashMap<String, Arc<Template>>) {
        self.registry.add_disk("default", templates);
    }

    pub fn register_helper(&mut self, name: &str, helper: Arc<dyn HelperFn>) {
        self.helpers.register(name.to_string(), helper);
    }

    pub fn register_transform(&mut self, name: &str, transform: Arc<dyn TransformFn>) {
        self.transforms.register(name, transform);
    }

    pub fn render(&self, path: &str, data: Value) -> Result<String, RenderError> {
        let template = self
            .registry
            .get(path)
            .ok_or_else(|| RenderError::TemplateNotFound {
                path: path.to_string(),
            })?;

        let mut renderer = Renderer::new(&template, &self.helpers, &self.registry);
        renderer.max_loop_iter = self.config.max_loop_iter;
        renderer.debug = self.config.debug;
        renderer.render()
    }

    pub fn render_raw(&self, source: &str, data: Value) -> Result<String, RenderError> {
        let template = self.parse(source)?;

        let mut renderer = Renderer::new(&template, &self.helpers, &self.registry);
        renderer.max_loop_iter = self.config.max_loop_iter;
        renderer.debug = self.config.debug;
        renderer.with_data(data).render()
    }

    pub fn parse(&self, source: &str) -> Result<Template, ParseError> {
        let tokens = Lexer::tokenize(source);
        let mut parser = Parser::new(tokens, source.to_string());
        Ok(parser.parse()?)
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct TransformRegistry {
    transforms: HashMap<String, Arc<dyn TransformFn>>,
}

pub trait TransformFn: Send + Sync {
    fn apply(&self, content: &str) -> Result<String, RenderError>;
}

impl TransformRegistry {
    pub fn new() -> Self {
        TransformRegistry {
            transforms: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, transform: Arc<dyn TransformFn>) {
        self.transforms.insert(name.to_string(), transform);
    }

    pub fn apply(&self, name: &str, content: &str) -> Result<String, RenderError> {
        self.transforms
            .get(name)
            .ok_or_else(|| RenderError::TransformError {
                name: name.to_string(),
                message: format!("Transform '{}' not found", name),
            })
            .and_then(|t| t.apply(content))
    }
}

impl Default for TransformRegistry {
    fn default() -> Self {
        Self::new()
    }
}
