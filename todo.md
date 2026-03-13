# Atom Engine v4 — Implementation Todo

> Rust crate: `atom-engine`. Pure template engine — no tiers, no framework opinions.  
> Work top-to-bottom. Each task is discrete and testable. Check the box when done.

---

## Phase 1 — Project Bootstrap

### 1.1 Cargo workspace

- [ ] `cargo new atom-engine --lib` — library crate, not binary
- [ ] Set `edition = "2021"` in `Cargo.toml`
- [ ] Add all `[dependencies]`: `anyhow`, `thiserror`, `indexmap`, `heck`, `chrono`, `serde`, `serde_json`, `unicode-segmentation`, `regex`, `base64`, `sha2`, `md-5`, `uuid`, `strsim`
- [ ] Add `rayon` as optional under `[features] parallel`
- [ ] Add `[features] html = []` for opt-in HTML escaping
- [ ] Add `[dev-dependencies]`: `insta`, `tempfile`
- [ ] Run `cargo build` — confirm zero errors

### 1.2 Module skeleton

- [ ] Create `src/parser/mod.rs` — declare submodules `lexer`, `parser`, `expr_parser`, `arg_list`, `source_map`
- [ ] Create `src/directives/mod.rs` — declare `Directive` trait stub + `DirectiveRegistry` stub + `builtin/` submodule
- [ ] Create `src/directives/builtin/mod.rs` — declare submodules `control`, `loops`, `variables`, `includes`, `layout`, `components`, `stacks`, `utils`
- [ ] Create `src/renderer/mod.rs` — declare submodules `scope`, `context_chain`, `stack_buffer`, `slot_resolver`, `section_map`, `component_renderer`, `layout_renderer`, `output`, `expr_eval`, `helper_eval`
- [ ] Create `src/helpers/mod.rs` — declare submodules `strings`, `collections`, `numbers`, `dates`, `types`, `encoding`
- [ ] Create `src/types/mod.rs` — declare submodules `value`, `coerce`, `props`
- [ ] Create `src/error.rs` — empty stubs for `ParseError`, `RenderError`
- [ ] Create `src/lib.rs` — re-export `Engine`, `Value`, `ParseError`, `RenderError`; declare all top-level modules
- [ ] Run `cargo check` — all modules resolve, zero errors

---

## Phase 2 — Types and Value System

### 2.1 `Value` enum (`src/types/value.rs`)

- [ ] Define `Value` enum: `Null`, `Bool(bool)`, `Num(f64)`, `Str(String)`, `Array(Vec<Value>)`, `Object(IndexMap<String, Value>)`, `Date(chrono::DateTime<Utc>)`
- [ ] Derive `Clone`, `Debug`, `PartialEq`
- [ ] Implement `Display` for `Value` — each variant formats as a human-readable string
- [ ] Implement `From<bool>`, `From<f64>`, `From<i64>`, `From<i32>`, `From<usize>`, `From<String>`, `From<&str>` for `Value`
- [ ] Implement `From<Vec<Value>>` and `From<IndexMap<String, Value>>` for `Value`
- [ ] Implement `Value::is_truthy(&self) -> bool` — null/false/0/""/empty-array/empty-object → false, everything else → true
- [ ] Implement `Value::type_name(&self) -> &'static str` → "null" | "bool" | "number" | "string" | "array" | "object" | "date"
- [ ] Implement `serde_json::Value` → `Value` conversion
- [ ] Implement `Value` → `serde_json::Value` conversion
- [ ] Unit tests: truthiness of each variant, type names, Display output, round-trip JSON conversion

### 2.2 Coercion (`src/types/coerce.rs`)

- [ ] Define trait `IntoValue` with method `into_value(self) -> Value`
- [ ] Implement `IntoValue` for all primitive Rust types
- [ ] Define `Value::as_str`, `as_f64`, `as_bool`, `as_array`, `as_object` — return `Option<T>`
- [ ] Define `Value::coerce_str(&self) -> String` — never fails, converts any variant to string
- [ ] Unit tests for each coercion path including edge cases

### 2.3 Props struct (`src/types/props.rs`)

- [ ] Define `PropType` enum: `String`, `Number`, `Boolean`, `Array`, `Object`, `Function`, `Any`
- [ ] Define `PropDecl { name: String, ty: PropType, default: Option<Value>, optional: bool }`
- [ ] Define `Props { declared: IndexMap<String, PropDecl>, provided: IndexMap<String, Value>, extra: IndexMap<String, Value> }`
- [ ] Implement `Props::validate(&self) -> Result<(), RenderError>` — checks required props present, types match
- [ ] Implement `Props::get`, `Props::has`, `Props::only`, `Props::except`, `Props::merge`, `Props::to_attrs`
- [ ] Unit tests: validation errors, optional defaults, `to_attrs()` output

---

## Phase 3 — Lexer

### 3.1 Token types and Lexer (`src/parser/lexer.rs`)

