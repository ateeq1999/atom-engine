//! Atom Engine - A component-oriented template engine for Rust
//! 
//! # Overview
//! 
//! Atom Engine is a high-performance template engine built on Tera with additional
//! features like components, props, slots, and provide/inject context.
//! 
//! # Features
//! 
//! - Built on Tera for robust template parsing and rendering
//! - Component system with props, slots, and validation
//! - Provide/Inject context (React-style)
//! - Stack system for content accumulation
//! - 50+ built-in filters
//! - Helper directives (@map, @filter, @each, @reduce)
//! - Async and parallel rendering support
//! - Component caching
//! 
//! # Quick Start
//! 
//! ```rust
//! use atom_engine::Atom;
//! use serde_json::json;
//! 
//! let mut engine = Atom::new();
//! engine.add_template("hello.html", "Hello, {{ name }}!").unwrap();
//! let result = engine.render("hello.html", &json!({"name": "World"})).unwrap();
//! assert_eq!(result, "Hello, World!");
//! ```

pub mod filters;

use glob::glob;
use serde_json::Value;
use std::cell::RefCell;
use std::rc::Rc;
use tera::{Context, Filter, Function, Tera};

mod components;
mod context;
mod error;
mod pool;

pub use components::{
    compute_cache_key, compute_props_hash, Component, ComponentCache, ComponentRegistry,
    ComponentRenderer, PropDef, PropType, ScopedSlotDef, SlotData,
};
pub use context::ContextChain;
pub use error::Error;
pub use pool::{MemoryPool, PooledString, StringPool};

thread_local! {
    static COMPONENT_RENDERER: RefCell<Option<Rc<RefCell<ComponentRenderer>>>> = const { RefCell::new(None) };
}

/// The main template engine struct.
///
/// Atom provides methods for creating templates, registering components,
/// rendering templates, and managing context.
///
/// # Example
///
/// ```rust
/// use atom_engine::Atom;
/// use serde_json::json;
///
/// let mut engine = Atom::new();
/// engine.add_template("greeting.html", "Hello, {{ name }}!").unwrap();
/// let result = engine.render("greeting.html", &json!({"name": "Alice"})).unwrap();
/// assert_eq!(result, "Hello, Alice!");
/// ```
#[derive(Clone)]
pub struct Atom {
    tera: Tera,
    components: ComponentRegistry,
    context_chain: ContextChain,
    max_loop_iter: usize,
    debug: bool,
    use_parallel: bool,
}

