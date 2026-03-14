# Atom Engine

### A Rust Template Engine Built on Tera with Edge.js-Inspired Components

**Version 5.0 · March 2026**

> Built on Tera · `.html`/`.tera` templates · Component system · Edge.js-inspired syntax  
> Format-agnostic · Zero-tier architecture · Composable · Extensible

---

## 1. What Atom Engine Is

Atom Engine is a **component-oriented template engine built on Tera**. It leverages Tera's mature parsing, caching, and evaluation engine while adding a powerful component system inspired by Edge.js and React.

Tera handles the heavy lifting:
- Template parsing and AST compilation
- Template inheritance (`extends`/`block`)
- Filters, functions, and tests
- Auto-escaping for HTML
- Template caching and reloading

Atom Engine adds on top:
- **Component system** — Props, slots, scoped slots, `@provide`/`@inject`
- **File-based component resolution** — Components as tags from file paths
- **Enhanced control flow** — `@each` with destructuring, `@switch`, `@while`
- **Built-in helper library** — Rich string, collection, number, and date helpers
- **Custom component directives** — Register components with the same API

The design is inspired by:

**EdgeJS**: `@directive` syntax that is terse, readable, and feels like writing code rather than learning a templating dialect.

**React**: The component model. Templates are components. Components accept props, render children, define named slots, and compose other components.

**Tera**: The battle-tested Rust template engine that handles parsing, evaluation, and caching.

---

## 2. Template Files

Every template is a `.html` or `.tera` file using Tera's syntax. The file extension determines auto-escaping behavior (`.html` files are auto-escaped by default).

```html
Hello, {{ name }}!
```

Templates can use all of Tera's features: inheritance, blocks, macros, includes, filters, and more.

### 2.1 Component Header (Optional)

Components can declare props using a comment-style header:

```html
{%-- atom:
@props({
  title: String,
  items: Array,
  count: Number = 0,
  active: Boolean = true,
  label: String = "default",
})
--%}
```

This is parsed at load time and validates props at render time.

---

## 3. Tera Syntax (Base)

Atom Engine inherits Tera's complete syntax:

### 3.1 Interpolation

```html
{{ name }}
{{ user.name }}
{{ items[0] }}
{{ "hello" | upper }}
```

### 3.2 Expressions

```html
{{ 1 + 2 }}
{{ "hello" ~ "world" }}
{% if user.is_admin and user.age >= 18 %}
{{ 'admin' if user.is_admin else 'user' }}
```

### 3.3 Control Flow

```html
{% if user.is_admin %}
  <span class="badge">Admin</span>
{% elif user.is_moderator %}
  <span class="badge">Moderator</span>
{% else %}
  <span class="badge">Member</span>
{% endif %}

{% for user in users %}
  <li>{{ user.name }}</li>
{% endfor %}

{% include "partial.html" %}
```

### 3.4 Inheritance

```html
{% extends "base.html" %}

{% block content %}
  <p>Page content</p>
{% endblock %}
```

---

## 4. Atom Components — The Component System

Components are the primary abstraction in Atom Engine. They are templates that accept props and render content.

### 4.1 Using Components

Components can be used with the `@component` directive:

```html
@component('components/card', { title: "Welcome" })
  <p>This is the card body — it fills the main slot.</p>
@endcomponent
```

Or using file-based tag resolution (see §5):

```html
@card({ title: "Welcome" })
  <p>This is the card body.</p>
@endcard
```

Self-closing form:

```html
@spinner({ size: "lg" })
```

### 4.2 Defining Components

Components are just templates. They declare props and use slots:

```html
{%-- atom:
@props({
  title: String,
  items: Array,
  elevated: Boolean = false,
})
--%}

<div class="card{% if elevated %} card-elevated{% endif %}">
  <div class="card-header">
    {% if slot_header %}
      {{ slot_header() }}
    {% else %}
      <h3>{{ title }}</h3>
    {% endif %}
  </div>
  <div class="card-body">
    {{ slot_default() }}
  </div>
  <div class="card-footer">
    {{ slot_footer() }}
  </div>
</div>
```

### 4.3 Slots

Slots are declared in components using function-call syntax:

| Slot Syntax | Description |
|---|---|
| `{{ slot_default() }}` | Main slot (content without a named slot) |
| `{{ slot_header() }}` | Named slot "header" |
| `{{ slot(name) }}` | Dynamic slot name |

