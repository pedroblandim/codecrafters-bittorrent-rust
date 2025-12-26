use std::{collections::HashMap, panic};

use serde_json::{Map, Number, Value};

#[derive(Debug, Clone)]
pub enum BencodeTypes {
    Integer(i64),
    ByteString(Vec<u8>),
    List(Vec<BencodeTypes>),
    Dictionary(HashMap<Vec<u8>, BencodeTypes>),
}

impl BencodeTypes {
    pub fn get_byte_string(&self, key: &str) -> Vec<u8> {
        let key_bytes = key.as_bytes().to_vec();
        let bencode_type = self.get(&key_bytes);

        match bencode_type {
            BencodeTypes::ByteString(string) => string.to_owned(),
            BencodeTypes::Integer(_) => panic!("Key {key} maps to an Integer"),
            BencodeTypes::List(_) => panic!("Key {key} maps to a List"),
            BencodeTypes::Dictionary(_) => panic!("Key {key} maps to a Dictionary"),
        }
    }

    pub fn get_integer(&self, key: &str) -> i64 {
        let key_bytes = key.as_bytes().to_vec();
        let bencode_type = self.get(&key_bytes);

        match bencode_type {
            BencodeTypes::ByteString(_) => panic!("Key {key} maps to an ByteString"),
            BencodeTypes::Integer(number) => *number,
            BencodeTypes::List(_) => panic!("Key {key} maps to a List"),
            BencodeTypes::Dictionary(_) => panic!("Key {key} maps to a Dictionary"),
        }
    }

    pub fn get_dict(&self, key: &str) -> &BencodeTypes {
        let key_bytes = key.as_bytes().to_vec();
        let bencode_type = self.get(&key_bytes);

        match bencode_type {
            BencodeTypes::ByteString(_) => panic!("Key {key} maps to an ByteString"),
            BencodeTypes::Integer(_) => panic!("Key {key} maps to an Integer"),
            BencodeTypes::List(_) => panic!("Key {key} maps to a List"),
            BencodeTypes::Dictionary(_) => (),
        }

        bencode_type
    }

    #[allow(dead_code)]
    pub fn get_list(&self, key: &str) -> &BencodeTypes {
        let key_bytes = key.as_bytes().to_vec();
        let bencode_type = self.get(&key_bytes);

        match bencode_type {
            BencodeTypes::ByteString(_) => panic!("Key {key} maps to an ByteString"),
            BencodeTypes::Integer(_) => panic!("Key {key} maps to an Integer"),
            BencodeTypes::List(_) => (),
            BencodeTypes::Dictionary(_) => panic!("Key {key} maps to a Dictionary"),
        }

        bencode_type
    }

    fn get(&self, key: &Vec<u8>) -> &BencodeTypes {
        match &self {
            BencodeTypes::Dictionary(map) => map.get(key).expect("Key {key} not found"),
            _ => panic!("Calling get on a BencodeType that is not a Dictionary"),
        }
    }

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