impl Atom {
    /// Creates a new Atom engine instance with all built-in filters and functions registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// use atom_engine::Atom;
    ///
    /// let engine = Atom::new();
    /// ```
    pub fn new() -> Self {
        let mut tera = Tera::default();

        // JSON
        tera.register_filter("json_encode", filters::json_encode);

        // String filters
        tera.register_filter("upper", filters::upper);
        tera.register_filter("lower", filters::lower);
        tera.register_filter("capitalize", filters::capitalize);
        tera.register_filter("title", filters::title);
        tera.register_filter("camel_case", filters::camel_case);
        tera.register_filter("pascal_case", filters::pascal_case);
        tera.register_filter("snake_case", filters::snake_case);
        tera.register_filter("kebab_case", filters::kebab_case);
        tera.register_filter("truncate", filters::truncate);
        tera.register_filter("slugify", filters::slugify);
        tera.register_filter("pluralize", filters::pluralize);
        tera.register_filter("replace", filters::replace);
        tera.register_filter("remove", filters::remove);
        tera.register_filter("prepend", filters::prepend);
        tera.register_filter("append", filters::append);
        tera.register_filter("strip", filters::strip);
        tera.register_filter("nl2br", filters::nl2br);
        tera.register_filter("word_count", filters::word_count);
        tera.register_filter("char_count", filters::char_count);
        tera.register_filter("starts_with", filters::starts_with);
        tera.register_filter("ends_with", filters::ends_with);
        tera.register_filter("contains", filters::contains);

        // Collection filters
        tera.register_filter("first", filters::first);
        tera.register_filter("last", filters::last);
        tera.register_filter("length", filters::length);
        tera.register_filter("reverse", filters::reverse);
        tera.register_filter("sort", filters::sort);
        tera.register_filter("group_by", filters::group_by);
        tera.register_filter("where", filters::where_filter);
        tera.register_filter("pluck", filters::pluck);
        tera.register_filter("join", filters::join);
        tera.register_filter("slice", filters::slice);
        tera.register_filter("uniq", filters::uniq);
        tera.register_filter("shuffle", filters::shuffle);
        tera.register_filter("map", filters::map_filter);
        tera.register_filter("filter", filters::filter_filter);
        tera.register_filter("each", filters::each_filter);
        tera.register_filter("reduce", filters::reduce_filter);
        tera.register_filter("flatten", filters::flatten_filter);
        tera.register_filter("partition", filters::partition_filter);

        // Number filters
        tera.register_filter("round", filters::round);
        tera.register_filter("abs", filters::abs);
        tera.register_filter("format", filters::format_number);
        tera.register_filter("min", filters::min_filter);
        tera.register_filter("max", filters::max_filter);
        tera.register_filter("sum", filters::sum);
        tera.register_filter("avg", filters::avg);
        tera.register_filter("ceil", filters::ceil);
        tera.register_filter("floor", filters::floor);

        // Date filters
        tera.register_filter("date", filters::date_format);

        // HTML filters
        tera.register_filter("escape_html", filters::escape_html);
        tera.register_filter("safe", filters::safe);

        // Encoding filters
        tera.register_filter("json_decode", filters::json_decode);
        tera.register_filter("urlescape", filters::urlescape);
        tera.register_filter("urlunescape", filters::urlunescape);
        tera.register_filter("strip_tags", filters::strip_tags);
        tera.register_filter("base64_encode", filters::base64_encode);
        tera.register_filter("base64_decode", filters::base64_decode);

        // Slot helpers
        tera.register_filter("slot", filters::slot_filter);
        tera.register_filter("has_slot", filters::has_slot_filter);
        tera.register_filter("scoped_slot", filters::scoped_slot_filter);
        tera.register_filter("with_scoped_data", filters::with_scoped_data_filter);

        // Stack filter
        tera.register_filter("stack", filters::stack_filter);

        // Conditional filters
        tera.register_filter("when", filters::when);
        tera.register_filter("default", filters::default_filter);
        tera.register_filter("coalesce", filters::coalesce);
        tera.register_filter("defined", filters::defined);
        tera.register_filter("undefined", filters::undefined);
        tera.register_filter("empty", filters::empty);
        tera.register_filter("not_empty", filters::not_empty);

        // Global functions
        tera.register_function("dump", filters::DumpFn);
        tera.register_function("log", filters::LogFn);
        tera.register_function("range", filters::RangeFn);
        tera.register_function("now", filters::NowFn);
        tera.register_function("cycle", filters::CycleFn::new());
        tera.register_function("uuid", filters::UuidFn);
        tera.register_function("random", filters::RandomFn);
        tera.register_function("choice", filters::ChoiceFn);
        tera.register_function("file_exists", filters::FileExistsFn);
        tera.register_function("env", filters::EnvFn);
        tera.register_function("md5", filters::Md5Fn);
        tera.register_function("sha256", filters::Sha256Fn);
        tera.register_function("repeat", filters::RepeatFn);
        tera.register_function("times", filters::TimesFn);
        tera.register_function("loop", filters::LoopFn);
        tera.register_function("iterate", filters::IterateFn);
        tera.register_function("object", filters::ObjectFn);
        tera.register_function("merge", filters::MergeFn);
        tera.register_function("chunk", filters::ChunkFn);
        tera.register_function("zip", filters::ZipFn);
        tera.register_function("compact", filters::CompactFn);

        // Component functions
        tera.register_function("push", filters::PushFn);
        tera.register_function("prepend", filters::PrependFn);
        tera.register_function("set_slot", filters::SetSlotFn);
        tera.register_function("once", filters::OnceFn);

        Atom {
            tera,
            components: ComponentRegistry::new(),
            context_chain: ContextChain::new(),
            max_loop_iter: 10000,
            debug: false,
            use_parallel: false,
        }
    }

