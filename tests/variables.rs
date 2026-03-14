mod common;

use common::engine;
use common::{obj, val};

#[test]
fn test_let_declaration() {
    let engine = engine();
    let result = engine
        .render_raw("@let(x = 5) { {{ x }} }", val(obj(vec![])))
        .unwrap();
    assert_eq!(result.trim(), "5");
}

#[test]
fn test_set_reassignment() {
    let engine = engine();
    let result = engine
        .render_raw("@let(x = 5) @set(x = 10) { {{ x }} }", val(obj(vec![])))
        .unwrap();
    assert_eq!(result.trim(), "10");
}

#[test]
fn test_let_shadowing() {
    let engine = engine();
    let result = engine
        .render_raw(
            "@let(x = 1) { @let(x = 2) { {{ x }} } {{ x }} }",
            val(obj(vec![])),
        )
        .unwrap();
    assert_eq!(result.trim(), "2 1");
}
