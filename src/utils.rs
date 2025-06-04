use serde::Serialize;
use serde_json::{Value, to_value};
use std::collections::HashMap;

pub(crate) fn struct_to_map<T: Serialize>(input: &T) -> Option<HashMap<String, String>> {
    match to_value(input).ok()? {
        Value::Object(map) => {
            let string_map = map
                .into_iter()
                .filter_map(|(k, v)| {
                    let value_str = match v {
                        Value::String(s) => s,
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null | Value::Array(_) | Value::Object(_) => return None,
                    };
                    Some((k, value_str))
                })
                .collect();
            Some(string_map)
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Array(_) => {
            None
        }
    }
}
