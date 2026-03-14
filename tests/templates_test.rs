use serde_json::json;

#[test]
fn test_variables() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ name | upper }}")
        .unwrap();
    let result = engine
        .render("test.html", &json!({"name": "hello"}))
        .unwrap();
    assert_eq!(result, "HELLO");
}

#[test]
fn test_filters_string() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ 'hello' | upper }}")
        .unwrap();
    let result = engine.render("test.html", &json!({})).unwrap();
    assert_eq!(result, "HELLO");
}

#[test]
fn test_filters_collection() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ items | first }}")
        .unwrap();
    let result = engine
        .render("test.html", &json!({"items": ["a", "b", "c"]}))
        .unwrap();
    assert_eq!(result, "a");
}

#[test]
fn test_filters_each() {
    let mut engine = atom_engine::Atom::new();
    engine.add_template("test.html", "{% for item in items | each(index=true) %}{{ item.index }}:{{ item.value }}{% if not loop.last %},{% endif %}{% endfor %}").unwrap();
    let result = engine
        .render("test.html", &json!({"items": ["a", "b", "c"]}))
        .unwrap();
    assert_eq!(result, "0:a,1:b,2:c");
}

#[test]
fn test_conditionals() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template(
            "test.html",
            "{% if is_active %}active{% else %}inactive{% endif %}",
        )
        .unwrap();
    let result = engine
        .render("test.html", &json!({"is_active": true}))
        .unwrap();
    assert_eq!(result, "active");
}

#[test]
fn test_loops() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{% for item in items %}{{ item }}{% endfor %}")
        .unwrap();
    let result = engine
        .render("test.html", &json!({"items": ["a", "b", "c"]}))
        .unwrap();
    assert_eq!(result, "abc");
}

#[test]
fn test_loop_index() {
    let mut engine = atom_engine::Atom::new();
    engine.add_template("test.html", "{% for item in items %}{{ loop.index }}:{{ item }}{% if not loop.last %},{% endif %}{% endfor %}").unwrap();
    let result = engine
        .render("test.html", &json!({"items": ["a", "b"]}))
        .unwrap();
    assert_eq!(result, "1:a,2:b");
}

#[test]
fn test_number_filters() {
    let mut engine = atom_engine::Atom::new();
    engine.add_template("round", "{{ 3.7 | round }}").unwrap();
    engine.add_template("abs", "{{ -42 | abs }}").unwrap();
    engine.add_template("sum", "{{ numbers | sum }}").unwrap();
    engine.add_template("avg", "{{ numbers | avg }}").unwrap();
    engine.add_template("min", "{{ numbers | min }}").unwrap();
    engine.add_template("max", "{{ numbers | max }}").unwrap();
    engine.add_template("ceil", "{{ 3.1 | ceil }}").unwrap();
    engine.add_template("floor", "{{ 3.9 | floor }}").unwrap();

    assert_eq!(engine.render("round", &json!({})).unwrap(), "4");
    assert_eq!(engine.render("abs", &json!({})).unwrap(), "42");
    assert_eq!(
        engine
            .render("sum", &json!({"numbers": [1, 2, 3]}))
            .unwrap(),
        "6"
    );
    assert_eq!(
        engine
            .render("avg", &json!({"numbers": [1, 2, 3]}))
            .unwrap(),
        "2"
    );
    assert_eq!(
        engine
            .render("min", &json!({"numbers": [3, 1, 2]}))
            .unwrap(),
        "1"
    );
    assert_eq!(
        engine
            .render("max", &json!({"numbers": [1, 2, 3]}))
            .unwrap(),
        "3"
    );
    assert_eq!(engine.render("ceil", &json!({})).unwrap(), "4");
    assert_eq!(engine.render("floor", &json!({})).unwrap(), "3");
}

#[test]
fn test_case_camel() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ 'hello_world' | camel_case }}")
        .unwrap();
    assert_eq!(
        engine.render("test.html", &json!({})).unwrap(),
        "helloWorld"
    );
}

