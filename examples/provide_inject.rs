use atom_engine::Atom;
use serde_json::json;

fn main() {
    let mut engine = Atom::new();

    engine.provide("theme", json!("dark"));
    engine.provide("user", json!({"name": "Alice", "role": "admin"}));
    engine.provide("site_name", json!("My Website"));

    engine
        .add_template(
            "context_basic.html",
            r#"<h1>Context Injection Demo</h1>

<p>Theme: {{ theme }}</p>
<p>Site: {{ site_name }}</p>
<p>User: {{ user.name }} ({{ user.role }})</p>"#,
        )
        .unwrap();

    let result = engine.render("context_basic.html", &json!({})).unwrap();
    println!("=== Basic Context ===\n{}", result);

    let mut engine2 = Atom::new();

    engine2.provide("theme", json!("light"));
    engine2.provide(
        "config",
        json!({
            "title": "My Blog",
            "items_per_page": 10,
            "show_comments": true
        }),
    );

    engine2
        .add_template(
            "nested_context.html",
            r#"<!DOCTYPE html>
<html>
<head>
  <title>{{ config.title }}</title>
  <style>
    body { 
      {% if theme == "dark" %}
      background: #1a1a1a; color: #fff;
      {% else %}
      background: #fff; color: #333;
      {% endif %}
    }
  </style>
</head>
<body>
  <h1>{{ config.title }}</h1>
  
  {% if config.show_comments %}
  <p>Comments are enabled.</p>
  {% endif %}
  
  <p>Items per page: {{ config.items_per_page }}</p>
</body>
</html>"#,
        )
        .unwrap();

    let result = engine2.render("nested_context.html", &json!({})).unwrap();
    println!("\n=== Nested Context ===\n{}", result);

    let mut engine3 = Atom::new();

    engine3
        .add_template(
            "set_context.html",
            r#"<h1>Using provide() in code</h1>

<p>In Rust code:</p>
<pre>
engine.provide("api_url", "https://api.example.com");
engine.provide("version", "1.0.0");
engine.provide("features", ["auth", "payments", "analytics"]);
</pre>

<p>In template:</p>
<ul>
  <li>API URL: {{ api_url }}</li>
  <li>Version: {{ version }}</li>
  <li>Features: {{ features | length }}</li>
  {% for feature in features %}
    <li>{{ feature }}</li>
  {% endfor %}
</ul>"#,
        )
        .unwrap();

    engine3.provide("api_url", json!("https://api.example.com"));
    engine3.provide("version", json!("1.0.0"));
    engine3.provide("features", json!(["auth", "payments", "analytics"]));

    let result = engine3.render("set_context.html", &json!({})).unwrap();
    println!("\n=== Provide in Code ===\n{}", result);

    let mut engine4 = Atom::new();

    engine4
        .add_template(
            "layered_context.html",
            r#"<h1>Layered Context</h1>

<p>Base value: {{ message }}</p>"#,
        )
        .unwrap();

    engine4.provide("message", json!("Base message"));

    let result = engine4
        .render(
            "layered_context.html",
            &json!({
                "message": "Override from render context"
            }),
        )
        .unwrap();

    println!("\n=== Render Context Override ===\n{}", result);
}
