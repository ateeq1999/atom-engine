mod common;

use atom_engine::Value;
use common::engine;
use common::{obj, val};

#[test]
fn test_each_array() {
    let engine = engine();
    let items = vec!["a".into(), "b".into(), "c".into()];
    let result = engine
        .render_raw(
            "@each(item in items) { {{ item }} } @endeach",
            val(obj(vec![("items", Value::Array(items))])),
        )
        .unwrap();
    assert_eq!(result, " a  b  c ");
}

#[test]
fn test_each_with_index() {
    let engine = engine();
    let items = vec!["a".into(), "b".into()];
    let result = engine
        .render_raw(
            "@each(item in items) { {{ $index }}:{{ item }} } @endeach",
            val(obj(vec![("items", Value::Array(items))])),
        )
        .unwrap();
    assert_eq!(result, " 0:a 1:b ");
}

#[test]
fn test_each_loop_vars() {
    let engine = engine();
    let items = vec!["a".into(), "b".into()];
    let result = engine
        .render_raw(
            "@each(item in items) { {{ $first }}-{{ $last }}-{{ $total }} } @endeach",
            val(obj(vec![("items", Value::Array(items))])),
        )
        .unwrap();
    assert_eq!(result, " true-false-2 false-true-2 ");
}

#[test]
fn test_each_empty() {
    let engine = engine();
    let empty_arr: Vec<Value> = vec![];
    let result = engine
        .render_raw(
            "@each(item in items) { {{ item }} } @empty { empty } @endeach",
            val(obj(vec![("items", Value::Array(empty_arr))])),
        )
        .unwrap();
    assert_eq!(result, " empty ");
}
