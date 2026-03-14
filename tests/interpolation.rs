mod common;

use common::engine;
use common::{obj, val};

#[test]
fn test_simple_text() {
    let engine = engine();
    let result = engine.render_raw("Hello World!", val(obj(vec![])));
    assert_eq!(result.unwrap(), "Hello World!");
}

#[test]
fn test_interpolation_string() {
    let engine = engine();
    let result = engine
        .render_raw(
            "Hello {{ name }}!",
            val(obj(vec![("name", "World".into())])),
        )
        .unwrap();
    assert_eq!(result, "Hello World!");
}

#[test]
fn test_interpolation_number() {
    let engine = engine();
    let result = engine
        .render_raw("Count: {{ count }}", val(obj(vec![("count", 42.0.into())])))
        .unwrap();
    assert_eq!(result, "Count: 42");
}

#[test]
fn test_interpolation_arithmetic() {
    let engine = engine();
    let result = engine
        .render_raw(
            "{{ price * quantity }}",
            val(obj(vec![("price", 10.0.into()), ("quantity", 5.0.into())])),
        )
        .unwrap();
    assert_eq!(result, "50");
}
