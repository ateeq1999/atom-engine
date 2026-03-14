use atom_engine::Atom;
use serde_json::json;

fn main() {
    let mut engine = Atom::new();

    engine
        .add_template("hello.html", "Hello, {{ name | capitalize }}!")
        .unwrap();

    let result = engine
        .render("hello.html", &json!({"name": "world"}))
        .unwrap();
    println!("{}", result);

    engine
        .add_template(
            "list.html",
            r#"
<ul>
{% for item in items %}
  <li>{{ item.name }} - {{ item.price | round(2) }}</li>
{% endfor %}
</ul>
"#,
        )
        .unwrap();

    let result = engine
        .render(
            "list.html",
            &json!({
                "items": [
                    {"name": "Apple", "price": 1.234},
                    {"name": "Banana", "price": 2.567}
                ]
            }),
        )
        .unwrap();
    println!("{}", result);

    engine
        .add_template(
            "conditional.html",
            r#"
{% if user.is_active %}
  Welcome back, {{ user.name }}!
{% elif user.is_new %}
  Welcome! Please complete your profile.
{% else %}
  Please log in.
{% endif %}
"#,
        )
        .unwrap();

    let result = engine
        .render(
            "conditional.html",
            &json!({
                "user": {"name": "Alice", "is_active": true}
            }),
        )
        .unwrap();
    println!("{}", result);
}
