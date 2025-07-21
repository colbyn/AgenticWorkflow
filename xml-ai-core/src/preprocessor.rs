use liquid::{model::{Object, Value}, ParserBuilder};
use serde_json::{Map, Value as JsonValue};

// ————————————————————————————————————————————————————————————————————————————
// INTERNAL
// ————————————————————————————————————————————————————————————————————————————

/// Recursively convert a serde_json::Value to a liquid::Value
fn json_to_liquid_value(value: &JsonValue) -> Value {
    match value {
        JsonValue::Null => Value::Nil,
        JsonValue::Bool(b) => Value::scalar(*b),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::scalar(i)
            } else if let Some(u) = n.as_u64() {
                unimplemented!("TODO: {u}")
            } else if let Some(f) = n.as_f64() {
                Value::scalar(f)
            } else {
                Value::Nil
            }
        }
        JsonValue::String(s) => Value::scalar(s.clone()),
        JsonValue::Array(arr) => {
            Value::Array(arr.iter().map(json_to_liquid_value).collect())
        }
        JsonValue::Object(map) => {
            let mut obj = Object::new();
            for (k, v) in map {
                obj.insert(k.clone().into(), json_to_liquid_value(v));
            }
            Value::Object(obj)
        }
    }
}

/// Convert serde_json::Map<String, Value> directly to a liquid::Object
fn json_map_to_liquid_object(map: &Map<String, JsonValue>) -> Object {
    let mut obj = Object::new();
    for (k, v) in map {
        obj.insert(k.clone().into(), json_to_liquid_value(v));
    }
    obj
}

// ————————————————————————————————————————————————————————————————————————————
// ENTRYPOINT
// ————————————————————————————————————————————————————————————————————————————

pub type JsonObject = Map<String, JsonValue>;

/// Render a Liquid template using a serde_json::Map as globals
pub fn apply_liquid(
    source: impl AsRef<str>,
    globals: &JsonObject
) -> Result<String, liquid::Error> {
    let template_str = source.as_ref();

    let parser = ParserBuilder::with_stdlib().build()?;
    let template = parser.parse(template_str)?;

    let liquid_globals = json_map_to_liquid_object(globals);
    template.render(&liquid_globals)
}


// pub fn apply_liquid(source: impl AsRef<str>, globals: &dyn liquid::ObjectView) {
//     let source = source.as_ref();
//     let source = liquid::ParserBuilder::with_stdlib()
//         .build()
//         .unwrap()
//         .parse(&source)
//         .unwrap();
//     let source = source.render(&globals).unwrap();
// }
