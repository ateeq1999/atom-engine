use crate::error::RenderError;
use crate::parser::expr_parser::Expr;
use crate::parser::parser::{Node, Template};
use crate::renderer::context_chain::ContextChain;
use crate::renderer::expr_eval::{eval_expr, EvalCtx};
use crate::renderer::helper_eval::HelperRegistry;
use crate::renderer::output::OutputBuffer;
use crate::renderer::scope::Scope;
use crate::renderer::section_map::SectionMap;
use crate::renderer::slot_resolver::SlotResolver;
use crate::renderer::stack_buffer::StackBuffer;
use crate::types::value::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Renderer<'a> {
    pub scope: Scope,
    pub context_chain: ContextChain,
    pub helpers: &'a HelperRegistry,
    pub output: OutputBuffer,
    pub stack_buffer: StackBuffer,
    pub slot_resolver: SlotResolver,
    pub section_map: SectionMap,
    pub template: &'a Template,
    pub templates: &'a TemplateRegistry,
    pub max_loop_iter: usize,
    pub debug: bool,
    pub once_set: HashMap<String, ()>,
}

pub struct TemplateRegistry {
    disks: HashMap<String, HashMap<String, Arc<Template>>>,
}

impl Clone for TemplateRegistry {
    fn clone(&self) -> Self {
        TemplateRegistry {
            disks: self.disks.clone(),
        }
    }
}

impl TemplateRegistry {
    pub fn new() -> Self {
        TemplateRegistry {
            disks: HashMap::new(),
        }
    }

    pub fn add_disk(&mut self, name: &str, templates: HashMap<String, Arc<Template>>) {
        self.disks.insert(name.to_string(), templates);
    }

    pub fn get(&self, path: &str) -> Option<Arc<Template>> {
        if let Some((disk, path)) = path.split_once("::") {
            self.disks.get(disk).and_then(|d| d.get(path)).cloned()
        } else {
            self.disks.get("default").and_then(|d| d.get(path)).cloned()
        }
    }
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Renderer<'a> {
    pub fn new(
        template: &'a Template,
        helpers: &'a HelperRegistry,
        templates: &'a TemplateRegistry,
    ) -> Self {
        Renderer {
            scope: Scope::new(),
            context_chain: ContextChain::new(),
            helpers,
            output: OutputBuffer::new(),
            stack_buffer: StackBuffer::new(),
            slot_resolver: SlotResolver::new(),
            section_map: SectionMap::new(),
            template,
            templates,
            max_loop_iter: 1000,
            debug: false,
            once_set: HashMap::new(),
        }
    }

    pub fn with_data(mut self, data: Value) -> Self {
        if let Value::Object(map) = data {
            for (k, v) in map {
                self.scope.declare(&k, v);
            }
        }
        self
    }