    /// Loads templates from the filesystem using a glob pattern.
    ///
    /// # Arguments
    ///
    /// * `glob_pattern` - A glob pattern to match template files (e.g., "templates/**/*.html")
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut engine = atom_engine::Atom::new();
    /// engine.load_templates("templates/**/*.html").unwrap();
    /// ```
    pub fn load_templates(&mut self, glob_pattern: &str) -> std::result::Result<(), Error> {
        let template_files: Vec<(std::path::PathBuf, Option<String>)> = glob(glob_pattern)
            .map_err(|e| Error::TemplateLoad {
                path: glob_pattern.to_string(),
                message: e.to_string(),
            })?
            .filter_map(|p| p.ok())
            .map(|p| {
                let name = p
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                (p, Some(name))
            })
            .collect();

        self.tera
            .add_template_files(template_files.into_iter())
            .map_err(|e| Error::TemplateLoad {
                path: glob_pattern.to_string(),
                message: e.to_string(),
            })?;
        Ok(())
    }

    /// Adds a raw template to the engine.
    ///
    /// # Arguments
    ///
    /// * `name` - The template name (e.g., "index.html")
    /// * `content` - The template content
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut engine = atom_engine::Atom::new();
    /// engine.add_template("hello.html", "Hello, {{ name }}!").unwrap();
    /// ```
    pub fn add_template(&mut self, name: &str, content: &str) -> std::result::Result<(), Error> {
        self.tera
            .add_raw_template(name, content)
            .map_err(|e| Error::TemplateParse {
                name: name.to_string(),
                message: e.to_string(),
            })?;
        Ok(())
    }

    /// Registers a reusable component.
    ///
    /// # Arguments
    ///
    /// * `path` - Component path/name (e.g., "button")
    /// * `template` - Component template content
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut engine = atom_engine::Atom::new();
    /// engine.register_component("button", "<button>{{ text }}</button>").unwrap();
    /// ```
    pub fn register_component(
        &mut self,
        path: &str,
        template: &str,
    ) -> std::result::Result<(), Error> {
        self.components.register(path, template)
    }

    /// Registers a custom filter.
    ///
    /// # Arguments
    ///
    /// * `name` - Filter name
    /// * `filter` - The filter function
    pub fn register_filter<F>(&mut self, name: &str, filter: F)
    where
        F: Filter + Send + Sync + 'static,
    {
        self.tera.register_filter(name, filter);
    }

    /// Registers a custom global function.
    ///
    /// # Arguments
    ///
    /// * `name` - Function name
    /// * `function` - The function to register
    pub fn register_function<F>(&mut self, name: &str, function: F)
    where
        F: Function + Send + Sync + 'static,
    {
        self.tera.register_function(name, function);
    }

    /// Sets the maximum number of iterations for loops.
    ///
    /// This prevents infinite loops in templates.
    pub fn set_max_loop_iter(&mut self, max: usize) {
        self.max_loop_iter = max;
    }

    /// Sets debug mode for the engine.
    ///
    /// When enabled, additional debugging information may be logged.
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    /// Renders a template with the given context.
    ///
    /// # Arguments
    ///
    /// * `template` - The template name to render
    /// * `context` - The context data as a JSON value
    ///
    /// # Example
    ///
    /// ```rust
    /// use atom_engine::Atom;
    /// use serde_json::json;
    ///
    /// let mut engine = Atom::new();
    /// engine.add_template("greeting.html", "Hello, {{ name }}!").unwrap();
    /// let result = engine.render("greeting.html", &json!({"name": "World"})).unwrap();
    /// assert_eq!(result, "Hello, World!");
    /// ```
    pub fn render(&self, template: &str, context: &Value) -> Result<String, Error> {
        let mut ctx = Context::from_serialize(context).map_err(|e| Error::Context {
            message: e.to_string(),
        })?;

        for (key, value) in self.context_chain.all() {
            ctx.insert(key, &value);
        }

        ctx.insert("__atom_components", &self.components.list_components());

        self.tera.render(template, &ctx).map_err(|e| Error::Render {
            template: template.to_string(),
            message: e.to_string(),
        })
    }

