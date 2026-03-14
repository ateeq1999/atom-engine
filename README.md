# Atom Engine v5

A component-oriented template engine for Rust built on [Tera](https://keats.github.io/tera/docs/).

## Features

- **Built on Tera** - Leverages Tera's robust parsing, caching, and expression evaluation
- **Component System** - Register reusable components with props, slots, and validation
- **Provide/Inject Context** - React-style context for passing data to descendants
- **Stack System** - Push content and render stacks
- **30+ Built-in Filters** - String, collection, number, date, and HTML filters
- **Layout Inheritance** - Full support for Tera's `extends`/`block` system
- **Macros** - Define reusable template fragments

## Installation

```toml
[dependencies]
atom-engine = "5"
```

## Quick Start

```rust
use atom_engine::Atom;
use serde_json::json;

fn main() {
    let mut engine = Atom::new();
    
    engine.add_template("hello.html", "Hello, {{ name }}!").unwrap();
    
    let result = engine.render("hello.html", &json!({"name": "World"})).unwrap();
    println!("{}", result);  // Hello, World!
}
```

## Templates

### Variables

```html
{{ user.name }}
{{ items[0] }}
```

### Filters

```html
{{ name | upper }}
{{ text | truncate(length=50) }}
{{ items | length }}
{{ price | round(2) }}
```

### Control Flow

```html
{% if user.is_active %}
  Welcome, {{ user.name }}!
{% elif user.is_pending %}
  Please verify your email.
{% else %}
  <a href="/login">Login</a>
{% endif %}

{% for item in items %}
  <li>{{ item.name }}</li>
{% endfor %}
```

### Layouts

```html
{% extends "base.html" %}

{% block content %}
  <h1>Page Title</h1>
{% endblock %}
```

### Components

```rust
engine.register_component(
    "button",
    r#"<button class="btn">{{ $slots.default() }}</button>"#
).unwrap();

engine.add_template("page.html", r#"{% call button() %}Click Me{% endcall %}"#).unwrap();
```

### Provide/Inject

```rust
engine.provide("theme", json!("dark"));

// In template: {{ inject("theme") }}
```

### Stacks

```html
{% call push("scripts") %}<script src="a.js">{% endcall %}
{{ "scripts" | stack }}
```

## Filters Reference

| Filter | Description |
|--------|-------------|
| `upper` | Uppercase |
| `lower` | Lowercase |
| `capitalize` | Capitalize first letter |
| `title` | Title case |
| `camel_case` | Convert to camelCase |
| `pascal_case` | Convert to PascalCase |
| `snake_case` | Convert to snake_case |
| `kebab_case` | Convert to kebab-case |
| `truncate(length, end?)` | Truncate string |
| `slugify` | URL-safe slug |
| `pluralize(n, singular?, plural?)` | Pluralize |
| `first` | First element |
| `last` | Last element |
| `length` | Count elements |
| `reverse` | Reverse array/string |
| `sort` | Sort array |
| `group_by(attr)` | Group by field |
| `where(attr, value?)` | Filter array |
| `pluck(attr)` | Extract field |
| `round(precision?)` | Round number |
| `abs` | Absolute value |
| `format(format?)` | Format number |
| `date(format?)` | Format date |
| `escape_html` | Escape HTML |
| `safe` | Mark as safe |

## Global Functions

| Function | Description |
|----------|-------------|
| `dump(...)` | Debug output |
| `log(...)` | Log to stderr |
| `range(start?, end, step_by?)` | Generate range |
| `now(utc?, timestamp?)` | Current datetime |
| `push(stack, content)` | Push to stack |
| `prepend(stack, content)` | Prepend to stack |
| `set_slot(name, content)` | Set slot content |

## Examples

Run examples:

```bash
cargo run --example basic
cargo run --example components
cargo run --example layouts
```

## License

MIT OR Apache-2.0
