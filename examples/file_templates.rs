use atom_engine::Atom;
use serde_json::json;

fn main() {
    let mut engine = Atom::new();

    engine
        .load_templates("examples/templates/**/*.html")
        .unwrap();

    let result = engine
        .render(
            "about.html",
            &json!({
                "title": "About Us",
                "heading": "Our Company",
                "team": [
                    {"name": "Alice", "role": "Engineer"},
                    {"name": "Bob", "role": "Designer"},
                    {"name": "Charlie", "role": "Manager"}
                ]
            }),
        )
        .unwrap();

    println!("{}", result);
}