    /// Renders a template with component data included in the context.
    ///
    /// This is useful when you want to pass additional component-specific data
    /// alongside the regular context.
    ///
    /// # Arguments
    ///
    /// * `template` - The template name to render
    /// * `context` - The context data as a JSON value
    /// * `component_data` - Additional component-specific data
    pub fn render_with_components(
        &self,
        template: &str,
        context: &Value,
        component_data: &Value,
    ) -> std::result::Result<String, Error> {
        let mut ctx = Context::from_serialize(context).map_err(|e| Error::Context {
            message: e.to_string(),
        })?;

        if let Some(obj) = component_data.as_object() {
            for (key, value) in obj {
                ctx.insert(key, &value);
            }
        }

        self.tera.render(template, &ctx).map_err(|e| Error::Render {
            template: template.to_string(),
            message: e.to_string(),
        })
    }

    /// Provides a value to the context chain.
    ///
    /// This implements a provide/inject pattern (similar to Vue.js) where
    /// values can be provided at a higher level and injected in child components.
    ///
    /// # Arguments
    ///
    /// * `key` - The context key
    /// * `value` - The value to provide
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use atom_engine::Atom;
    /// use serde_json::json;
    ///
    /// let mut engine = Atom::new();
    /// engine.add_template("child.html", "{{ theme }}").unwrap();
    /// engine.provide("theme", json!("dark"));
    /// let result = engine.render("child.html", &json!({})).unwrap();
    /// assert_eq!(result, "dark");
    /// ```
    pub fn provide(&mut self, key: &str, value: Value) {
        self.context_chain.provide(key, value);
    }

    /// Reloads all templates from the filesystem.
    ///
    /// This is useful during development when templates change on disk.
    pub fn reload(&mut self) -> std::result::Result<(), Error> {
        self.tera.full_reload().map_err(|e| Error::TemplateLoad {
            path: "reload".to_string(),
            message: e.to_string(),
        })
    }

    /// Checks if a template exists in the engine.
    ///
    /// # Arguments
    ///
    /// * `name` - The template name to check
    ///
    /// # Returns
    ///
    /// `true` if the template exists, `false` otherwise
    pub fn template_exists(&self, name: &str) -> bool {
        self.tera.get_template(name).is_ok()
    }

    /// Gets a list of all registered template names.
    ///
    /// # Returns
    ///
    /// A vector of template names
    pub fn get_registered_templates(&self) -> Vec<String> {
        self.tera
            .get_template_names()
            .map(|s| s.to_string())
            .collect()
    }

    /// Clears the template cache.
    ///
    /// This forces templates to be re-parsed on next render.
    pub fn clear_cache(&mut self) {
        self.tera.templates.clear();
    }

    /// Enables or disables parallel rendering.
    ///
    /// When enabled with the `parallel` feature, multiple templates can be
    /// rendered concurrently using Rayon.
    pub fn set_parallel(&mut self, enabled: bool) {
        self.use_parallel = enabled;
    }

    /// Returns whether parallel rendering is enabled.
    pub fn is_parallel(&self) -> bool {
        self.use_parallel
    }

    /// Enables or disables component caching.
    ///
    /// When enabled, rendered components are cached based on their props hash.
    pub fn enable_component_cache(&mut self, enabled: bool) {
        self.components.enable_cache(enabled);
    }

    /// Returns whether component caching is enabled.
    pub fn is_component_cache_enabled(&self) -> bool {
        self.components.is_cache_enabled()
    }

    /// Clears the component cache.
    pub fn clear_component_cache(&mut self) {
        self.components.clear_cache();
    }

    /// Returns the number of cached component renders.
    pub fn component_cache_len(&self) -> usize {
        self.components.cache_len()
    }