    pub fn with_max_loop_iter(mut self, max: usize) -> Self {
        self.max_loop_iter = max;
        self
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn render(&mut self) -> Result<String, RenderError> {
        self.render_nodes(&self.template.nodes)?;
        Ok(self.output.as_str().to_string())
    }

    pub fn render_nodes(&mut self, nodes: &[Node]) -> Result<(), RenderError> {
        for node in nodes {
            self.render_node(node)?;
        }
        Ok(())
    }

    pub fn render_node(&mut self, node: &Node) -> Result<(), RenderError> {
        match node {
            Node::Text(text) => {
                self.output.push_str(text);
            }
            Node::Interpolation { expr, raw } => {
                let ctx = EvalCtx::new(&self.scope).with_debug(self.debug);
                let value = eval_expr(expr, &ctx)?;
                let str_value = value.coerce_str();
                if *raw {
                    self.output.push_str(&str_value);
                } else {
                    self.output.push_escaped(&str_value);
                }
            }
            Node::Directive {
                name, args, body, ..
            } => {
                self.render_directive(name, args.as_ref(), body.as_deref())?;
            }
            Node::Component {
                path,
                props,
                fills,
                main,
                ..
            } => {
                self.render_component(path, props.as_ref(), fills, main)?;
            }
            Node::Slot {
                name,
                default,
                scoped,
                ..
            } => {
                self.render_slot(name.as_deref(), default, scoped.as_ref())?;
            }
            Node::Section { name, body } => {
                self.section_map.insert(name.to_string(), body.clone());
            }
            Node::Yield { name, .. } => {
                if let Some(nodes) = self.section_map.get(name) {
                    let nodes_vec = nodes.to_vec();
                    self.render_nodes(&nodes_vec)?;
                }
            }
            Node::Extends { path, .. } => {
                if let Some(layout) = self.templates.get(path) {
                    let mut layout_renderer = Renderer::new(&layout, self.helpers, self.templates);
                    layout_renderer.section_map = std::mem::take(&mut self.section_map);
                    layout_renderer.scope = self.scope.clone();
                    layout_renderer.context_chain = self.context_chain.clone();
                    let output = layout_renderer.render()?;
                    self.output.push_str(&output);
                }
            }
            Node::Include { path, data, .. } => {
                if let Some(template) = self.templates.get(path) {
                    self.scope.push_frame();
                    if let Some(expr) = data {
                        let ctx = EvalCtx::new(&self.scope);
                        if let Value::Object(map) = eval_expr(expr, &ctx)? {
                            for (k, v) in map {
                                self.scope.declare(&k, v);
                            }
                        }
                    }
                    let mut include_renderer =
                        Renderer::new(&template, self.helpers, self.templates);
                    include_renderer.scope = self.scope.clone();
                    include_renderer.max_loop_iter = self.max_loop_iter;
                    include_renderer.debug = self.debug;
                    let output = include_renderer.render()?;
                    self.output.push_str(&output);
                    self.scope.pop_frame();
                }
            }
            Node::Push {
                stack,
                prepend,
                body,
            } => {
                let temp_template = Template {
                    nodes: body.clone(),
                    source_map: self.template.source_map.clone(),
                    file: self.template.file.clone(),
                    extends: None,
                    prop_decls: vec![],
                    slot_decls: vec![],
                };
                let mut temp_renderer = Renderer::new(&temp_template, self.helpers, self.templates);
                temp_renderer.scope = self.scope.clone();
                let output = temp_renderer.render()?;
                if *prepend {
                    self.stack_buffer.prepend(stack, output);
                } else {
                    self.stack_buffer.push(stack, output);
                }
            }
            Node::Stack { name, .. } => {
                let buffer = self.stack_buffer.drain(name);
                if !buffer.is_empty() {
                    self.output.push_str(&buffer);
                }
            }
            Node::RawTransform { content, transform } => {
                self.output.push_str(content);
            }
        }
        Ok(())
    }

    fn render_directive(
        &mut self,
        name: &str,
        args: Option<&crate::parser::arg_list::ArgList>,
        body: Option<&[Node]>,
    ) -> Result<(), RenderError> {
        eprintln!("DEBUG render_directive: name = {}", name);
        match name {
            "if" => self.render_if(args, body),
            "unless" => self.render_unless(args, body),
            "elseif" | "elsif" => Ok(()),
            "else" => Ok(()),
            "switch" => self.render_switch(args, body),
            "case" => Ok(()),
            "default" => Ok(()),
            "each" => self.render_each(args, body),
            "for" => self.render_for(args, body),
            "while" => self.render_while(args, body),
            "let" => self.render_let(args),
            "set" => self.render_set(args),
            "const" => self.render_const(args),
            "include" => self.render_include(args),
            "dump" => self.render_dump(args),
            "log" => self.render_log(args),
            "once" => self.render_once(body),
            "raw" => self.render_raw(body),
            "provide" => self.render_provide(args),
            "inject" => self.render_inject(args),
            _ => Ok(()),
        }
    }

    fn render_if(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
        body: Option<&[Node]>,
    ) -> Result<(), RenderError> {
        eprintln!("DEBUG render_if: args = {:?}", args.is_some());
        eprintln!("DEBUG render_if: body = {:?}", body.is_some());

        let should_render = if let Some(args) = args {
            eprintln!(
                "DEBUG render_if: has args, positional len = {}",
                args.positional.len()
            );
            if let Some(expr) = args.positional.first() {
                let ctx = EvalCtx::new(&self.scope);
                match eval_expr(expr, &ctx) {
                    Ok(value) => value.is_truthy(),
                    Err(_) => false,
                }
            } else {
                false
            }
        } else {
            false
        };

        eprintln!("DEBUG render_if: should_render = {}", should_render);

        if should_render {
            if let Some(body) = body {
                self.scope.push_frame();
                self.render_nodes(body)?;
                self.scope.pop_frame();
            }
        }
        Ok(())
    }

    fn render_unless(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
        body: Option<&[Node]>,
    ) -> Result<(), RenderError> {
        if let Some(args) = args {
            let ctx = EvalCtx::new(&self.scope);
            if let Some(expr) = args.positional.first() {
                let value = eval_expr(expr, &ctx)?;
                if !value.is_truthy() {
                    if let Some(body) = body {
                        self.scope.push_frame();
                        self.render_nodes(body)?;
                        self.scope.pop_frame();
                    }
                }
            }
        }
        Ok(())
    }

    fn render_switch(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
        body: Option<&[Node]>,
    ) -> Result<(), RenderError> {
        let switch_value = if let Some(args) = args {
            let ctx = EvalCtx::new(&self.scope);
            args.positional
                .first()
                .and_then(|e| eval_expr(e, &ctx).ok())
        } else {
            None
        };

        if let Some(body) = body {
            let mut found_case = false;
            let mut render_default = false;

            for node in body {
                if let Node::Directive {
                    name, args, body, ..
                } = node
                {
                    match name.as_str() {
                        "case" => {
                            if found_case {
                                break;
                            }
                            if let Some(case_args) = args {
                                let ctx = EvalCtx::new(&self.scope);
                                if let Some(case_expr) = case_args.positional.first() {
                                    if let Ok(case_value) = eval_expr(case_expr, &ctx) {
                                        if let Some(sw_val) = &switch_value {
                                            if sw_val == &case_value {
                                                found_case = true;
                                                if let Some(case_body) = body {
                                                    self.scope.push_frame();
                                                    self.render_nodes(case_body)?;
                                                    self.scope.pop_frame();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        "default" => {
                            if !found_case {
                                render_default = true;
                                if let Some(def_body) = body {
                                    self.scope.push_frame();
                                    self.render_nodes(def_body)?;
                                    self.scope.pop_frame();
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn render_each(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
        body: Option<&[Node]>,
    ) -> Result<(), RenderError> {
        if let Some(args) = args {
            let collection_expr = &args.positional[0];
            let ctx = EvalCtx::new(&self.scope);
            let collection = eval_expr(collection_expr, &ctx)?;

            let item_name = if args.positional.len() >= 2 {
                if let Expr::Ident(_, name) = &args.positional[1] {
                    name.clone()
                } else {
                    "it".to_string()
                }
            } else {
                "it".to_string()
            };

            let array = match &collection {
                Value::Array(arr) => arr.clone(),
                _ => vec![collection.clone()],
            };

            if array.is_empty() {
                return Ok(());
            }

            for (i, item) in array.iter().enumerate() {
                self.scope.push_frame();

                self.scope.declare(&item_name, item.clone());
                self.scope.declare("$index", Value::Num(i as f64));
                self.scope.declare("$first", Value::Bool(i == 0));
                self.scope
                    .declare("$last", Value::Bool(i == array.len() - 1));
                self.scope.declare("$even", Value::Bool(i % 2 == 0));
                self.scope.declare("$odd", Value::Bool(i % 2 == 1));
                self.scope.declare("$total", Value::Num(array.len() as f64));

                if let Some(body) = body {
                    self.render_nodes(body)?;
                }

                self.scope.pop_frame();
            }
        }
        Ok(())
    }

    fn render_for(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
        body: Option<&[Node]>,
    ) -> Result<(), RenderError> {
        Ok(())
    }

    fn render_while(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
        body: Option<&[Node]>,
    ) -> Result<(), RenderError> {
        if let Some(args) = args {
            let mut iter_count = 0;
            while iter_count < self.max_loop_iter {
                let ctx = EvalCtx::new(&self.scope);
                if let Some(expr) = args.positional.first() {
                    let value = eval_expr(expr, &ctx)?;
                    if !value.is_truthy() {
                        break;
                    }
                    if let Some(body) = body {
                        self.scope.push_frame();
                        self.render_nodes(body)?;
                        self.scope.pop_frame();
                    }
                }
                iter_count += 1;
            }
        }
        Ok(())
    }

    fn render_let(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
    ) -> Result<(), RenderError> {
        if let Some(args) = args {
            if args.positional.len() >= 2 {
                let name = &args.positional[0];
                let value_expr = &args.positional[1];

                if let Expr::Ident(_, var_name) = name {
                    let ctx = EvalCtx::new(&self.scope);
                    let value = eval_expr(value_expr, &ctx)?;
                    self.scope.declare(var_name, value);
                }
            }
        }
        Ok(())
    }

    fn render_set(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
    ) -> Result<(), RenderError> {
        if let Some(args) = args {
            if args.positional.len() >= 2 {
                let name = &args.positional[0];
                let value_expr = &args.positional[1];

                if let Expr::Ident(_, var_name) = name {
                    let ctx = EvalCtx::new(&self.scope);
                    let value = eval_expr(value_expr, &ctx)?;
                    self.scope.assign(var_name, value);
                }
            }
        }
        Ok(())
    }

    fn render_const(
        &mut self,
        _args: Option<&crate::parser::arg_list::ArgList>,
    ) -> Result<(), RenderError> {
        Ok(())
    }

    fn render_include(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
    ) -> Result<(), RenderError> {
        Ok(())
    }

    fn render_dump(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
    ) -> Result<(), RenderError> {
        if self.debug {
            if let Some(args) = args {
                let ctx = EvalCtx::new(&self.scope);
                if let Some(expr) = args.positional.first() {
                    if let Ok(value) = eval_expr(expr, &ctx) {
                        eprintln!("[DUMP] {:?}", value);
                    }
                }
            }
        }
        Ok(())
    }

    fn render_log(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
    ) -> Result<(), RenderError> {
        if let Some(args) = args {
            let ctx = EvalCtx::new(&self.scope);
            if let Some(expr) = args.positional.first() {
                if let Ok(value) = eval_expr(expr, &ctx) {
                    eprintln!("[LOG] {}", value.coerce_str());
                }
            }
        }
        Ok(())
    }

    fn render_once(&mut self, body: Option<&[Node]>) -> Result<(), RenderError> {
        Ok(())
    }

    fn render_raw(&mut self, body: Option<&[Node]>) -> Result<(), RenderError> {
        if let Some(body) = body {
            for node in body {
                if let Node::Text(text) = node {
                    self.output.push_str(text);
                }
            }
        }
        Ok(())
    }

    fn render_provide(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
    ) -> Result<(), RenderError> {
        Ok(())
    }

    fn render_inject(
        &mut self,
        args: Option<&crate::parser::arg_list::ArgList>,
    ) -> Result<(), RenderError> {
        Ok(())
    }

    fn render_component(
        &mut self,
        path: &str,
        props: Option<&crate::parser::arg_list::ArgList>,
        fills: &HashMap<String, Vec<Node>>,
        main: &[Node],
    ) -> Result<(), RenderError> {
        if let Some(template) = self.templates.get(path) {
            let mut comp_scope = self.scope.clone();
            comp_scope.push_frame();

            if let Some(props) = props {
                let ctx = EvalCtx::new(&self.scope);
                for arg in &props.positional {
                    if let Ok(value) = eval_expr(arg, &ctx) {
                        comp_scope.declare("$attrs", value);
                    }
                }
            }

            let mut resolver = SlotResolver::new();
            for (name, nodes) in fills {
                resolver.add_fill(name, nodes.clone());
            }
            resolver.add_fill("main", main.to_vec());

            let mut comp_renderer = Renderer::new(&template, self.helpers, self.templates);
            comp_renderer.scope = comp_scope;
            comp_renderer.slot_resolver = resolver;
            comp_renderer.max_loop_iter = self.max_loop_iter;
            comp_renderer.debug = self.debug;

            let output = comp_renderer.render()?;
            self.output.push_str(&output);
        }
        Ok(())
    }

    fn render_slot(
        &mut self,
        name: Option<&str>,
        default: &[Node],
        scoped: Option<&(String, Expr)>,
    ) -> Result<(), RenderError> {
        let slot_name = name.unwrap_or("main");

        if let Some(fill_nodes) = self.slot_resolver.get_fill(slot_name).cloned() {
            self.render_nodes(&fill_nodes)?;
        } else if !default.is_empty() {
            self.render_nodes(default)?;
        }
        Ok(())
    }
}