#[test]
fn test_case_snake() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ 'helloWorld' | snake_case }}")
        .unwrap();
    assert_eq!(
        engine.render("test.html", &json!({})).unwrap(),
        "hello_world"
    );
}

#[test]
fn test_case_kebab() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ 'helloWorld' | kebab_case }}")
        .unwrap();
    assert_eq!(
        engine.render("test.html", &json!({})).unwrap(),
        "hello-world"
    );
}

#[test]
fn test_case_title() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ 'hello world' | title }}")
        .unwrap();
    assert_eq!(
        engine.render("test.html", &json!({})).unwrap(),
        "Hello World"
    );
}

#[test]
fn test_case_capitalize() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ 'hello world' | capitalize }}")
        .unwrap();
    assert_eq!(
        engine.render("test.html", &json!({})).unwrap(),
        "Hello world"
    );
}

#[test]
fn test_string_modify_filters() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template(
            "replace",
            "{{ 'Hello World' | replace(old='World', new='Rust') }}",
        )
        .unwrap();
    engine
        .add_template("remove", "{{ 'Hello World' | remove(string='o') }}")
        .unwrap();
    engine
        .add_template("prepend", "{{ 'World' | prepend(string='Hello ') }}")
        .unwrap();
    engine
        .add_template("append", "{{ 'Hello' | append(string=' World') }}")
        .unwrap();
    engine
        .add_template("strip", "{{ '  hello  ' | strip }}")
        .unwrap();
    engine
        .add_template("truncate", "{{ 'Hello World' | truncate(length=5) }}")
        .unwrap();

    assert_eq!(engine.render("replace", &json!({})).unwrap(), "Hello Rust");
    assert_eq!(engine.render("remove", &json!({})).unwrap(), "Hell Wrld");
    assert_eq!(engine.render("prepend", &json!({})).unwrap(), "Hello World");
    assert_eq!(engine.render("append", &json!({})).unwrap(), "Hello World");
    assert_eq!(engine.render("strip", &json!({})).unwrap(), "hello");
    assert_eq!(engine.render("truncate", &json!({})).unwrap(), "Hello...");
}

#[test]
fn test_default_filter() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ value | default(value='fallback') }}")
        .unwrap();

    assert_eq!(
        engine.render("test.html", &json!({"value": null})).unwrap(),
        "fallback"
    );
    assert_eq!(
        engine
            .render("test.html", &json!({"value": "actual"}))
            .unwrap(),
        "actual"
    );
    assert_eq!(engine.render("test.html", &json!({})).unwrap(), "fallback");
}

#[test]
fn test_when_filter() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ is_active | when(then='yes', else='no') }}")
        .unwrap();

    assert_eq!(
        engine
            .render("test.html", &json!({"is_active": true}))
            .unwrap(),
        "yes"
    );
    assert_eq!(
        engine
            .render("test.html", &json!({"is_active": false}))
            .unwrap(),
        "no"
    );
}

#[test]
fn test_uuid() {
    let mut engine = atom_engine::Atom::new();
    engine.add_template("test.html", "{{ uuid() }}").unwrap();
    let result = engine.render("test.html", &json!({})).unwrap();
    assert!(result.len() > 30);
}

#[test]
fn test_random() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ random(min=1, max=10) }}")
        .unwrap();
    let result: i64 = engine
        .render("test.html", &json!({}))
        .unwrap()
        .parse()
        .unwrap();
    assert!(result >= 1 && result <= 10);
}

#[test]
fn test_choice() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ choice(array=['a','b','c']) }}")
        .unwrap();
    let result = engine.render("test.html", &json!({})).unwrap();
    assert!(["a", "b", "c"].contains(&result.as_str()));
}

#[test]
fn test_repeat() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ repeat(content='ab', count=3) }}")
        .unwrap();
    let result = engine.render("test.html", &json!({})).unwrap();
    assert_eq!(result, "ababab");
}