#### Default Slot Content

```html
{% if slot_default() %}
  {{ slot_default() }}
{% else %}
  <p>Default content here</p>
{% endif %}
```

### 4.4 Filling Slots

When calling a component, use `@slot` blocks:

```html
@card({ title: "My Card", elevated: true })
  @slot(header)
    <h2>Custom Header</h2>
  @endslot
  
  <p>Main content goes here.</p>
  
  @slot(footer)
    <button>Action</button>
  @endslot
@endcard
```

### 4.5 Scoped Slots

Components can expose data to callers via scoped slots:

```html
{%-- In list component --%}
{% for item in items %}
  <li>
    {{ slot_item({ item: item, index: loop.index0, first: loop.first, last: loop.last }) }}
  </li>
{% endfor %}
```

Callers access scoped slot data:

```html
@list({ items: products })
  @slot(item, { item, index, first, last })
    <span class="{% if last %}last{% endif %}">{{ item.name }}</span>
  @endslot
@endlist
```

### 4.6 Context: `@provide` and `@inject`

React-style context for cross-component data:

```html
{%-- In a parent template --%}
{% provide("theme", "dark") %}
{% provide("locale", user.locale) %}

@layout
  @nav @endnav
@endlayout
```

```html
{%-- In nav component --%}
{% inject("theme") %}
{% inject("locale") %}

<nav class="nav-{{ theme }}">
  <span>{{ locale }}</span>
</nav>
```

---

## 5. Component Tags — File-Based Resolution

Any template file inside the `components/` directory can be referenced as a tag:

| File path | Tag usage |
|---|---|
| `components/button.html` | `@button({ label: "Click" })` |
| `components/form/input.html` | `@form.input({ name: "email" })` |
| `components/ui/modal.html` | `@ui.modal({ open: true })` |

```html
@button({ label: "Submit", type: "submit" })

@form.input({ name: "email", type: "email" })
  @slot(icon)
    <i class="icon-email"></i>
  @endslot
@endform

@!ui.spinner({ size: "sm" })
```

---

## 6. Enhanced Directives

Atom Engine adds several directives beyond Tera's built-in ones.

### 6.1 Control Flow

#### `@each` with Destructuring

```html
@each([key, value] in config)
  {{ key }}: {{ value }}
@endeach

@each({ name, email } in users)
  {{ name }} <{{ email }}>
@endeach
```

#### `@switch` / `@case` / `@default`

```html
@switch(status)
  @case("active")
    <span class="green">Active</span>
  @case("pending")
    <span class="yellow">Pending</span>
  @case("banned")
    <span class="red">Banned</span>
  @default
    <span class="grey">Unknown</span>
@endswitch
```

#### `@while`

```html
{% set n = 1 %}
{% while n <= 5 %}
  {{ n }}
  {% set n = n + 1 %}
{% endwhile %}
```

### 6.2 Variable Directives

#### `@let`

```html
@let(greeting = "Hello, " ~ name ~ "!")
@let(count = items | length)
@let(isLong = count > 100)
```

#### `@set`

```html
{% set sep = "" %}
@each(tag in tags)
  {{ sep }}{{ tag }}
  {% set sep = ", " %}
@endeach
```

### 6.3 Stack Directives

CSS/JS accumulation pattern:

```html
@push(styles)
  <link rel="stylesheet" href="/page.css">
@endpush

@prepend(styles)
  <link rel="stylesheet" href="/critical.css">
@endprepend

{{ stack("styles") }}
```

### 6.4 Utility Directives

```html
@dump(user)
@log("Rendering: " ~ name)
@comment
  This is ignored
@endcomment
@once
  <script src="/heavy.js"></script>
@endonce
```

---

## 7. Built-in Filters

Atom Engine provides a rich filter library on top of Tera's built-ins.

### 7.1 String Filters

| Filter | Description | Example |
|---|---|---|
| `upper` | UPPER CASE | `"hello" \| upper` → `"HELLO"` |
| `lower` | lower case | `"HELLO" \| lower` → `"hello"` |
| `capitalize` | First letter upper | `"hello" \| capitalize` → `"Hello"` |
| `title` | Title Case | `"hello world" \| title` → `"Hello World"` |
| `camel` | camelCase | `"hello-world" \| camel` → `"helloWorld"` |
| `pascal` | PascalCase | `"hello-world" \| pascal` → `"HelloWorld"` |
| `snake` | snake_case | `"HelloWorld" \| snake` → `"hello_world"` |
| `kebab` | kebab-case | `"HelloWorld" \| kebab` → `"hello-world"` |
| `truncate(n)` | Truncate to n chars | `"hello world" \| truncate(5)` → `"hello..."` |
| `slugify` | URL-safe slug | `"Hello World!" \| slugify` → `"hello-world"` |
| `pluralize` | "1 item" / "2 items" | `count \| pluralize` |

