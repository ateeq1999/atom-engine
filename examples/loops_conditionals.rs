use atom_engine::Atom;
use serde_json::json;

fn main() {
    let mut engine = Atom::new();

    engine
        .add_template(
            "conditionals.html",
            r#"<h1>Conditionals</h1>

{% if user.is_admin %}
  <p>Welcome, Admin!</p>
{% elif user.is_logged_in %}
  <p>Welcome, {{ user.name }}!</p>
{% else %}
  <p>Please log in.</p>
{% endif %}

{% if items | length > 0 %}
  <p>There are {{ items | length }} items.</p>
{% endif %}"#,
        )
        .unwrap();

    let result = engine
        .render(
            "conditionals.html",
            &json!({
                "user": {"name": "Alice", "is_admin": false, "is_logged_in": true},
                "items": [1, 2, 3]
            }),
        )
        .unwrap();
    println!("=== Conditionals ===\n{}", result);

    let mut engine2 = Atom::new();

    engine2
        .add_template(
            "loops.html",
            r#"<h1>Loops</h1>

<h2>Basic For Loop</h2>
<ul>
{% for item in items %}
  <li>{{ item }}</li>
{% endfor %}
</ul>

<h2>For Loop with Index</h2>
<ol>
{% for item in items %}
  <li>{{ loop.index }}: {{ item }}</li>
{% endfor %}
</ol>

<h2>First/Last</h2>
{% for item in items %}
  {% if loop.first %}(First) {% endif %}{{ item }}{% if loop.last %} (Last){% endif %}
{% endfor %}

<h2>For Loop Keys</h2>
{% for key, value in user %}
  <p>{{ key }}: {{ value }}</p>
{% endfor %}"#,
        )
        .unwrap();

    let result = engine2
        .render(
            "loops.html",
            &json!({
                "items": ["apple", "banana", "cherry"],
                "user": {"name": "John", "age": "30", "city": "NYC"}
            }),
        )
        .unwrap();
    println!("\n=== Loops ===\n{}", result);

    let mut engine3 = Atom::new();

    engine3
        .add_template(
            "nested.html",
            r#"<h1>Nested Loops</h1>

{% for category in categories %}
<h3>{{ category.name }}</h3>
<ul>
{% for product in category.products %}
  <li>{{ product.name }} - ${{ product.price }}</li>
{% endfor %}
</ul>
{% endfor %}"#,
        )
        .unwrap();

    let result = engine3
        .render(
            "nested.html",
            &json!({
                "categories": [
                    {
                        "name": "Electronics",
                        "products": [
                            {"name": "Laptop", "price": 999},
                            {"name": "Phone", "price": 699}
                        ]
                    },
                    {
                        "name": "Books",
                        "products": [
                            {"name": "Rust Book", "price": 49}
                        ]
                    }
                ]
            }),
        )
        .unwrap();
    println!("\n=== Nested Loops ===\n{}", result);
}
