# Getting Started

This guide will help you get up and running with Atom Engine in minutes.

## Installation

Add Atom Engine to your `Cargo.toml`:

```toml
[dependencies]
atom-engine = "5"
```

For additional features:

```toml
[dependencies]
atom-engine = { version = "5", features = ["parallel", "async"] }
```

## Your First Template

```rust
use atom_engine::Atom;
use serde_json::json;

fn main() {
    // Create a new engine instance
    let mut engine = Atom::new();
    
    // Add a template
    engine.add_template("greeting.html", 
        "Hello, {{ name }}! Welcome to {{ site_name }}."
    ).unwrap();
    
    // Render with context data
    let context = json!({
        "name": "Alice",
        "site_name": "My Website"
    });
    
    let result = engine.render("greeting.html", &context).unwrap();
    println!("{}", result);
    // Output: Hello, Alice! Welcome to My Website.
}
```

## Loading Templates from Files

Instead of adding templates inline, you can load them from files:

```rust
let mut engine = Atom::new();

// Load all HTML templates from a directory
engine.load_templates("templates/**/*.html").unwrap();

// Or add individual templates
engine.add_template("header.html", "<header>My Site</header>").unwrap();
engine.add_template("footer.html", "<footer>&copy; 2024</footer>").unwrap();
```

## Using Filters

Filters transform data in your templates:

```rust
engine.add_template("formatted.html", 
    "{{ name | upper }} has {{ items | length }} items"
).unwrap();

let result = engine.render("formatted.html", &json!({
    "name": "bob",
    "items": ["a", "b", "c"]
})).unwrap();

// Output: BOB has 3 items
```

## Working with Components

```rust
// Register a component
engine.register_component(
    "button",
    r#"<button class="btn btn-{{ type }}">{{ text }}</button>"#
).unwrap();

// Use in template
engine.add_template("page.html", 
    r#"{% call button(text="Click Me", type="primary") %}"#
).unwrap();
```

## Next Steps

- Learn about [Template Syntax](templates/syntax.html)
- Explore [All Filters](filters/all.html)
- Read about [Components](features/components.html)
- Check out [Examples](examples/index.html)