### 7.2 Collection Filters

| Filter | Description |
|---|---|
| `first` | First element |
| `last` | Last element |
| `nth(n)` | Element at index n |
| `pluck(key)` | Extract field from each |
| `group_by(key)` | Group by field |
| `sort_by(key)` | Sort by field |
| `where(key, value)` | Filter by field |
| `reject(key, value)` | Inverse filter |

### 7.3 Number Filters

| Filter | Description |
|---|---|
| `round(n)` | Round to n decimals |
| `format(pattern)` | Locale number format |
| `currency(symbol)` | Currency format |
| `percent(decimals)` | Percentage format |

### 7.4 Date Filters

| Filter | Description |
|---|---|
| `format(fmt)` | Format date |
| `to_relative` | "3 days ago" |
| `to_iso` | ISO 8601 string |
| `start_of(unit)` | Start of unit |
| `end_of(unit)` | End of unit |

---

## 8. Engine Architecture

### 8.1 Processing Pipeline

```
Source (.html file)
       │
       ▼
   Tera::new() / add_raw_templates()
       │
       ▼
   AST (parsed by Tera)
       │
       ▼
   AtomComponentLoader (processes @props, registers components)
       │
       ▼
   ComponentRegistry (maps file paths to component templates)
       │
       ▼
   RenderContext with components + filters + globals
       │
       ▼
   Rendered string
```

### 8.2 Component Resolution

```rust
pub struct ComponentRegistry {
    components: HashMap<String, Component>,
}

pub struct Component {
    pub path: String,
    pub props: Vec<PropDef>,
    pub template: String,
}

impl ComponentRegistry {
    pub fn register(&mut self, path: &str, template: &str) -> Result<(), Error>;
    pub fn get(&self, path: &str) -> Option<&Component>;
    pub fn resolve_tag(&self, tag: &str) -> Option<String>;  // button -> components/button
}
```

### 8.3 Slot Resolution

```rust
pub struct SlotResolver {
    slots: HashMap<String, Vec<Node>>,
}

impl SlotResolver {
    pub fn set(&mut self, name: &str, content: Vec<Node>);
    pub fn get(&self, name: &str) -> Option<&Vec<Node>>;
    pub fn has(&self, name: &str) -> bool;
}
```

### 8.4 Context Chain (Provide/Inject)

```rust
pub struct ContextChain {
    layers: Vec<HashMap<String, Value>>,
}

impl ContextChain {
    pub fn provide(&mut self, key: &str, value: Value);
    pub fn inject(&self, key: &str) -> Option<&Value>;
}
```

---

## 9. Public API

```rust
use tera::Tera;
use serde_json::Value;

pub struct Atom {
    tera: Tera,
    components: ComponentRegistry,
    filters: FilterRegistry,
}

impl Atom {
    pub fn new() -> Self;

    /// Load templates from a directory glob
    pub fn load_templates(&mut self, glob: &str) -> Result<(), Error>;

    /// Load templates from raw strings
    pub fn add_template(&mut self, name: &str, content: &str) -> Result<(), Error>;

    /// Register a component
    pub fn register_component(&mut self, path: &str, template: &str) -> Result<(), Error>;

    /// Register a custom filter
    pub fn register_filter<F>(&mut self, name: &str, filter: F)
    where F: Fn(Value, Vec<Value>, HashMap<String, Value>) -> Result<Value> + Send + Sync + 'static;

    /// Register a global function
    pub fn register_function<F>(&mut self, name: &str, func: F)
    where F: Fn(HashMap<String, Value>) -> Result<Value> + Send + Sync + 'static;

    /// Render a template
    pub fn render(&self, template: &str, context: &Value) -> Result<String, Error>;

    /// Render with component context
    pub fn render_with_components(&self, template: &str, context: &Value, components: &Value) -> Result<String, Error>;
}
```

### 9.1 Usage Example

