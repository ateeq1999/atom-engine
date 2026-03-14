mod common;

use common::{arr, obj, val};
use serde_json::json;

#[test]
fn test_basic_interpolation() {
    let mut engine = common::engine();
    engine
        .add_template("hello.html", "Hello, {{ name }}!")
        .unwrap();

    let result = engine
        .render("hello.html", &json!({"name": "World"}))
        .unwrap();
    assert_eq!(result, "Hello, World!");
}

#[test]
fn test_filter_upper() {
    let mut engine = common::engine();
    engine
        .add_template("upper.html", "{{ name | upper }}")
        .unwrap();

    let result = engine
        .render("upper.html", &json!({"name": "hello"}))
        .unwrap();
    assert_eq!(result, "HELLO");
}

#[test]
fn test_filter_lower() {
    let mut engine = common::engine();
    engine
        .add_template("lower.html", "{{ name | lower }}")
        .unwrap();

    let result = engine
        .render("lower.html", &json!({"name": "HELLO"}))
        .unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_filter_capitalize() {
    let mut engine = common::engine();
    engine
        .add_template("capitalize.html", "{{ name | capitalize }}")
        .unwrap();

    let result = engine
        .render("capitalize.html", &json!({"name": "hello world"}))
        .unwrap();
    assert_eq!(result, "Hello world");
}

#[test]
fn test_filter_length() {
    let mut engine = common::engine();
    engine
        .add_template("length.html", "{{ items | length }}")
        .unwrap();

    let result = engine
        .render("length.html", &json!({"items": [1, 2, 3]}))
        .unwrap();
    assert_eq!(result, "3");
}

#[test]
fn test_filter_first() {
    let mut engine = common::engine();
    engine
        .add_template("first.html", "{{ items | first }}")
        .unwrap();

    let result = engine
        .render("first.html", &json!({"items": ["a", "b", "c"]}))
        .unwrap();
    assert_eq!(result, "a");
}

#[test]
fn test_filter_last() {
    let mut engine = common::engine();
    engine
        .add_template("last.html", "{{ items | last }}")
        .unwrap();

    let result = engine
        .render("last.html", &json!({"items": ["a", "b", "c"]}))
        .unwrap();
    assert_eq!(result, "c");
}

#[test]
fn test_filter_reverse() {
    let mut engine = common::engine();
    engine
        .add_template("reverse.html", "{{ items | reverse | first }}")
        .unwrap();

    let result = engine
        .render("reverse.html", &json!({"items": ["a", "b", "c"]}))
        .unwrap();
    assert_eq!(result, "c");
}

#[test]
fn test_filter_truncate() {
    let mut engine = common::engine();
    engine
        .add_template("truncate.html", "{{ text | truncate(length=5) }}")
        .unwrap();

    let result = engine
        .render("truncate.html", &json!({"text": "Hello World"}))
        .unwrap();
    assert_eq!(result, "Hello...");
}

#[test]
fn test_filter_pluralize() {
    let mut engine = common::engine();
    engine
        .add_template("plural.html", "{{ count }} item{{ count | pluralize }}")
        .unwrap();

    let result1 = engine.render("plural.html", &json!({"count": 1})).unwrap();
    assert_eq!(result1, "1 item");

    let result2 = engine.render("plural.html", &json!({"count": 2})).unwrap();
    assert_eq!(result2, "2 items");
}

#[test]
fn test_filter_json_encode() {
    let mut engine = common::engine();
    engine
        .add_template("json.html", "{{ data | json_encode | safe }}")
        .unwrap();

    let result = engine
        .render("json.html", &json!({"data": {"name": "test"}}))
        .unwrap();
    assert_eq!(result, r#"{"name":"test"}"#);
}

#[test]
fn test_case_conversion() {
    let mut engine = common::engine();

    engine
        .add_template("camel.html", "{{ name | camel_case }}")
        .unwrap();
    let result = engine
        .render("camel.html", &json!({"name": "hello_world"}))
        .unwrap();
    assert_eq!(result, "helloWorld");

    engine
        .add_template("snake.html", "{{ name | snake_case }}")
        .unwrap();
    let result = engine
        .render("snake.html", &json!({"name": "helloWorld"}))
        .unwrap();
    assert_eq!(result, "hello_world");

    engine
        .add_template("kebab.html", "{{ name | kebab_case }}")
        .unwrap();
    let result = engine
        .render("kebab.html", &json!({"name": "helloWorld"}))
        .unwrap();
    assert_eq!(result, "hello-world");
}

#[test]
fn test_conditional_filter() {
    let mut engine = common::engine();
    engine
        .add_template("when.html", "{{ value | default(value=\"fallback\") }}")
        .unwrap();

    let result = engine.render("when.html", &json!({"value": null})).unwrap();
    assert_eq!(result, "fallback");

    let result = engine
        .render("when.html", &json!({"value": "actual"}))
        .unwrap();
    assert_eq!(result, "actual");
}

#[test]
fn test_if_condition() {
    let mut engine = common::engine();
    engine
        .add_template("if.html", "{% if show %}visible{% endif %}")
        .unwrap();

    let result = engine.render("if.html", &json!({"show": true})).unwrap();
    assert_eq!(result, "visible");

    let result = engine.render("if.html", &json!({"show": false})).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_for_loop() {
    let mut engine = common::engine();
    engine
        .add_template("for.html", "{% for item in items %}{{ item }},{% endfor %}")
        .unwrap();

    let result = engine
        .render("for.html", &json!({"items": ["a", "b", "c"]}))
        .unwrap();
    assert_eq!(result, "a,b,c,");
}

#[test]
fn test_for_loop_with_index() {
    let mut engine = common::engine();
    engine.add_template("for_index.html", "{% for item in items %}{{ loop.index }}:{{ item }}{% if not loop.last %},{% endif %}{% endfor %}").unwrap();

    let result = engine
        .render("for_index.html", &json!({"items": ["a", "b"]}))
        .unwrap();
    assert_eq!(result, "1:a,2:b");
}
