use atom_engine::Atom;
use serde_json::json;

fn main() {
    let mut engine = Atom::new();

    engine
        .add_template(
            "header.html",
            r#"<header class="site-header">
  <nav>
    <a href="/">Home</a>
    <a href="/about">About</a>
    <a href="/contact">Contact</a>
  </nav>
</header>"#,
        )
        .unwrap();

    engine
        .add_template(
            "footer.html",
            r#"<footer class="site-footer">
  <p>&copy; 2024 My Company. All rights reserved.</p>
</footer>"#,
        )
        .unwrap();

    engine
        .add_template(
            "sidebar.html",
            r#"<aside class="sidebar">
  <h3>Categories</h3>
  <ul>
    <li><a href="/cat/news">News</a></li>
    <li><a href="/cat/tutorials">Tutorials</a></li>
    <li><a href="/cat/blog">Blog</a></li>
  </ul>
</aside>"#,
        )
        .unwrap();

    engine
        .add_template(
            "include_example.html",
            r#"<!DOCTYPE html>
<html>
<head>
  <title>{{ title }}</title>
  <style>
    body { font-family: sans-serif; margin: 0; }
    .container { display: flex; }
    main { flex: 1; padding: 20px; }
  </style>
</head>
<body>
  {% include "header.html" %}
  
  <div class="container">
    {% include "sidebar.html" %}
    
    <main>
      <h1>{{ heading }}</h1>
      <p>{{ content }}</p>
    </main>
  </div>
  
  {% include "footer.html" %}
</body>
</html>"#,
        )
        .unwrap();

    let result = engine
        .render(
            "include_example.html",
            &json!({
                "title": "Welcome",
                "heading": "Hello World",
                "content": "This page demonstrates the include directive."
            }),
        )
        .unwrap();

    println!("=== Include Example ===\n{}", result);

    engine
        .add_template(
            "include_if_example.html",
            r#"<div class="user-info">
{% if user %}
  <p>Welcome, {{ user.name }}!</p>
  {% include "user_menu.html" %}
{% else %}
  <p>Please <a href="/login">login</a></p>
{% endif %}
</div>"#,
        )
        .unwrap();

    engine
        .add_template(
            "user_menu.html",
            r#"<ul class="user-menu">
  <li><a href="/profile">Profile</a></li>
  <li><a href="/settings">Settings</a></li>
  <li><a href="/logout">Logout</a></li>
</ul>"#,
        )
        .unwrap();

    let result = engine
        .render(
            "include_if_example.html",
            &json!({
                "user": {"name": "Alice", "email": "alice@example.com"}
            }),
        )
        .unwrap();

    println!("\n=== Include with If ===\n{}", result);
}