- [ ] Define `Span { start: usize, end: usize, line: u32, col: u32 }`
- [ ] Define `Token` enum with all variants: `Text`, `OpenInterp`, `OpenRawInterp`, `CloseInterp`, `CloseRawInterp`, `OpenComment`, `CloseComment`, `At`, `Bang`, `Ident`, `LParen`, `RParen`, `LBrace`, `RBrace`, `LBracket`, `RBracket`, `Comma`, `Dot`, `Colon`, `Equals`, `StringLit`, `NumLit`, `BoolLit`, `NullLit`, `Whitespace`, `Newline`, `Eof`
- [ ] Each `Token` variant carries a `Span`
- [ ] Implement `Lexer::new(src: &str) -> Self`
- [ ] Implement `Lexer::next_token(&mut self) -> Token`
- [ ] Implement `Lexer::tokenise(src: &str) -> Vec<Token>` — drives to completion
- [ ] Handle `{{--` before `{{` to avoid mis-tokenising comments as interpolations
- [ ] Handle `{{{` before `{{` for raw interpolation
- [ ] Handle `@!` → `At` then `Bang`
- [ ] Handle string literals with `"` and `'` delimiters, with escape sequences
- [ ] Track line/col correctly through newlines and multi-line strings
- [ ] Unit tests: each token type, comment boundaries, `@!`, string escapes, number formats

### 3.2 Source map (`src/parser/source_map.rs`)

- [ ] Define `SourceMap { file: String, spans: Vec<(usize, Span)> }`
- [ ] Implement `SourceMap::record`, `SourceMap::get`
- [ ] Unit test: record three spans, retrieve by index

---

## Phase 4 — Expression Parser

### 4.1 Expression AST (`src/parser/expr_parser.rs`)

- [ ] Define `Expr` enum: `Null`, `Bool`, `Num`, `Str`, `Ident`, `Prop`, `Index`, `OptChain`, `Call`, `Unary`, `Binary`, `Ternary`, `NullCoalesce`, `Array`, `Object`, `TemplateLit`
- [ ] Define `UnaryOp`: `Not`, `Neg`
- [ ] Define `BinaryOp`: `Add`, `Sub`, `Mul`, `Div`, `Mod`, `Pow`, `Eq`, `Ne`, `Lt`, `Le`, `Gt`, `Ge`, `And`, `Or`
- [ ] Define `TemplateSegment`: `Text(String)`, `Expr(Expr)`

### 4.2 Expression parser

- [ ] Implement Pratt/precedence-climbing parser for all precedence levels
- [ ] Parse primaries: null, bool, num, string, ident, `(expr)`, `[array]`, `{object}`, `` `template` ``
- [ ] Parse property access, optional chaining, index access, method/function calls
- [ ] Correct operator precedence (ternary lowest → exponent → unary → call/access highest)
- [ ] Error with span info on unknown operators or malformed expressions

### 4.3 ArgList parser (`src/parser/arg_list.rs`)

- [ ] Define `ArgList { positional: Vec<Expr>, named: IndexMap<String, Expr> }`
- [ ] Implement `ArgList::parse` — parses `(expr, key: expr, ...)` between parens
- [ ] Implement `required_expr`, `optional_expr`, `required_string`, `optional_string`, `first`
- [ ] Unit tests: all arg forms, missing required → error

---

## Phase 5 — Parser (Token → AST)

### 5.1 Node types (`src/parser/parser.rs`)

- [ ] Define `Node` enum: `Text`, `Interpolation { expr, raw }`, `Directive { name, args, body, span }`, `Component { path, props, fills, main, span }`, `Slot { name, default, scoped, span }`, `Extends { path, span }`, `Section { name, body }`, `Yield { name, span }`, `Include { path, data, span }`, `Push { stack, prepend, body }`, `Stack { name, span }`, `RawTransform { content, transform }`
- [ ] Define `Template { nodes: Vec<Node>, source_map: SourceMap, file: String, extends: Option<String>, prop_decls: Vec<PropDecl>, slot_decls: Vec<SlotDecl> }`

### 5.2 Parser struct and main loop

- [ ] Implement `Parser::parse(src, file, directives) -> Result<Template, ParseError>`
- [ ] Main loop: dispatch on `Text`, `{{`, `{{{`, `{{--`, `@`
- [ ] Parse `{{ expr }}` → `Node::Interpolation { raw: false }`
- [ ] Parse `{{{ expr }}}` → `Node::Interpolation { raw: true }`
- [ ] Parse `{{-- ... --}}` → skip tokens until `--}}`
- [ ] Directive dispatch: consume `@`, read `Ident`, look up in registry, call `compile()`
- [ ] Self-closing dispatch: `@!` → read `Ident`, call with `body: None`
- [ ] Block body parsing: collect nodes recursively until matching `}`; track brace depth
- [ ] Parse `@elseif` / `@else` / `@empty` as continuations after closing `}`
- [ ] Error on unmatched `}`
- [ ] Error on unknown directive — include fuzzy suggestion via `strsim`

### 5.3 Built-in directive `compile()` implementations

#### `src/directives/builtin/control.rs`

- [ ] `IfDirective` — block, seekable; parses condition expr; looks ahead for `@elseif`/`@else` chains
- [ ] `UnlessDirective` — block, seekable; wraps body in negated if
- [ ] `SwitchDirective` — block, seekable; body must contain only `@case`/`@default` blocks
- [ ] `CaseDirective` — block, seekable; only valid inside `@switch`
- [ ] `DefaultDirective` — block, not-seekable; only valid as last block in `@switch`

#### `src/directives/builtin/loops.rs`

