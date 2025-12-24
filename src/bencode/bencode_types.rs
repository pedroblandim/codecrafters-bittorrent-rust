use std::collections::HashMap;

use serde_json::{Map, Number, Value};

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

    pub fn deserialize_from_json(value: &Value) -> Option<Self> {
        match value {
            Value::Number(number) => Some(BencodeTypes::Integer(number.as_i64().unwrap())),
            Value::String(string) => Some(BencodeTypes::ByteString(string.clone().into_bytes())),

            Value::Array(elements) => {
                let deserialized_elements = elements
                    .iter()
                    .map(|e| BencodeTypes::deserialize_from_json(e))
                    .filter(|o| o.is_some())
                    .map(|o| o.unwrap())
                    .collect::<Vec<BencodeTypes>>();
                Some(BencodeTypes::List(deserialized_elements))
            }
            Value::Object(dict) => {
                let mut deserialized_entries_map: HashMap<Vec<u8>, BencodeTypes> = HashMap::new();

                for (k, v) in dict.iter() {
                    let k_bytes = k.clone().into_bytes();
                    let v_bytes = BencodeTypes::deserialize_from_json(v).unwrap();
                    deserialized_entries_map.insert(k_bytes, v_bytes);
                }

                Some(BencodeTypes::Dictionary(deserialized_entries_map))
            }
            _ => None,
        }
    }
}
