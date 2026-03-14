# API Reference

## Atom Engine

The main engine struct for template rendering.

### Creating an Instance

```rust
let engine = Atom::new();
```

### Methods

#### add_template

Adds a raw template to the engine.

```rust
engine.add_template("hello.html", "Hello, {{ name }}!").unwrap();
```

#### load_templates

Loads templates from filesystem using glob pattern.

```rust
engine.load_templates("templates/**/*.html").unwrap();
```

#### render

Renders a template with context data.

```rust
let result = engine.render("template.html", &json!({"name": "World"})).unwrap();
```

#### register_component

Registers a reusable component.

```rust
engine.register_component("button", "<button>{{ text }}</button>").unwrap();
```

#### provide

Provides a value to the context.

```rust
engine.provide("theme", json!("dark"));
```

#### reload

Reloads all templates from disk.

```rust
engine.reload().unwrap();
```

#### template_exists

Checks if a template exists.

```rust
if engine.template_exists("header.html") {
    // ...
}
```

#### get_registered_templates

Returns list of all registered templates.

```rust
let templates = engine.get_registered_templates();
```

#### clear_cache

Clears the template cache.

```rust
engine.clear_cache();
```

#### set_parallel

Enables/disables parallel rendering.

```rust
engine.set_parallel(true);
```

#### enable_component_cache

Enables/disables component caching.

```rust
engine.enable_component_cache(true);
```

#### clear_component_cache

Clears the component cache.

```rust
engine.clear_component_cache();
```

#### component_cache_len

Returns the number of cached components.

```rust
let size = engine.component_cache_len();
```

## ComponentRegistry

Manages component registration and lookup.

### register

```rust
registry.register("component_name", template).unwrap();
```

### get

```rust
if let Some(component) = registry.get("button") {
    // Use component
}
```

## ContextChain

Manages provide/inject context layers.

### provide

```rust
context_chain.provide("key", value);
```

### inject

```rust
if let Some(value) = context_chain.inject("key") {
    // Use value
}
```
