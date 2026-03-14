use atom_engine::Atom;
use serde_json::json;
use std::collections::HashMap;

fn test_template(name: &str, engine: &Atom, context: &serde_json::Value) {
    match engine.render(name, context) {
        Ok(output) => {
            println!("\n=== {} ===\n{}", name, output);
        }
        Err(e) => {
            println!("\n=== {} - ERROR ===\n{}", name, e);
        }
    }
}

fn main() {
    let mut engine = Atom::new();

    // Load all test templates
    engine
        .load_templates("examples/templates/test_*.html")
        .unwrap();
    engine
        .load_templates("examples/templates/base_*.html")
        .unwrap();
    engine
        .load_templates("examples/templates/include_*.html")
        .unwrap();

    println!(
        "Loaded templates: {:?}",
        engine.render("test_variables.html", &json!({})).is_ok()
    );

    // Test data
    let variables_data = json!({
        "title": "Variables Test",
        "name": "John Doe",
        "age": 30,
        "city": "New York",
        "user": {"name": "Alice", "email": "alice@example.com"},
        "items": ["apple", "banana", "cherry"]
    });

    test_template("test_variables.html", &engine, &variables_data);

    let filters_string_data = json!({
        "text": "hello world",
        "UPPER": "HELLO WORLD",
        "long_text": "This is a very long text that should be truncated",
        "slug_text": "Hello World! How Are You?"
    });

    test_template("test_filters_string.html", &engine, &filters_string_data);

    let filters_collection_data = json!({
        "items": ["cherry", "apple", "banana"],
        "posts": [
            {"title": "Post One", "category": "tech"},
            {"title": "Post Two", "category": "news"},
            {"title": "Post Three", "category": "tech"},
            {"title": "Post Four", "category": "tech"}
        ]
    });

    test_template(
        "test_filters_collection.html",
        &engine,
        &filters_collection_data,
    );

    let filters_number_data = json!({
        "price": 1234.5678,
        "created_at": "2024-01-15T10:30:00Z"
    });

    test_template("test_filters_number.html", &engine, &filters_number_data);

    let filters_conditional_data = json!({
        "null_value": null,
        "empty_string": "",
        "has_value": "hello",
        "is_active": true
    });

    test_template(
        "test_filters_conditional.html",
        &engine,
        &filters_conditional_data,
    );

    let conditionals_data = json!({
        "user": {
            "is_admin": false,
            "is_moderator": false,
            "is_logged_in": true,
            "is_banned": false,
            "is_suspended": false,
            "is_active": true,
            "is_verified": true,
            "age": 25,
            "role": "user"
        },
        "items": ["a", "b", "c"]
    });

    test_template("test_conditionals.html", &engine, &conditionals_data);

    let loops_data = json!({
        "items": ["apple", "banana", "cherry"],
        "user": {"name": "John", "age": "30", "city": "NYC"},
        "empty_items": [],
        "categories": [
            {"name": "Electronics", "products": [
                {"name": "Laptop", "price": 999},
                {"name": "Phone", "price": 699}
            ]},
            {"name": "Books", "products": [
                {"name": "Rust Book", "price": 49}
            ]}
        ]
    });

    test_template("test_loops.html", &engine, &loops_data);

    let inheritance_data = json!({
        "company_name": "Acme Corp",
        "team": [
            {"name": "Alice", "role": "Engineer"},
            {"name": "Bob", "role": "Designer"},
            {"name": "Charlie", "role": "Manager"}
        ]
    });

    test_template("test_inheritance.html", &engine, &inheritance_data);

    let includes_data = json!({
        "page_title": "Welcome",
        "content": "This is the main content.",
        "year": 2024,
        "company": "My Company"
    });

    test_template("test_includes.html", &engine, &includes_data);

    // Test context with provide/inject
    let mut context_engine = Atom::new();
    context_engine
        .load_templates("examples/templates/test_*.html")
        .unwrap();
    context_engine.provide("theme", json!("dark"));
    context_engine.provide("site_name", json!("My Website"));
    context_engine.provide("user", json!({"name": "Alice", "role": "admin"}));
    context_engine.provide("api_url", json!("https://api.example.com"));
    context_engine.provide("features", json!(["auth", "payments", "analytics"]));

    // For stacks, we need a fresh engine
    let mut stacks_engine = Atom::new();
    stacks_engine
        .load_templates("examples/templates/test_stacks.html")
        .unwrap();
    test_template("test_stacks.html", &stacks_engine, &json!({}));

    let full_data = json!({
        "page_title": "Welcome",
        "site_name": "my site",
        "theme": "dark",
        "year": 2024,
        "user": {
            "is_logged_in": true,
            "name": "john",
            "posts": [
                {"title": "First Post", "views": 100},
                {"title": "Second Post", "views": 250}
            ]
        },
        "products": [
            {"name": "Laptop", "category": "electronics", "price": 999.99},
            {"name": "Phone", "category": "electronics", "price": 699.99},
            {"name": "Rust Book", "category": "books", "price": 49.99},
            {"name": "Go Book", "category": "books", "price": 39.99}
        ],
        "categories": ["electronics", "books"],
        "featured_category": "electronics"
    });

    test_template("test_full.html", &engine, &full_data);

    println!("\n\n=== All tests completed ===");
}