- [ ] `EachDirective` — block, seekable; parses `item in collection` with destructuring support (`[k, v]`, `{ name, email }`); looks ahead for `@empty`
- [ ] `ForDirective` — block, seekable; parses `i from start to end` + optional `step n`
- [ ] `WhileDirective` — block, seekable; parses condition expr

#### `src/directives/builtin/variables.rs`

- [ ] `LetDirective` — not-block, seekable; parses `name = expr`
- [ ] `SetDirective` — not-block, seekable; same shape, flagged as mutation
- [ ] `ConstDirective` — not-block, seekable; same shape, flagged as const

#### `src/directives/builtin/includes.rs`

- [ ] `IncludeDirective` — not-block, seekable; parses `'path'` + optional data expr
- [ ] `IncludeWhenDirective` — not-block, seekable; parses `condition, 'path'` + optional data
- [ ] `IncludeUnlessDirective` — not-block, seekable; parses `condition, 'path'` + optional data
- [ ] `IncludeFirstDirective` — not-block, seekable; parses array of path strings
- [ ] `IncludeEachDirective` — not-block, seekable; parses `'path', collection` + optional `as: name`

#### `src/directives/builtin/layout.rs`

- [ ] `ExtendsDirective` — not-block, seekable; parses `'path'`; errors if not first node
- [ ] `SectionDirective` — block, seekable; parses name; only valid in template with `@extends`
- [ ] `YieldDirective` — not-block, seekable; parses name

#### `src/directives/builtin/components.rs`

- [ ] `UseDirective` — block, seekable; parses `'path'` + optional props; scans body for `@fill` blocks; remainder goes to main slot
- [ ] `SlotDirective` — block; parses optional name + optional scoped data expr; default = block body
- [ ] `FillDirective` — block, seekable; parses name + optional `as: binding`; error if outside `@use` body
- [ ] `ProvideDirective` — not-block, seekable; parses `key, expr`
- [ ] `InjectDirective` — not-block, seekable; parses key

#### `src/directives/builtin/stacks.rs`

- [ ] `PushDirective` — block, seekable; parses stack name; prepend = false
- [ ] `PrependDirective` — block, seekable; parses stack name; prepend = true
- [ ] `StackDirective` — not-block, seekable; parses stack name

#### `src/directives/builtin/utils.rs`

- [ ] `DumpDirective` — not-block, seekable; parses expr
- [ ] `LogDirective` — not-block, seekable; parses expr
- [ ] `CommentDirective` — block, not-seekable; compile body to nothing (drop all nodes)
- [ ] `RawDirective` — block, not-seekable; body tokens reduced to `Node::Text`, no further parsing
- [ ] `OnceDirective` — block, not-seekable; wraps body in `Node::Directive { name: "once" }`
- [ ] `PropsDirective` — not-block, seekable; parses prop declaration object; stored on `Template`, not emitted as a node
- [ ] `SlotsDirective` — not-block, seekable; parses array of slot names; stored on `Template`
- [ ] `MetaDirective` — not-block, seekable; parses key-value object; stored on `Template`

### 5.4 `DirectiveRegistry` and `CompileContext`

- [ ] Implement `DirectiveRegistry::new()` — registers all built-ins
- [ ] Implement `DirectiveRegistry::register`, `get`, `names`
- [ ] Define `CompileContext<'r>` with `directives`, `file`
- [ ] Implement `CompileContext::nodes_to_raw_text` — extracts text, errors on non-text nodes

---

## Phase 6 — Error Types

### 6.1 `ParseError` (`src/error.rs`)

- [ ] Define `ParseError { file, span, kind: ParseErrorKind, hint }`
- [ ] All `ParseErrorKind` variants: `UnknownDirective { name, similar }`, `UnclosedBlock { directive, opened_at }`, `UnexpectedToken { expected, found }`, `ElseifWithoutIf`, `ElseWithoutIf`, `FillOutsideComponent`, `SlotOutsideComponent`, `SetTargetingProp { name }`, `ConstReassignment { name }`, `DuplicatePropDeclaration { name }`, `MissingRequiredArg { directive, arg }`, `ExprError { message }`, `InvalidSlotName { name }`, `MultipleExtends`, `ExtendsNotFirst`
- [ ] Implement `Display` — Rust-compiler-style: error code, file:line:col, source snippet with caret, optional hint
- [ ] Implement `std::error::Error` and `From<ParseError> for anyhow::Error`

### 6.2 `RenderError` (`src/error.rs`)

- [ ] Define `CallFrame { file, span, note }`
- [ ] Define `RenderError { file, span, kind: RenderErrorKind, trace: Vec<CallFrame>, hint }`
- [ ] All `RenderErrorKind` variants: `MissingRequiredProp { name, template }`, `PropTypeMismatch { name, expected, found }`, `UndefinedVariable { name }`, `SetUndeclaredVariable { name }`, `UnknownTemplate { path, disk }`, `MaxLoopIterationsExceeded { limit }`, `HelperError { name, message }`, `TransformError { name, message }`, `DivisionByZero`, `InjectWithoutProvide { key }`, `RequiredSlotUnfilled { name, component }`, `CircularInclude { a, b }`
- [ ] Implement `Display` — error kind + component call trace with indented stack frames
- [ ] Implement `std::error::Error`

---

## Phase 7 — Renderer Infrastructure