    /// Renders multiple templates in parallel (when the `parallel` feature is enabled).
    ///
    /// Requires the `parallel` feature to be enabled. Without it, templates
    /// are rendered sequentially.
    ///
    /// # Arguments
    ///
    /// * `templates` - A slice of template name and context pairs
    ///
    /// # Returns
    ///
    /// A vector of (template_name, rendered_output) pairs
    #[cfg(feature = "parallel")]
    pub fn render_many(
        &self,
        templates: &[(&str, &Value)],
    ) -> std::result::Result<Vec<(String, String)>, Error> {
        use rayon::prelude::*;

        let results: Vec<std::result::Result<(String, String), Error>> = templates
            .par_iter()
            .map(|(name, context)| {
                let rendered = self.render(name, context)?;
                Ok((name.to_string(), rendered))
            })
            .collect();

        let mut output = Vec::new();
        for result in results {
            output.push(result?);
        }
        Ok(output)
    }

    /// Renders multiple templates sequentially.
    ///
    /// This is the fallback implementation when the `parallel` feature is not enabled.
    #[cfg(not(feature = "parallel"))]
    pub fn render_many(
        &self,
        templates: &[(&str, &Value)],
    ) -> std::result::Result<Vec<(String, String)>, Error> {
        let mut results = Vec::new();
        for (name, context) in templates {
            let rendered = self.render(name, context)?;
            results.push((name.to_string(), rendered));
        }
        Ok(results)
    }

    /// Renders a template asynchronously.
    ///
    /// Requires the `async` feature to be enabled.
    ///
    /// # Arguments
    ///
    /// * `template` - The template name to render
    /// * `context` - The context data as a JSON value
    ///
    /// # Example
    ///
    /// ```rust
    /// #[cfg(feature = "async")]
    /// async fn render_template() {
    ///     use atom_engine::Atom;
    ///     use serde_json::json;
    ///
    ///     let engine = Atom::new();
    ///     let result = engine.render_async("hello.html", &json!({"name": "World"})).await;
    /// }
    /// ```
    #[cfg(feature = "async")]
    pub async fn render_async(&self, template: &str, context: &Value) -> Result<String, Error> {
        let template = template.to_string();
        let context = context.clone();

        tokio::task::spawn_blocking(move || {
            let mut tera = Tera::default();
            tera.register_filter("json_encode", filters::json_encode);
            tera.render(
                &template,
                &Context::from_serialize(&context).map_err(|e| Error::Context {
                    message: e.to_string(),
                })?,
            )
            .map_err(|e| Error::Render {
                template,
                message: e.to_string(),
            })
        })
        .await
        .map_err(|e| Error::Render {
            template: "async".to_string(),
            message: e.to_string(),
        })?
    }

    /// Renders multiple templates asynchronously.
    ///
    /// Requires the `async` feature to be enabled.
    ///
    /// # Arguments
    ///
    /// * `templates` - A slice of template name and context pairs
    ///
    /// # Returns
    ///
    /// A vector of (template_name, rendered_output) pairs
    #[cfg(feature = "async")]
    pub async fn render_many_async(
        &self,
        templates: &[(&str, &Value)],
    ) -> std::result::Result<Vec<(String, String)>, Error> {
        use tokio::task::JoinSet;

        let mut join_set = JoinSet::new();

        for (name, context) in templates {
            let name = name.to_string();
            let context = context.clone();
            let filters = filters::Filters::new();

            join_set.spawn(async move {
                tokio::task::spawn_blocking(move || {
                    let mut tera = Tera::default();
                    tera.register_filter("json_encode", filters::json_encode);
                    let mut ctx =
                        Context::from_serialize(&context).map_err(|e| Error::Context {
                            message: e.to_string(),
                        })?;
                    tera.render(&name, &ctx)
                        .map(|r| (name, r))
                        .map_err(|e| Error::Render {
                            template: name,
                            message: e.to_string(),
                        })
                })
                .await
                .map_err(|e| Error::Render {
                    template: "async".to_string(),
                    message: e.to_string(),
                })?
            });
        }

        let mut results = Vec::new();
        while let Some(result) = join_set.join_next().await {
            results.push(result??);
        }
        Ok(results)
    }
}

impl Default for Atom {
    fn default() -> Self {
        Self::new()
    }
}
