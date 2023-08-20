use serde_json::{self, Map, Value};

pub fn json_insert(json: &mut Value, field: &str, value: Value) {
    if let Some(obj) = json.as_object_mut() {
        obj.insert(field.to_string(), value);
    }
}

pub fn json_merge(json1: Value, json2: Value) -> Value {
    if let (Value::Object(map1), Value::Object(map2)) = (json1.clone(), json2.clone()) {
        let merged_map = merge_json_objects(map1.clone(), map2.clone());
        Value::Object(merged_map)
    } else {
        ().into()
    }
}
pub fn merge_json_objects(
    mut base: Map<String, Value>,
    other: Map<String, Value>,
) -> Map<String, Value> {
    base.extend(other);
    base
}
