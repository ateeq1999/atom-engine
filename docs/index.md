# Atom Engine v5

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

## Next Steps

- [Getting Started Guide](guide/getting-started.html)
- [Template Syntax](templates/syntax.html)
- [Filters Reference](filters/all.html)
- [Components](features/components.html)
- [Examples](examples/index.html)
