# Atom Engine v5

[![Crates.io](https://img.shields.io/crates/v/atom-engine)](https://crates.io/crates/atom-engine)
[![Docs](https://docs.rs/atom-engine/badge.svg)](https://docs.rs/atom-engine)
[![License](https://img.shields.io/crates/l/atom-engine)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://rust-lang.org)
[![Build Status](https://github.com/ateeq1999/atom-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/ateeq1999/atom-engine/actions)
[![codecov](https://codecov.io/gh/atom-engine/atom-engine/branch/main/graph/badge.svg)](https://codecov.io/gh/atom-engine/atom-engine)

A high-performance, component-oriented template engine for Rust built on [Tera](https://keats.github.io/tera/docs/).

## Features

- **Built on Tera** - Leverages Tera's robust parsing, caching, and expression evaluation
- **Component System** - Register reusable components with props, slots, and validation
- **Provide/Inject Context** - React-style context for passing data to descendants
- **Stack System** - Push content and render stacks
- **50+ Built-in Filters** - String, collection, number, date, HTML, conditional filters
- **Helper Macros/Directives** - @map, @filter, @each, @reduce, @flatten, @partition
- **Layout Inheritance** - Full support for Tera's `extends`/`block` system
- **Macros** - Define reusable template fragments
- **Async Support** - Render templates asynchronously with tokio
- **Component Caching** - Cache rendered components for performance
- **Parallel Rendering** - Render multiple templates in parallel with Rayon
- **Memory Pool** - Optional memory pooling for allocations

## Installation

```toml
[dependencies]
atom-engine = "5"
```

### Optional Features

```toml
[dependencies]
atom-engine = { version = "5", features = ["parallel", "async", "pool-alloc"] }
```

| Feature | Description |
|---------|-------------|
| `parallel` | Enable parallel rendering with Rayon |
| `async` | Enable async template rendering with Tokio |
| `pool-alloc` | Enable memory pooling for allocations |

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

## Template Syntax

### Variables

```html
{{ user.name }}
{{ items[0] }}
{{ company.address.city }}
{{ items | length }}
```

### Comments

```html
{# This is a comment #}
```

### Filters

Filters transform values. Chain multiple filters:

```html
{{ name | upper | capitalize }}
{{ text | truncate(length=50, end="...") }}
{{ items | sort | reverse | first }}
```

### Control Flow

#### Conditionals

```html
{% if user.is_active %}
    Welcome, {{ user.name }}!
{% elif user.is_pending %}
    Please verify your email.
{% else %}
    <a href="/login">Login</a>
{% endif %}
```

#### Boolean Operators

```html
{% if user.is_active and user.is_verified %}
    Verified user
{% endif %}

{% if is_admin or is_editor %}
    Can edit content
{% endif %}

{% if not user.is_banned %}
    Not banned
{% endif %}
```

#### Comparison Operators

```html
{% if count > 10 %}
    Many items
{% endif %}

{% if status == "active" %}
    Active
{% endif %}

{% if price >= 100 %}
    Premium
{% endif %}
```

#### Loops

```html
{% for item in items %}
    <li>{{ item }}</li>
{% endfor %}

{% for item in items %}
    {{ loop.index }}: {{ item }}
{% endfor %}

{% for item in items %}
    {% if loop.first %}First: {% endif %}{{ item }}
    {% if loop.last %} (Last){% endif %}
{% endfor %}

{% for item in empty_array %}
    {{ item }}
{% empty %}
    No items found
{% endfor %}
```

### Template Inheritance

#### Base Template (base.html)

```html
<!DOCTYPE html>
<html>
<head>
    <title>{% block title %}Default Title{% endblock %}</title>
    {% block extra_head %}{% endblock %}
</head>
<body>
    <header>{% block header %}Header{% endblock %}</header>
    <main>{% block content %}{% endblock %}</main>
    <footer>{% block footer %}Footer{% endblock %}</footer>
</body>
</html>
```

#### Child Template

```html
{% extends "base.html" %}

{% block title %}{{ page_title }}{% endblock %}

{% block content %}
    <h1>{{ article.title }}</h1>
    <p>{{ article.content }}</p>
{% endblock %}
```

### Includes

```html
{% include "header.html" %}

{% includeIf "optional.html" %}

{% includeFirst ["primary.html", "fallback.html"] %}
```

### Macros

```html
{% macro button(text, type="primary") %}
<button class="btn btn-{{ type }}">{{ text }}</button>
{% endmacro %}

{% macro input(name, type="text", placeholder="") %}
<input type="{{ type }}" name="{{ name }}" placeholder="{{ placeholder }}">
{% endmacro %}

{{ button("Submit", type="success") }}
{{ input("email", type="email") }}
```

## Components

### Registering Components

```rust
engine.register_component(
    "button",
    r#"<button class="btn btn-{{ type }}">{{ text }}</button>"#
).unwrap();
```

### Component with Props

```rust
engine.register_component(
    "card",
    r#"<div class="card">
        <h3>{{ title }}</h3>
        <div class="content">{{ $slots.default() }}</div>
    </div>"#
).unwrap();
```

### Using Components

```html
{% call button(text="Click Me", type="primary") %}

{% call card(title="My Card") %}
    Card content here
{% endcall %}
```

### Props Validation

```rust
engine.register_component(
    "user-card",
    r#"{%-- atom: @props(name:String, age:Number, active:Boolean=true) --%}
    <div class="user">
        <h3>{{ name }}</h3>
        <p>Age: {{ age }}</p>
    </div>"#
).unwrap();
```

### Scoped Slots

```html
{# In component #}
{% for item in items %}
    {{ $slots.item(item=item) }}
{% endfor %}

{# Usage #}
{% call list(items=users) %}
    {% fill item(item=user) %}{{ user.name }}{% endfill %}
{% endcall %}
```

## Context (Provide/Inject)

### Providing Values

```rust
engine.provide("theme", json!("dark"));
engine.provide("user", json!({"name": "Alice", "role": "admin"}));
```

### Using in Templates

```html
{# Access provided values #}
Theme: {{ __context.theme }}
User: {{ __context.user.name }}
```

## Stack System

### Push to Stack

```html
{% call push(name="scripts", content="<script>alert('hi')</script>") %}
{% call push(name="scripts", content="<script>console.log('test')</script>") %}
```

### Prepend to Stack

```html
{% call prepend(name="header", content="<li>First</li>") %}
```

### Render Stack

```html
{{ "scripts" | stack }}
```

## Filters Reference

### String Filters

| Filter | Description | Example |
|--------|-------------|---------|
| `upper` | Uppercase | `{{ "hello" \| upper }}` → `HELLO` |
| `lower` | Lowercase | `{{ "HELLO" \| lower }}` → `hello` |
| `capitalize` | Capitalize first | `{{ "hello" \| capitalize }}` → `Hello` |
| `title` | Title case | `{{ "hello world" \| title }}` → `Hello World` |
| `camel_case` | camelCase | `{{ "hello_world" \| camel_case }}` → `helloWorld` |
| `pascal_case` | PascalCase | `{{ "hello_world" \| pascal_case }}` → `HelloWorld` |
| `snake_case` | snake_case | `{{ "helloWorld" \| snake_case }}` → `hello_world` |
| `kebab_case` | kebab-case | `{{ "helloWorld" \| kebab_case }}` → `hello-world` |
| `truncate(length, end?)` | Truncate | `{{ "Hello World" \| truncate(length=5) }}` → `Hello...` |
| `slugify` | URL slug | `{{ "Hello World!" \| slugify }}` → `hello-world` |
| `pluralize(n, singular?, plural?)` | Pluralize | `{{ 1 \| pluralize }}` → `` |
| `replace(old, new)` | Replace | `{{ "Hi" \| replace(old="Hi", new="Hello") }}` → `Hello` |
| `remove(string)` | Remove | `{{ "Hello" \| remove(string="l") }}` → `Heo` |
| `prepend(string)` | Prepend | `{{ "World" \| prepend(string="Hello ") }}` → `Hello World` |
| `append(string)` | Append | `{{ "Hello" \| append(string=" World") }}` → `Hello World` |
| `strip` | Trim whitespace | `{{ "  hi  " \| strip }}` → `hi` |
| `nl2br` | Newlines to `<br>` | `{{ "a\nb" \| nl2br \| safe }}` → `a<br>b` |
| `word_count` | Count words | `{{ "hello world" \| word_count }}` → `2` |
| `char_count` | Count chars | `{{ "hello" \| char_count }}` → `5` |
| `starts_with(prefix)` | Check prefix | `{{ "hello" \| starts_with(prefix="hel") }}` → `true` |
| `ends_with(suffix)` | Check suffix | `{{ "hello" \| ends_with(suffix="llo") }}` → `true` |
| `contains(substring)` | Check contains | `{{ "hello" \| contains(substring="ell") }}` → `true` |

### Collection Filters

| Filter | Description | Example |
|--------|-------------|---------|
| `first` | First element | `{{ [1,2,3] \| first }}` → `1` |
| `last` | Last element | `{{ [1,2,3] \| last }}` → `3` |
| `length` | Count elements | `{{ [1,2,3] \| length }}` → `3` |
| `reverse` | Reverse array | `{{ [1,2,3] \| reverse \| join(",") }}` → `3,2,1` |
| `sort` | Sort array | `{{ [3,1,2] \| sort \| join(",") }}` → `1,2,3` |
| `group_by(attribute)` | Group by field | See docs |
| `where(attribute, value?)` | Filter array | See docs |
| `pluck(attribute)` | Extract field | `{{ users \| pluck(attribute="name") }}` |
| `join(separator)` | Join array | `{{ ["a","b"] \| join("-") }}` → `a-b` |
| `slice(start, length)` | Array slice | `{{ [1,2,3,4] \| slice(start=1, length=2) }}` → `[2,3]` |
| `uniq` | Unique elements | `{{ [1,2,1] \| uniq }}` → `[1,2]` |
| `shuffle` | Random shuffle | `{{ [1,2,3] \| shuffle }}` |
| `map(prop, transform?)` | Transform elements | See helper directives |
| `filter(key, value, op?)` | Filter elements | See helper directives |
| `each(index?)` | Iterate with index | See helper directives |
| `reduce(initial, prop?)` | Reduce to value | See helper directives |
| `flatten` | Flatten nested arrays | `{{ [[1,2],[3]] \| flatten }}` → `[1,2,3]` |
| `partition(key)` | Split array | See helper directives |

### Number Filters

| Filter | Description | Example |
|--------|-------------|---------|
| `round(precision?)` | Round number | `{{ 3.7 \| round }}` → `4` |
| `abs` | Absolute value | `{{ -5 \| abs }}` → `5` |
| `format(format?)` | Format number | `{{ 1000000 \| format }}` → `1,000,000` |
| `min` | Minimum | `{{ [3,1,2] \| min }}` → `1` |
| `max` | Maximum | `{{ [3,1,2] \| max }}` → `3` |
| `sum` | Sum | `{{ [1,2,3] \| sum }}` → `6` |
| `avg` | Average | `{{ [1,2,3] \| avg }}` → `2` |
| `ceil` | Ceiling | `{{ 3.1 \| ceil }}` → `4` |
| `floor` | Floor | `{{ 3.9 \| floor }}` → `3` |

### Date Filters

| Filter | Description | Example |
|--------|-------------|---------|
| `date(format?)` | Format date | `{{ now() \| date }}` → `2024-01-15` |

### HTML Filters

| Filter | Description | Example |
|--------|-------------|---------|
| `escape_html` | Escape HTML | `{{ "<>" \| escape_html }}` → `&lt;&gt;` |
| `safe` | Mark as safe | `{{ "<b>" \| safe }}` → `<b>` |
| `strip_tags` | Remove HTML tags | `{{ "<p>hi</p>" \| strip_tags }}` → `hi` |

### Encoding Filters

| Filter | Description | Example |
|--------|-------------|---------|
| `json_decode` | Parse JSON | `{{ '{"a":1}' \| json_decode \| safe }}` |
| `urlescape` | URL encode | `{{ "a b" \| urlescape }}` → `a%20b` |
| `urlunescape` | URL decode | `{{ "a%20b" \| urlunescape }}` → `a b` |
| `base64_encode` | Base64 encode | `{{ "hi" \| base64_encode }}` → `aGk=` |
| `base64_decode` | Base64 decode | `{{ "aGk=" \| base64_decode }}` → `hi` |

### Conditional Filters

| Filter | Description | Example |
|--------|-------------|---------|
| `default(value)` | Default value | `{{ null \| default(value="x") }}` → `x` |
| `when(then, else)` | Ternary | `{{ true \| when(then="yes", else="no") }}` → `yes` |
| `coalesce(other)` | First non-null | `{{ null \| coalesce(other="x") }}` → `x` |
| `defined` | Check defined | See docs |
| `undefined` | Check undefined | See docs |
| `empty` | Check empty | `{{ [] \| empty }}` → `true` |
| `not_empty` | Check not empty | `{{ [1] \| not_empty }}` → `true` |

## Helper Directives

### @map - Transform Array Elements

```html
{# Extract property #}
{{ users | map(prop="name") | join(", ") }}

{# With transform #}
{{ users | map(prop="name", transform="upper") | join(", ") }}
```

### @filter - Filter Array Elements

```html
{# By value #}
{{ users | filter(key="active", value=true) | map(prop="name") }}

{# By operator #}
{{ users | filter(key="age", value=18, op="gte") }}
```

Operators: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`, `contains`, `exists`

### @each - Iterate with Index

```html
{% for item in items | each(index=true) %}
    {{ item.index }}: {{ item.value }}
{% endfor %}
```

### @reduce - Reduce to Single Value

```html
{{ numbers | reduce(initial=0) }}
{{ users | reduce(initial=0, prop="age") }}
```

### @flatten - Flatten Nested Arrays

```html
{{ [[1,2], [3,4]] | flatten | join(", ") }}
{# Output: 1,2,3,4 #}
```

### @partition - Split Array

```html
{% set partitioned = users | partition(key="active") %}
Matched: {{ partitioned.matched | length }}
Rest: {{ partitioned.rest | length }}
```

## Global Functions

| Function | Description |
|----------|-------------|
| `dump(...)` | Debug output to stderr |
| `log(...)` | Log to stderr |
| `range(start?, end, step_by?)` | Generate range array |
| `now(utc?, timestamp?)` | Current datetime |
| `uuid()` | Generate UUID |
| `random(min, max)` | Random number |
| `choice(array)` | Random array element |
| `file_exists(path)` | Check if file exists |
| `env(key)` | Get environment variable |
| `md5(string)` | MD5 hash |
| `sha256(string)` | SHA256 hash |
| `repeat(content, count, separator?)` | Repeat string |
| `times(times, start?, step?)` | Generate sequence |
| `loop(from, to, step?, inclusive?)` | Loop range |
| `iterate(array, limit?, skip?)` | Iterate with metadata |
| `object(keys, values)` | Create object from arrays |
| `merge(array1, array2)` | Merge arrays |
| `chunk(array, size)` | Chunk array |
| `zip(arrays)` | Zip arrays |
| `compact(array)` | Remove nulls |

## Advanced Features

### Component Caching

```rust
engine.enable_component_cache(true);

// Render components (will be cached)
let result = engine.render("page.html", &ctx).unwrap();

// Check cache stats
println!("Cache size: {}", engine.component_cache_len());

// Clear cache
engine.clear_component_cache();
```

### Async Rendering

```rust
#[cfg(feature = "async")]
async fn render() {
    let engine = Atom::new();
    let result = engine.render_async("template.html", &json!({})).await.unwrap();
}
```

### Parallel Rendering

```rust
#[cfg(feature = "parallel")]
let results = engine.render_many(&[
    ("t1.html", &json!({})),
    ("t2.html", &json!({})),
    ("t3.html", &json!({})),
]).unwrap();
```

### Memory Pool

```rust
use atom_engine::{MemoryPool, StringPool, PooledString};

// Create a memory pool
let pool = MemoryPool::new(4096, 16);

// Create pooled strings
let s = pool.allocate("Hello");
```

## Examples

Run the examples:

```bash
cargo run --example basic
cargo run --example components
cargo run --example layouts
cargo run --example filters
cargo run --example comprehensive_test
```

## Testing

```bash
# Run all tests
cargo test

# Run with coverage
cargo test -- --nocapture

# Run specific test
cargo test test_filters
```

## License

MIT OR Apache-2.0