#[test]
fn test_partition() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template(
            "test.html",
            "{{ users | partition(key='active') | json_encode | safe }}",
        )
        .unwrap();
    let result = engine.render(
        "test.html",
        &json!({"users": [{"name": "Alice", "active": true}, {"name": "Bob", "active": false}]})
    ).unwrap();
    assert!(result.contains("Alice"));
    assert!(result.contains("Bob"));
}

#[test]
fn test_json_encode() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ data | json_encode | safe }}")
        .unwrap();
    let result = engine
        .render("test.html", &json!({"data": {"name": "test"}}))
        .unwrap();
    assert_eq!(result, r#"{"name":"test"}"#);
}

#[test]
fn test_safe() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ '<b>bold</b>' | safe }}")
        .unwrap();
    let result = engine.render("test.html", &json!({})).unwrap();
    assert_eq!(result, "<b>bold</b>");
}

#[test]
fn test_date() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ now() | date }}")
        .unwrap();
    let result = engine.render("test.html", &json!({})).unwrap();
    assert!(result.len() > 0);
}

#[test]
fn test_mixed_pipeline() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ [5,3,8,1,9,2,7,4,6] | sort | first }}")
        .unwrap();
    let result = engine.render("test.html", &json!({})).unwrap();
    assert_eq!(result, "1");
}

#[test]
fn test_length() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ items | length }}")
        .unwrap();
    let result = engine
        .render("test.html", &json!({"items": ["a", "b", "c"]}))
        .unwrap();
    assert_eq!(result, "3");
}

#[test]
fn test_last() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ items | last }}")
        .unwrap();
    let result = engine
        .render("test.html", &json!({"items": ["a", "b", "c"]}))
        .unwrap();
    assert_eq!(result, "c");
}

#[test]
fn test_pluralize() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ count }} item{{ count | pluralize }}")
        .unwrap();
    assert_eq!(
        engine.render("test.html", &json!({"count": 1})).unwrap(),
        "1 item"
    );
    assert_eq!(
        engine.render("test.html", &json!({"count": 2})).unwrap(),
        "2 items"
    );
}

#[test]
fn test_slugify() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{{ text | slugify }}")
        .unwrap();
    let result = engine
        .render("test.html", &json!({"text": "Hello World! 123"}))
        .unwrap();
    assert_eq!(result, "hello-world-123");
}

#[test]
fn test_loop_first_last() {
    let mut engine = atom_engine::Atom::new();
    engine.add_template("test.html", "{% for item in items %}{% if loop.first %}FIRST{% endif %}{% if loop.last %}LAST{% endif %}{{ item }}{% endfor %}").unwrap();
    let result = engine
        .render("test.html", &json!({"items": ["a", "b"]}))
        .unwrap();
    assert_eq!(result, "FIRSTaLASTb");
}

#[test]
fn test_if_elif_else() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template(
            "test.html",
            "{% if x == 1 %}one{% elif x == 2 %}two{% else %}other{% endif %}",
        )
        .unwrap();
    assert_eq!(engine.render("test.html", &json!({"x": 1})).unwrap(), "one");
    assert_eq!(engine.render("test.html", &json!({"x": 2})).unwrap(), "two");
    assert_eq!(
        engine.render("test.html", &json!({"x": 3})).unwrap(),
        "other"
    );
}

#[test]
fn test_boolean_operators() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("and", "{% if a and b %}yes{% else %}no{% endif %}")
        .unwrap();
    engine
        .add_template("or", "{% if a or b %}yes{% else %}no{% endif %}")
        .unwrap();
    engine
        .add_template("not", "{% if not a %}yes{% else %}no{% endif %}")
        .unwrap();

    assert_eq!(
        engine
            .render("and", &json!({"a": true, "b": true}))
            .unwrap(),
        "yes"
    );
    assert_eq!(
        engine
            .render("and", &json!({"a": true, "b": false}))
            .unwrap(),
        "no"
    );
    assert_eq!(
        engine
            .render("or", &json!({"a": false, "b": true}))
            .unwrap(),
        "yes"
    );
    assert_eq!(
        engine
            .render("or", &json!({"a": false, "b": false}))
            .unwrap(),
        "no"
    );
    assert_eq!(engine.render("not", &json!({"a": false})).unwrap(), "yes");
    assert_eq!(engine.render("not", &json!({"a": true})).unwrap(), "no");
}

