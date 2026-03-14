mod common;

use common::engine;
use common::{obj, val};

#[test]
fn test_if_true() {
    let engine = engine();
    let result = engine
        .render_raw("@if(true) 123 @!if", val(obj(vec![])))
        .unwrap();
    assert_eq!(result.trim(), "123");
}

#[test]
fn test_if_false() {
    let engine = engine();
    let result = engine
        .render_raw("@if(false) 123 @!if", val(obj(vec![])))
        .unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_if_else() {
    let engine = engine();
    let result = engine
        .render_raw(
            "@if(flag) YES @!if @else NO @!if",
            val(obj(vec![("flag", true.into())])),
        )
        .unwrap();
    assert!(result.contains("YES"));
}

#[test]
fn test_unless() {
    let engine = engine();
    let result = engine
        .render_raw(
            "@unless(visible) hidden @!unless",
            val(obj(vec![("visible", false.into())])),
        )
        .unwrap();
    assert!(result.contains("hidden"));
}

#[test]
fn test_if_with_var() {
    let engine = engine();
    let result = engine
        .render_raw(
            "@if(name) {{ name }} @!if",
            val(obj(vec![("name", "World".into())])),
        )
        .unwrap();
    assert!(result.contains("World"));
}
