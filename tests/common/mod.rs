use atom_engine::{Engine, Value};
use indexmap::IndexMap;

pub fn engine() -> Engine {
    Engine::new()
}

pub fn val(v: Value) -> Value {
    v
}

pub fn obj(pairs: Vec<(&str, Value)>) -> Value {
    let mut map = IndexMap::new();
    for (k, v) in pairs {
        map.insert(k.to_string(), v);
    }
    Value::Object(map)
}

pub fn arr(items: Vec<Value>) -> Value {
    Value::Array(items)
}