#[test]
fn test_comparison_operators() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("gt", "{% if x > 5 %}yes{% else %}no{% endif %}")
        .unwrap();
    engine
        .add_template("lt", "{% if x < 5 %}yes{% else %}no{% endif %}")
        .unwrap();
    engine
        .add_template("gte", "{% if x >= 5 %}yes{% else %}no{% endif %}")
        .unwrap();
    engine
        .add_template("lte", "{% if x <= 5 %}yes{% else %}no{% endif %}")
        .unwrap();
    engine
        .add_template("eq", "{% if x == 5 %}yes{% else %}no{% endif %}")
        .unwrap();
    engine
        .add_template("ne", "{% if x != 5 %}yes{% else %}no{% endif %}")
        .unwrap();

    assert_eq!(engine.render("gt", &json!({"x": 10})).unwrap(), "yes");
    assert_eq!(engine.render("gt", &json!({"x": 3})).unwrap(), "no");
    assert_eq!(engine.render("lt", &json!({"x": 3})).unwrap(), "yes");
    assert_eq!(engine.render("lt", &json!({"x": 10})).unwrap(), "no");
    assert_eq!(engine.render("gte", &json!({"x": 5})).unwrap(), "yes");
    assert_eq!(engine.render("gte", &json!({"x": 4})).unwrap(), "no");
    assert_eq!(engine.render("lte", &json!({"x": 5})).unwrap(), "yes");
    assert_eq!(engine.render("lte", &json!({"x": 6})).unwrap(), "no");
    assert_eq!(engine.render("eq", &json!({"x": 5})).unwrap(), "yes");
    assert_eq!(engine.render("eq", &json!({"x": 3})).unwrap(), "no");
    assert_eq!(engine.render("ne", &json!({"x": 3})).unwrap(), "yes");
    assert_eq!(engine.render("ne", &json!({"x": 5})).unwrap(), "no");
}

#[test]
fn test_nested_loops() {
    let mut engine = atom_engine::Atom::new();
    engine.add_template("test.html", "{% for outer in outer_items %}{% for inner in inner_items %}{{ outer }}-{{ inner }} {% endfor %}{% endfor %}").unwrap();
    let result = engine
        .render(
            "test.html",
            &json!({"outer_items": ["a", "b"], "inner_items": ["1", "2"]}),
        )
        .unwrap();
    assert!(result.contains("a-1"));
    assert!(result.contains("b-2"));
}

#[test]
fn test_set_variable() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("test.html", "{% set x = 42 %}{{ x }}")
        .unwrap();
    let result = engine.render("test.html", &json!({})).unwrap();
    assert_eq!(result, "42");
}

#[test]
fn test_include() {
    let mut engine = atom_engine::Atom::new();
    engine
        .add_template("header.html", "<header>Header</header>")
        .unwrap();
    engine
        .add_template("test.html", "{% include 'header.html' %}")
        .unwrap();
    let result = engine.render("test.html", &json!({})).unwrap();
    assert!(result.contains("Header"));
}

#[test]
fn test_passthrough_filters() {
    let mut engine = atom_engine::Atom::new();
    engine.add_template("lower", "{{ name | lower }}").unwrap();
    engine.add_template("upper", "{{ name | upper }}").unwrap();
    assert_eq!(
        engine.render("lower", &json!({"name": "HELLO"})).unwrap(),
        "hello"
    );
    assert_eq!(
        engine.render("upper", &json!({"name": "hello"})).unwrap(),
        "HELLO"
    );
}
