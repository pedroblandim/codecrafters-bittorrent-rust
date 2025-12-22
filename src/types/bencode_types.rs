use serde_json::{Number, Value};

pub enum BencodeTypes {
    Integer(isize),
    ByteString(String),
    List(Vec<BencodeTypes>),
}

pub trait ToJSON {
    fn serialize(&self) -> Value;
}

impl ToJSON for BencodeTypes {
    fn serialize(&self) -> Value {
        match self {
            Self::Integer(number) => serde_json::Value::Number(Number::from(*number)),
            Self::ByteString(string) => serde_json::Value::String(string.clone()),
            Self::List(elements) => {
                let elements_json_array = elements.iter().map(|e| e.serialize()).collect();
                serde_json::Value::Array(elements_json_array)
            }
        }
    }
}

impl BencodeTypes {
    pub fn parse(encoded_value: &str) -> Self {
        parse(encoded_value).unwrap().0
    }
}

fn parse(encoded_value: &str) -> Option<(BencodeTypes, usize)> {
    let first_char = encoded_value.chars().next().unwrap();

    if first_char.is_ascii_digit() {
        // ex.: 5:hello -> hello
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<usize>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number];

        let value = BencodeTypes::ByteString(string.to_string());
        let len = number_string.len() + string.len() + 1; // + 1 for the ':' separator

        Some((value, len))
    } else if first_char == 'i' {
        // ex.: i42e -> 42

        let first_e_index = encoded_value.find("e").unwrap();

        let number_string = encoded_value.get(1..first_e_index).unwrap().to_owned();
        let number = number_string.parse::<isize>().unwrap();

        let value = BencodeTypes::Integer(number);
        let len = number_string.len() + 2; // + 2 for the 'i' and 'e' separators

        Some((value, len))
    } else if first_char == 'l' {
        // ex.: li42ee -> [42]
        let mut elements_string_option = encoded_value.get(1..); // removes the 'l'
        let mut values = vec![];
        let mut total_len = 1; // count the removed 'l'

        loop {
            let Some(elements_string) = elements_string_option else {
                break;
            };

            let Some((value, len)) = parse(elements_string) else {
                break;
            };

            values.push(value);
            total_len += len;

            elements_string_option = elements_string.get(len..);
        }

        Some((BencodeTypes::List(values), total_len))
    } else if first_char == 'e' {
        None
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}
