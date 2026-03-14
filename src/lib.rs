pub mod filters;

use glob::glob;
use serde_json::Value;
use std::cell::RefCell;
use std::rc::Rc;
use tera::{Context, Filter, Function, Tera};

mod components;
mod context;
mod error;

pub use components::{
    Component, ComponentRegistry, ComponentRenderer, PropDef, PropType, SlotData,
};
pub use context::ContextChain;
pub use error::Error;

thread_local! {
    static COMPONENT_RENDERER: RefCell<Option<Rc<RefCell<ComponentRenderer>>>> = RefCell::new(None);
}

#[derive(Clone)]
pub struct Atom {
    tera: Tera,
    components: ComponentRegistry,
    context_chain: ContextChain,
    max_loop_iter: usize,
    debug: bool,
}

impl Atom {
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
        }
    }

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

    pub fn add_template(&mut self, name: &str, content: &str) -> std::result::Result<(), Error> {
        self.tera
            .add_raw_template(name, content)
            .map_err(|e| Error::TemplateParse {
                name: name.to_string(),
                message: e.to_string(),
            })?;
        Ok(())
    }

    pub fn register_component(
        &mut self,
        path: &str,
        template: &str,
    ) -> std::result::Result<(), Error> {
        self.components.register(path, template)
    }

    pub fn register_filter<F>(&mut self, name: &str, filter: F)
    where
        F: Filter + Send + Sync + 'static,
    {
        self.tera.register_filter(name, filter);
    }

    pub fn register_function<F>(&mut self, name: &str, function: F)
    where
        F: Function + Send + Sync + 'static,
    {
        self.tera.register_function(name, function);
    }

    pub fn set_max_loop_iter(&mut self, max: usize) {
        self.max_loop_iter = max;
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

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

    pub fn provide(&mut self, key: &str, value: Value) {
        self.context_chain.provide(key, value);
    }

    pub fn reload(&mut self) -> std::result::Result<(), Error> {
        self.tera.full_reload().map_err(|e| Error::TemplateLoad {
            path: "reload".to_string(),
            message: e.to_string(),
        })
    }

    pub fn template_exists(&self, name: &str) -> bool {
        self.tera.get_template(name).is_ok()
    }

    pub fn get_registered_templates(&self) -> Vec<String> {
        self.tera
            .get_template_names()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn clear_cache(&mut self) {
        self.tera.templates.clear();
    }
}

impl Default for Atom {
    fn default() -> Self {
        Self::new()
    }
}