### 7.1 Output buffer (`src/renderer/output.rs`)

- [ ] Define `struct OutputBuffer(String)`
- [ ] Implement `new`, `push(&str)`, `push_char(char)`, `finish() -> String`
- [ ] Implement `OutputBuffer::escape_html(s: &str) -> String` — escapes `&`, `<`, `>`, `"`, `'`

### 7.2 Scope (`src/renderer/scope.rs`)

- [ ] Define `Frame { vars: IndexMap<String, Value>, is_props: bool, is_const: HashSet<String> }`
- [ ] Define `Scope { frames: Vec<Frame> }`
- [ ] Implement `push_frame`, `pop_frame`, `push_props`
- [ ] Implement `declare` — error if name exists in current frame
- [ ] Implement `declare_const` — same + marks name as const
- [ ] Implement `assign` — walks inner→outer, errors if not found, errors if props or const frame
- [ ] Implement `get` — walks inner→outer, first match wins
- [ ] Implement `inject_loop_vars(index, total)` — injects `$index`, `$number`, `$first`, `$last`, `$even`, `$odd`, `$total`, `$key`
- [ ] Unit tests: declare/assign/shadow, props immutability, const reassignment, loop vars

### 7.3 Context chain (`src/renderer/context_chain.rs`)

- [ ] Define `ContextChain { layers: Vec<IndexMap<String, Value>> }`
- [ ] Implement `push_layer`, `pop_layer`, `provide`, `inject`
- [ ] Unit tests: provide in parent, inject in child, unknown key returns None

### 7.4 Stack buffer (`src/renderer/stack_buffer.rs`)

- [ ] Define `StackBuffer { stacks: IndexMap<String, Vec<String>> }`
- [ ] Implement `push`, `prepend`, `drain`, `peek`
- [ ] Unit tests: push/drain order, prepend goes first, drain empties stack

### 7.5 Slot resolver (`src/renderer/slot_resolver.rs`)

- [ ] Define `SlotResolver { named: HashMap<String, Vec<Node>>, main: Vec<Node> }`
- [ ] Implement `from_component_body(nodes)` — splits `@fill` blocks from main content
- [ ] Implement `resolve(name: Option<&str>) -> &[Node]`

### 7.6 Section map (`src/renderer/section_map.rs`)

- [ ] Define `SectionMap { sections: HashMap<String, Vec<Node>> }`
- [ ] Implement `from_template(nodes)` — collects all `Node::Section`
- [ ] Implement `get(name) -> Option<&[Node]>`

---

## Phase 8 — Expression Evaluator

### 8.1 Core evaluator (`src/renderer/expr_eval.rs`)

- [ ] Define `EvalCtx<'r>` holding scope, context_chain, helpers, config refs
- [ ] Implement `eval(expr: &Expr, ctx: &EvalCtx) -> Result<Value, RenderError>` for every `Expr` variant:
  - `Null`, `Bool`, `Num`, `Str` → direct conversion
  - `Ident` → `scope.get()`, error if not found
  - `Prop` → eval object, index into `Object` or use property shims
  - `OptChain` → eval object; return `Null` if object is `Null`, else access property
  - `Index` → eval both; array numeric index; object string key; out-of-bounds → `Null`
  - `Unary Not` → negate truthiness; `Neg` → negate number
  - `Binary` → eval both; apply operator; `+` concatenates strings; div-by-zero error
  - `NullCoalesce` → eval left; if not Null return it; else eval right
  - `Ternary` → eval condition; eval only the matching branch
  - `Array` → eval each element
  - `Object` → eval each value
  - `Call` → dispatch to helper_eval
  - `TemplateLit` → eval each segment, concatenate
- [ ] Unit tests for every branch including edge cases

### 8.2 Helper evaluator (`src/renderer/helper_eval.rs`)

- [ ] Define `trait HelperFn: Send + Sync { fn call(&self, receiver: Option<Value>, args: Vec<Value>, ctx: &EvalCtx) -> Result<Value, RenderError> }`
- [ ] Define `HelperRegistry(HashMap<String, Box<dyn HelperFn>>)`
- [ ] Implement `HelperRegistry::new()` — registers all built-in helpers
- [ ] Implement `call_method(receiver, method, args, ctx)` — type-checks receiver, dispatches; error on unknown method
- [ ] Implement `call_global(path, args, ctx)` — dispatches `h.*` calls
- [ ] Dispatch logic in `eval()`: `Call { callee: Prop { object, key }, args }` → eval object as receiver → `call_method`; `Call { callee: Prop { Ident("h"), key }, args }` → `call_global`

---

## Phase 9 — Built-in Helpers

### 9.1 String helpers (`src/helpers/strings.rs`)

