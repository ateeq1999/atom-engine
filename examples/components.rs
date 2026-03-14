use atom_engine::Atom;
use serde_json::json;

fn main() {
    let mut engine = Atom::new();

    engine
        .register_component(
            "card",
            r#"<div class="card">
  <div class="card-header">{{ title }}</div>
  <div class="card-body">{{ content }}</div>
</div>"#,
        )
        .unwrap();

    engine
        .register_component(
            "button",
            r#"<button class="btn btn-{{ variant }}">{{ text }}</button>"#,
        )
        .unwrap();

    engine
        .add_template(
            "registered_components.html",
            r#"<h1>Components</h1>

<div class="card">
  <div class="card-header">Component Test</div>
  <div class="card-body">This is a registered component</div>
</div>

<button class="btn btn-primary">Primary</button>
<button class="btn btn-secondary">Secondary</button>"#,
        )
        .unwrap();

    let result = engine
        .render("registered_components.html", &json!({}))
        .unwrap();
    println!("=== Components ===\n{}", result);

    let mut engine2 = Atom::new();

    engine2
        .add_template(
            "props_demo.html",
            r#"<h1>Props Demo</h1>

<h2>From Context</h2>
<p>Title: {{ title }}</p>
<p>User: {{ user.name }}</p>
<p>Count: {{ count }}</p>

<h2>Conditionals</h2>
{% if user.is_admin %}
  <p>Admin User</p>
{% elif user.is_active %}
  <p>Active User</p>
{% else %}
  <p>Inactive User</p>
{% endif %}

<h2>Loops</h2>
{% for item in items %}
  <li>{{ item }}</li>
{% endfor %}"#,
        )
        .unwrap();

    let result = engine2
        .render(
            "props_demo.html",
            &json!({
                "title": "Props and Data",
                "user": {"name": "Alice", "is_admin": true, "is_active": true},
                "count": 42,
                "items": ["Apple", "Banana", "Cherry"]
            }),
        )
        .unwrap();

    println!("\n=== Props Demo ===\n{}", result);
}