```rust
use atom_engine::Atom;
use serde_json::json;

let mut atom = Atom::new();

// Load templates
atom.load_templates("templates/**/*.html")?;

// Or add components directly
atom.register_component("components/card.html", r#"
<div class="card">
  <h3>{{ title }}</h3>
  {{ slot_default() }}
</div>
"#)?;

// Render
let context = json!({
    "title": "Welcome",
    "user": { "name": "Alice" }
});

let output = atom.render("index.html", &context)?;
```

---

## 10. Template Examples

### 10.1 Component with Slots

```html
{%-- components/card.html --%}
{%-- atom:
@props({
  title: String,
  elevated: Boolean = false,
})
--%}
<div class="card{% if elevated %} elevated{% endif %}">
  <div class="card-header">
    {% if slot_header %}
      {{ slot_header() }}
    {% else %}
      <h3>{{ title }}</h3>
    {% endif %}
  </div>
  <div class="card-body">
    {{ slot_default() }}
  </div>
</div>
```

```html
{%-- Usage --%}
@card({ title: "Hello", elevated: true })
  <p>Card content goes here.</p>
  
  @slot(header)
    <h2>Custom Title</h2>
  @endslot
@endcard
```

### 10.2 Layout with Stacks

```html
{%-- layouts/base.html --%}
<!DOCTYPE html>
<html>
<head>
  <title>{% block title %}My Site{% endblock %}</title>
  {{ stack("styles") }}
</head>
<body>
  {% block content %}{% endblock %}
  {{ stack("scripts") }}
</body>
</html>
```

```html
{%-- extends base --%}
{% extends "layouts/base.html" %}

{% block title %}Home Page{% endblock %}

{% block content %}
  <h1>Welcome</h1>
{% endblock %}

{% push styles %}
  <link rel="stylesheet" href="/home.css">
{% endpush %}

{% push scripts %}
  <script src="/home.js"></script>
{% endpush %}
```

### 10.3 List with Scoped Slots

```html
{%-- components/list.html --%}
{%-- atom:
@props({
  items: Array,
})
--%}
<ul>
{% for item in items %}
  <li>
    {{ slot_item({ 
      item: item, 
      index: loop.index0, 
      first: loop.first, 
      last: loop.last 
    }) }}
  </li>
{% endfor %}
</ul>
```

```html
{%-- Usage --%}
@list({ items: products })
  @slot(item, { item, index, first, last })
    <span class="{% if last %}last-item{% endif %}">
      {{ index + 1 }}. {{ item.name }}
    </span>
  @endslot
@endlist
```

---

## 11. Implementation Plan

### Phase 1 — Core Integration (Week 1)

- Integrate Tera as the base engine
- Set up ComponentRegistry and slot resolution
- Implement `@component` / `@endcomponent` directive
- Implement `@slot` / `@endslot` directive
- Basic props validation

### Phase 2 — Component System (Week 1-2)

- File-based component tag resolution
- Scoped slots
- `@provide` / `@inject` context system
- Component inheritance patterns

### Phase 3 — Enhanced Directives (Week 2)

- `@each` with destructuring
- `@switch` / `@case` / `@default`
- `@while` with iteration limits
- `@let` / `@set`
- `@push` / `@prepend` / `stack()`
- `@dump` / `@log` / `@comment` / `@once`

### Phase 4 — Filters and Helpers (Week 2-3)

- String transformation filters
- Collection manipulation filters
- Number and date formatting
- Custom filter registration API

### Phase 5 — Testing and Polish (Week 3-4)

- Integration tests for all component features
- Error handling with helpful messages
- Documentation and examples

---

## 12. Dependencies

```toml
[package]
name = "atom-engine"
version = "5.0.0"
edition = "2021"

[dependencies]
tera = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
anyhow = "1"
indexmap = "2"

[dev-dependencies]
tempfile = "3"
```

---

## 13. Conclusion

Atom Engine v5 builds on Tera's mature foundation to provide a powerful component system. By leveraging Tera's parsing, caching, and evaluation, Atom Engine focuses on what matters: building composable, reusable template components.

The component system brings React-like patterns to server-side templates:
- Props for component input
- Slots for content composition  
- Scoped slots for data sharing
- Context for cross-component communication
- File-based resolution for intuitive usage

Combined with enhanced control flow directives and a rich filter library, Atom Engine provides everything needed for modern template development in Rust.