- [ ] `.upper()`, `.lower()`, `.trim()`, `.trimStart()`, `.trimEnd()`
- [ ] `.capitalize()` — first char upper, rest lower
- [ ] `.titleCase()` — each word capitalised
- [ ] `.camelCase()`, `.pascalCase()`, `.snakeCase()`, `.kebabCase()`, `.screamingSnake()` — via `heck`
- [ ] `.dotCase()`, `.sentenceCase()`
- [ ] `.replace(from, to)`, `.replaceFirst(from, to)`
- [ ] `.startsWith(s)`, `.endsWith(s)`, `.contains(s)` → `Value::Bool`
- [ ] `.padStart(n, char?)`, `.padEnd(n, char?)`
- [ ] `.repeat(n)`
- [ ] `.split(sep)` → `Value::Array`
- [ ] `.chars()`, `.lines()` → `Value::Array`
- [ ] `.len()` — grapheme count via `unicode-segmentation`
- [ ] `.isEmpty()` → `Value::Bool`
- [ ] `.truncate(n, suffix?)` — at word boundary
- [ ] `.excerpt(n)` — strip HTML tags, then truncate
- [ ] `.indent(n, char?)`, `.dedent()`, `.wrap(n)`
- [ ] `h.pluralize(n, word, plural?)`, `h.singularize(word)`, `h.slugify(s)`

### 9.2 Encoding helpers (`src/helpers/encoding.rs`)

- [ ] `h.escape(s)`, `h.unescape(s)` — HTML entities
- [ ] `h.encodeUri(s)`, `h.decodeUri(s)` — percent-encoding
- [ ] `h.base64Encode(s)`, `h.base64Decode(s)` — via `base64` crate
- [ ] `h.sha256(s)` — hex-encoded SHA-256 via `sha2`
- [ ] `h.md5(s)` — hex-encoded MD5 via `md-5`
- [ ] `h.uuid()` — random UUID v4 via `uuid`
- [ ] `h.random(n)` — random alphanumeric string of length n
- [ ] Unit tests: known inputs, base64 round-trip, sha256 of empty string

### 9.3 Collection helpers (`src/helpers/collections.rs`)

Array receiver methods:

- [ ] `.len()`, `.isEmpty()`, `.first()`, `.last()`, `.first(n)`, `.last(n)`, `.nth(i)`, `.slice(start, end?)`
- [ ] `.take(n)`, `.skip(n)`
- [ ] `.includes(val)`, `.indexOf(val)`
- [ ] `.push(val)`, `.prepend(val)`, `.concat(arr)` — immutable, return new array
- [ ] `.flatten()`, `.unique()`, `.uniqueBy(key)`, `.compact()`
- [ ] `.sort()`, `.sortBy(key, dir?)`, `.reverse()`, `.shuffle()`
- [ ] `.zip(arr)`, `.chunk(n)`
- [ ] `.groupBy(key)`, `.countBy(key)`, `.keyBy(key)`
- [ ] `.join(sep)`, `.joinWith(sep, lastSep?)`
- [ ] `.sum()`, `.sum(key)`, `.min()`, `.min(key)`, `.max()`, `.max(key)`, `.avg()`, `.avg(key)`
- [ ] `.pluck(key)`, `.filter(key, val?)`, `.reject(key, val?)`, `.where(key, op, val)`, `.find(key, val)`, `.findIndex(key, val)`
- [ ] `.map(key)` — alias for `.pluck(key)` when arg is a string
- [ ] `.pick(keys)`, `.omit(keys)` — applied to each object in array
- [ ] `h.range(start, end, step?)`, `h.zip(a, b)`
- [ ] Unit tests: chunk, groupBy, zip, sortBy, joinWith, where, sum/min/max on empty array

Object receiver methods:

- [ ] `.keys()`, `.values()`, `.entries()`, `.has(key)`, `.get(key, default?)`, `.set(key, val)`
- [ ] `.merge(obj)`, `.deepMerge(obj)`, `.pick(keys)`, `.omit(keys)`, `.len()`, `.isEmpty()`
- [ ] `h.fromEntries(pairs)` — build object from `[key, value]` pairs

### 9.4 Number helpers (`src/helpers/numbers.rs`)

- [ ] `.round(n?)`, `.floor()`, `.ceil()`, `.abs()`, `.clamp(min, max)`
- [ ] `.toFixed(n)` → `Value::Str`
- [ ] `.format(pattern?)` → thousand-separated string
- [ ] `.currency(symbol?)`, `.percent(decimals?)`
- [ ] `.isEven()`, `.isOdd()` → `Value::Bool`
- [ ] `.pad(n, char?)` → zero-padded string
- [ ] `h.min(a, b)`, `h.max(a, b)`, `h.random(min, max)`

### 9.5 Date helpers (`src/helpers/dates.rs`)

- [ ] `.format(pattern)` — tokens: `YYYY`, `MM`, `DD`, `HH`, `mm`, `ss`, `MMMM`, `MMM`, `D`
- [ ] `.toRelative()` — "3 days ago", "in 2 hours", "just now"
- [ ] `.toIso()`, `.toUnix()`
- [ ] `.add(n, unit)`, `.sub(n, unit)` — units: years, months, weeks, days, hours, minutes, seconds
- [ ] `.startOf(unit)`, `.endOf(unit)`
- [ ] `.isBefore(d)`, `.isAfter(d)`, `.isSame(d, unit?)`, `.diff(d, unit)`
- [ ] `h.now()`, `h.today()`, `h.parseDate(s, fmt?)`

### 9.6 Type helpers (`src/helpers/types.rs`)

- [ ] `h.isDefined`, `h.isNull`, `h.isEmpty`, `h.isString`, `h.isNumber`, `h.isBool`, `h.isArray`, `h.isObject`
- [ ] `h.default(x, fallback)`, `h.coalesce(a, b, c, ...)`
- [ ] `h.str`, `h.int`, `h.float`, `h.bool`
- [ ] `h.json(x)`, `h.parseJson(s)`, `h.inspect(x)`, `h.typeof(x)`

