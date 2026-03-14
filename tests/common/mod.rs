use atom_engine::Atom;
use serde_json::json;

pub fn engine() -> Atom {
    Atom::new()
}

pub fn val(v: serde_json::Value) -> serde_json::Value {
    v
}

pub fn obj(pairs: Vec<(&str, serde_json::Value)>) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (k, v) in pairs {
        map.insert(k.to_string(), v);
    }
    serde_json::Value::Object(map)
}

pub fn arr(items: Vec<serde_json::Value>) -> serde_json::Value {
    serde_json::Value::Array(items)
}
