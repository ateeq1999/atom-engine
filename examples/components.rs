use atom_engine::Atom;
use serde_json::json;

fn main() {
    let mut engine = Atom::new();

    engine
        .register_component(
            "button",
            r#"<button class="btn btn-{{ variant | default(value="primary") }}">
{{ $slots.default() }}
</button>"#,
        )
        .unwrap();

    engine
        .register_component(
            "card",
            r#"<div class="card">
  <div class="card-header">{{ title }}</div>
  <div class="card-body">
    {{ $slots.default() }}
  </div>
</div>"#,
        )
        .unwrap();

    engine
        .add_template(
            "button_example.html",
            r#"<div>
  {% set_slot("primary") %}Click Me{% endset_slot %}
  {% call button(variant="primary") %}
</div>"#,
        )
        .unwrap();

    engine
        .add_template(
            "card_example.html",
            r#"{% call card(title="Welcome") %}
  This is the card body content.
{% endcall %}"#,
        )
        .unwrap();

    let result = engine.render("card_example.html", &json!({})).unwrap();
    println!("Card example:\n{}", result);

    engine.provide("theme", json!("dark"));
    engine
        .add_template("theme_example.html", r#"Theme: {{ inject("theme") }}"#)
        .unwrap();

    let result = engine.render("theme_example.html", &json!({})).unwrap();
    println!("\nTheme example:\n{}", result);

    engine
        .add_template(
            "stack_example.html",
            r#"
{% call push("scripts") %}<script src="a.js">{% endcall %}
{% call push("scripts") %}<script src="b.js">{% endcall %}
{{ "scripts" | stack }}
"#,
        )
        .unwrap();

    let result = engine.render("stack_example.html", &json!({})).unwrap();
    println!("\nStack example:\n{}", result);
}
