use atom_engine::Atom;
use serde_json::json;

fn main() {
    let mut engine = Atom::new();

    engine
        .add_template(
            "strings.html",
            r#"<h1>String Filters</h1>
<p>Original: "hello world"</p>
<p>upper: "{{ "hello world" | upper }}"</p>
<p>lower: "{{ "HELLO" | lower }}"</p>
<p>capitalize: "{{ "hello world" | capitalize }}"</p>
<p>title: "{{ "hello world" | title }}"</p>
<p>camel: "{{ "hello_world" | camel_case }}"</p>
<p>snake: "{{ "helloWorld" | snake_case }}"</p>
<p>kebab: "{{ "helloWorld" | kebab_case }}"</p>
<p>truncate: "{{ "Hello World!" | truncate(length=8) }}"</p>
<p>slugify: "{{ "Hello World! How Are You?" | slugify }}"</p>"#,
        )
        .unwrap();

    let result = engine.render("strings.html", &json!({})).unwrap();
    println!("=== String Filters ===\n{}", result);

    let mut engine2 = Atom::new();

    engine2
        .add_template(
            "numbers.html",
            r#"<h1>Number Filters</h1>
<p>Original: 1234.5678</p>
<p>round: {{ 1234.5678 | round }}</p>"#,
        )
        .unwrap();

    let result = engine2.render("numbers.html", &json!({})).unwrap();
    println!("\n=== Number Filters ===\n{}", result);

    let mut engine3 = Atom::new();

    engine3
        .add_template(
            "collections.html",
            r#"<h1>Collection Filters</h1>
<p>Items: {{ items }}</p>
<p>first: "{{ items | first }}"</p>
<p>last: "{{ items | last }}"</p>
<p>length: {{ items | length }}</p>
<p>reverse: {{ items | reverse }}</p>
<p>sort: {{ items | sort }}</p>"#,
        )
        .unwrap();

    let result = engine3
        .render(
            "collections.html",
            &json!({
                "items": ["cherry", "apple", "banana"]
            }),
        )
        .unwrap();
    println!("\n=== Collection Filters ===\n{}", result);

    let mut engine4 = Atom::new();

    engine4
        .add_template(
            "conditional.html",
            r#"<h1>Conditional Filters</h1>
<p>null | default: "{{ null_value | default(value="fallback") }}"</p>
<p>empty | default: "{{ empty | default(value="fallback") }}"</p>
<p>value | default: "{{ value | default(value="fallback") }}"</p>"#,
        )
        .unwrap();

    let result = engine4
        .render(
            "conditional.html",
            &json!({
                "null_value": null,
                "empty": "",
                "value": "has value"
            }),
        )
        .unwrap();
    println!("\n=== Conditional Filters ===\n{}", result);

    let mut engine5 = Atom::new();

    engine5
        .add_template(
            "html_filter.html",
            r#"<h1>HTML Filters</h1>
<p>escape: {{ "<b>bold</b>" | escape_html }}</p>
<p>safe: {{ "<b>bold</b>" | safe }}</p>"#,
        )
        .unwrap();

    let result = engine5.render("html_filter.html", &json!({})).unwrap();
    println!("\n=== HTML Filters ===\n{}", result);
}