---

## Phase 10 — Renderer

### 10.1 Renderer entry point (`src/renderer/mod.rs`)

- [ ] Define `RenderCtx` holding scope, context_chain, stack_buffer, config, registries, `once_set: HashSet<u64>`, `include_stack: Vec<String>`, `call_trace: Vec<CallFrame>`
- [ ] Implement `render(template, data, ctx) -> Result<String, RenderError>`
- [ ] Dispatch on each `Node` variant in the main render loop:
  - `Text` → push to output
  - `Interpolation` → eval, coerce to string, optionally HTML-escape, push
  - `Directive` → `render_directive()`
  - `Component` → `component_renderer`
  - `Slot` → `slot_resolver`
  - `Extends` → `layout_renderer`
  - `Section` → store in `section_map` (not rendered here)
  - `Yield` → lookup in `section_map`, render section nodes
  - `Include` → check circular, push/pop include stack, recurse
  - `Push` → render body to temp buffer, push to `stack_buffer`
  - `Stack` → drain `stack_buffer`, push to output
  - `RawTransform` → lookup transform, apply, push result

### 10.2 Directive renderer

- [ ] `@if` — eval condition, push frame, render body, pop; chain else-if / else
- [ ] `@unless` — negate then same as `@if`
- [ ] `@switch`/`@case`/`@default` — eval switch expr, find matching case, render only that body
- [ ] `@each` — eval collection; iterate; push/pop frame per item; inject loop vars; handle `@empty`
- [ ] `@for` — eval start/end/step; iterate; inject `$index`, `$first`, `$last`; enforce `max_loop_iter`
- [ ] `@while` — eval condition before each iteration; enforce `max_loop_iter`
- [ ] `@let` → `scope.declare()`
- [ ] `@set` → `scope.assign()`
- [ ] `@const` → `scope.declare_const()`
- [ ] `@include` — resolve path, check circular, push frame with merged data, render, pop
- [ ] `@includeWhen`, `@includeUnless` — conditional include
- [ ] `@includeFirst` — try each path, render first found
- [ ] `@includeEach` — iterate collection, include partial per item
- [ ] `@provide` → `context_chain.provide()`
- [ ] `@inject` → `context_chain.inject()`, declare as let; error if not found
- [ ] `@dump` — if debug: pretty-print eval result; if not debug: noop
- [ ] `@log` — eval expr, write to stderr; never affects output
- [ ] `@once` — hash body nodes; check `once_set`; skip if seen; else render and record
- [ ] `@raw` — already `Node::Text` from parser; emit as-is
- [ ] `@push` / `@prepend` — render body to temp buffer, push to `stack_buffer`
- [ ] `@props` / `@slots` / `@meta` — noop at render time

### 10.3 Component renderer (`src/renderer/component_renderer.rs`)

- [ ] Resolve template path; error `UnknownTemplate` if not found
- [ ] Validate props against template's `@props` declarations
- [ ] Push call frame onto `ctx.call_trace`
- [ ] Build `SlotResolver` from fills and main content
- [ ] Push new scope frame, push props frame
- [ ] Render template nodes with `SlotResolver` on ctx
- [ ] Pop frames, pop call trace
- [ ] Integration test: required/optional props, `$attrs`, slot filling

### 10.4 Slot rendering

- [ ] When rendering `Node::Slot { name, default, scoped }`:
  - Look up name in `SlotResolver`
  - If fill provided: render fill nodes in **caller's** scope; if scoped, bind data expr result to binding name
  - If no fill: render default nodes in component scope
  - Error if slot is required (non-optional) and no fill provided

### 10.5 Layout renderer (`src/renderer/layout_renderer.rs`)

- [ ] On `@extends`: render child template first to collect all `@section` bodies into `SectionMap`
- [ ] Then render layout template; at each `@yield(name)`, render matching section nodes in child's scope
- [ ] No fill for a `@yield` → render nothing
- [ ] Integration test: layout with two yields, child fills both, verify correct output structure

### 10.6 Template registry

- [ ] Define `TemplateRegistry { disks: HashMap<String, HashMap<String, Arc<Template>>> }`
- [ ] Implement `add_disk` — walks dir for `.atom` files, parses each, stores as `disk::relative/path`
- [ ] Implement `add_embedded` — same from in-memory `(path, source)` pairs
- [ ] Implement `get(path)` — handles `disk::path` and plain path (uses default disk)
- [ ] Implement `resolve_component_tag(tag)` — converts `form.input` → looks up `components/form/input.atom`

---

## Phase 11 — Public Engine API

### 11.1 `Engine` struct (`src/lib.rs`)

- [ ] Define `Engine { registry, helpers, directives, transforms, config }`
- [ ] Define `EngineConfig { escape_html: bool, max_loop_iter: usize, debug: bool, strict_props: bool }` with `Default`
- [ ] Implement `Engine::new()`, `Engine::with_config(config)`
- [ ] Implement `add_disk`, `add_embedded`, `register_directive`, `register_helper`, `register_transform`
- [ ] Implement `render(path, data) -> Result<String, RenderError>`
- [ ] Implement `render_raw(source, data) -> Result<String, RenderError>` — parses in-place, not stored
- [ ] Implement `parse(source) -> Result<Template, ParseError>` — for validation/precompilation

