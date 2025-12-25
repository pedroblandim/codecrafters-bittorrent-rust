use std::collections::HashMap;

use serde_json::{Map, Number, Value};

#[derive(Debug, Clone)]
pub enum BencodeTypes {
    Integer(i64),
    ByteString(Vec<u8>),
    List(Vec<BencodeTypes>),
    Dictionary(HashMap<Vec<u8>, BencodeTypes>),
}

impl BencodeTypes {
    pub fn serialize_to_json(&self) -> Value {
        match self {
            Self::Integer(number) => Value::Number(Number::from(*number)),
            Self::ByteString(string) => Value::String(String::from_utf8_lossy(string).to_string()),
            Self::List(elements) => {
                let elements_json_array = elements.iter().map(|e| e.serialize_to_json()).collect();
                Value::Array(elements_json_array)
            }
            Self::Dictionary(elements) => {
                let mut elements_map: Map<String, Value> = Map::new();

                elements.iter().for_each(|(k, v)| {
                    let k_string = String::from_utf8_lossy(k).to_string();
                    elements_map.insert(k_string, v.serialize_to_json());
                });

                Value::Object(elements_map)
            }
        }
    }
}