### 11.2 `TransformRegistry`

- [ ] Define `trait TransformFn: Send + Sync { fn apply(&self, content: &str) -> Result<String, RenderError> }`
- [ ] Define `TransformRegistry(HashMap<String, Box<dyn TransformFn>>)`
- [ ] Implement `register`, `apply` — error `TransformError` if transform name not found

### 11.3 Thread-safety

- [ ] Verify `Engine: Send + Sync` — add `static_assertions` dev dep, write a compile-time test
- [ ] Verify `Template: Send + Sync`, `Value: Send + Sync`

---

## Phase 12 — Snapshot Tests

### 12.1 Test infrastructure

- [ ] Create `tests/common/mod.rs` — `engine()` helper builds test engine with `tests/fixtures/` disk mounted
- [ ] Create `tests/fixtures/` — all test `.atom` files
- [ ] Configure `insta` in `Cargo.toml`

### 12.2 Interpolation tests (`tests/interpolation.rs`)

- [ ] `{{ name }}` string value
- [ ] `{{ count }}` number value
- [ ] `{{ price * quantity }}` arithmetic expression
- [ ] `{{ name.upper() }}` method call
- [ ] `{{ user?.name }}` optional chain on null → empty string
- [ ] `{{ label ?? "default" }}` null-coalesce
- [ ] `{{ count > 0 ? "items" : "empty" }}` ternary
- [ ] `` {{ `hello ${name}` }} `` template literal
- [ ] `{{{ html }}}` in HTML mode — no escaping
- [ ] `{{-- text --}}` — nothing emitted

### 12.3 Control flow tests (`tests/control.rs`)

- [ ] `@if(true)` renders body; `@if(false)` renders nothing
- [ ] `@if` / `@elseif` / `@else` chain — only correct branch renders
- [ ] `@unless(false)` renders body
- [ ] Nested `@if` inside `@if`
- [ ] `@switch` with matching case renders only that case
- [ ] `@switch` with no match renders `@default`
- [ ] `@switch` with no match and no default renders nothing

### 12.4 Loop tests (`tests/loops.rs`)

- [ ] `@each` over array — correct elements in order
- [ ] `@each` with `$index`, `$first`, `$last`, `$even`, `$odd`, `$total`
- [ ] `@each {} @empty {}` — empty branch on empty array
- [ ] `@each` over object — `$key` available
- [ ] `@for(i from 1 to 4)` — renders 3 times
- [ ] `@for` with `step 2`
- [ ] Nested `@each` with outer variable captured via `@let`
- [ ] `@while` with counter — terminates correctly
- [ ] `max_loop_iter` exceeded → `MaxLoopIterationsExceeded` error

### 12.5 Variable tests (`tests/variables.rs`)

- [ ] `@let(x = 5)` then `{{ x }}` → "5"
- [ ] `@set(x = 10)` after `@let(x = 5)` → "10"
- [ ] `@const` then `@set` → `ConstReassignment` error
- [ ] `@set` targeting prop name → `SetTargetingProp` error
- [ ] `@let` duplicate in same scope → error
- [ ] `@let` shadowing in nested scope — allowed, outer unchanged

### 12.6 Component tests (`tests/components.rs`)

- [ ] Required prop — renders correctly
- [ ] Missing required prop → `MissingRequiredProp` error
- [ ] Optional prop uses default
- [ ] `$attrs` receives undeclared props
- [ ] Main slot: content between `@use` tags goes to main slot
- [ ] Named slot: `@fill(name)` fills `@slot(name)`
- [ ] Named slot with default: unfilled renders default
- [ ] Required slot not filled → error
- [ ] Scoped slot: component passes data, caller's `@fill(name, as: x)` accesses it
- [ ] Nested components — grandparent → parent → child all render
- [ ] Component tag resolution: `@button()` finds `components/button.atom`
- [ ] Disk-prefixed: `@ui.button()` finds `components/button.atom` on `ui` disk

### 12.7 Layout tests (`tests/layouts.rs`)

- [ ] `@extends` + `@section` — section injected at `@yield`
- [ ] Multiple sections each appear in correct position
- [ ] Unfilled `@yield` renders nothing
- [ ] `@extends` not first node → `ExtendsNotFirst` error
- [ ] Two `@extends` → `MultipleExtends` error

### 12.8 Include tests (`tests/includes.rs`)

- [ ] `@include('partial')` — content inlined
- [ ] `@include('partial', { key: val })` — extra data in scope
- [ ] `@includeWhen(false, 'partial')` — nothing rendered
- [ ] `@includeFirst(['missing', 'exists'])` — renders `exists`
- [ ] `@includeEach('row', items)` — renders once per item
- [ ] Circular include A → B → A → `CircularInclude` error

### 12.9 Stack tests (`tests/stacks.rs`)

- [ ] `@push` from partial, `@stack` in layout — content at stack location
- [ ] `@prepend` appears before regular pushes
- [ ] Multiple pushes in same stack — all collected in order
- [ ] `@stack` for empty stack — renders nothing

### 12.10 Utility tests (`tests/utils.rs`)

- [ ] `@once {}` inside `@each` — body renders only once total
- [ ] Two distinct `@once {}` blocks — both render
- [ ] `@dump` in debug mode — output contains value representation
- [ ] `@dump` in non-debug mode — nothing emitted
- [ ] `@log` writes to stderr, not stdout

### 12.11 Helper snapshot tests (`tests/helpers/`)

- [ ] One test file per helper group; 5–10 representative cases each
- [ ] String: `snakeCase`, `pascalCase`, `truncate`, `slugify`, `pluralize`, `base64Encode`/`Decode`, `sha256`
- [ ] Collection: `groupBy`, `sortBy`, `chunk`, `joinWith`, `where`, `pluck`, `unique`
- [ ] Number: `round`, `toFixed`, `currency`, `percent`, `clamp`, `format`
- [ ] Date: `format`, `toRelative`, `add`, `diff`, `startOf`
- [ ] Type: `isDefined`, `isEmpty`, `coalesce`, `json`/`parseJson` round-trip

### 12.12 Error display tests (`tests/errors.rs`)

- [ ] Each `ParseErrorKind` — display output matches expected format (error code, caret, hint)
- [ ] Each `RenderErrorKind` — display output matches expected format
- [ ] Fuzzy suggestion: `@iff(x)` → error message includes `did you mean @if?`
- [ ] Component call trace: deeply nested error shows all parent frames in order

### 12.13 Custom directive test (`tests/custom_directive.rs`)

- [ ] Implement `EchoDirective` — echoes arg wrapped in brackets
- [ ] Register, render `@echo("hello")` → `[hello]`
- [ ] Implement block `WrapDirective` — wraps body in custom markers
- [ ] Register, render, assert output

---

## Phase 13 — Polish

### 13.1 Error display formatting

- [ ] Stable error codes: `P001`–`P015` for parse errors, `R001`–`R012` for render errors — add constant map
- [ ] `ParseError` display: `error[PXXX]: description`, `--> file:line:col`, source snippet line, caret line, optional `= hint: ...`
- [ ] `RenderError` display: same first block, then `\ncomponent call trace:\n` with indented frames newest-first

### 13.2 Documentation

- [ ] `//!` module-level doc comments: `lib.rs`, `parser/mod.rs`, `renderer/mod.rs`, `directives/mod.rs`, `helpers/mod.rs`, `types/mod.rs`
- [ ] `///` item-level doc on all public structs, traits, and methods
- [ ] `README.md`: what it is, 10-line quickstart, directive reference table, helper reference table, custom directive example
- [ ] `examples/basic.rs` — engine setup, disk mount, render from JSON, print
- [ ] `examples/custom_directive.rs` — register and use an `AuthorizeDirective`
- [ ] Run `cargo doc --no-deps` — zero warnings

### 13.3 Code quality

- [ ] Run `cargo clippy -- -D warnings` — zero warnings
- [ ] Run `cargo fmt --check` — fully formatted
- [ ] Run `cargo test` — all tests pass
- [ ] Run `cargo test --release` — all tests pass in release mode

### 13.4 Release profile

- [ ] Add `[profile.release]`: `opt-level = 3`, `lto = "thin"`, `codegen-units = 1`, `strip = true`
- [ ] Run `cargo build --release` — clean compile

### 13.5 Crates.io readiness

- [ ] Add `[package]` metadata: `description`, `license = "MIT OR Apache-2.0"`, `repository`, `keywords`, `categories`
- [ ] Add `LICENSE-MIT` and `LICENSE-APACHE` files
- [ ] Run `cargo publish --dry-run` — no issues

---

## Checklist Summary

| Phase | Description | Tasks | Done |
|---|---|---|---|
| 1 | Project Bootstrap | 16 | 0/16 |
| 2 | Types and Value System | 26 | 0/26 |
| 3 | Lexer | 14 | 10/14 |
| 4 | Expression Parser | 18 | 14/18 |
| 5 | Parser (Token → AST) | 57 | 30/57 |
| 6 | Error Types | 12 | 0/12 |
| 7 | Renderer Infrastructure | 26 | 0/26 |
| 8 | Expression Evaluator | 18 | 0/18 |
| 9 | Built-in Helpers | 82 | 0/82 |
| 10 | Renderer | 36 | 0/36 |
| 11 | Public Engine API | 14 | 0/14 |
| 12 | Snapshot Tests | 64 | 0/64 |
| 13 | Polish | 16 | 0/16 |
| **Total** | | **399** | **0 / 399** |

---

## Recommended Build Order

```
Phase 1 → Phase 2 → Phase 3 → Phase 4
                                  ↓
Phase 6 (errors early — referenced by everything after)
                                  ↓
Phase 5 (parser needs errors + lexer + expr parser)
                                  ↓
Phase 7 (renderer infrastructure — scope, buffers, maps)
                                  ↓
Phase 8 (evaluator — needs scope + value)
                                  ↓
Phase 9 (helpers — needs value + evaluator)
                                  ↓
Phase 10 (full renderer — assembles everything)
                                  ↓
Phase 11 (public API wraps renderer)
                                  ↓
Phase 12 (final test verification pass)
                                  ↓
Phase 13 (polish — always last)
```

> Write tests **alongside** each phase as you go — don't wait for Phase 12. The Phase 12 section is a final verification sweep, not the first time each feature is tested.
