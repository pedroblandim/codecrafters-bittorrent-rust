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
    pub fn parse(encoded_value: &str) -> Vec<Self> {
        if encoded_value.is_empty() {
            return vec![];
        }

        let first_char = encoded_value.chars().next().unwrap();

        if first_char.is_ascii_digit() {
            // Example: "5:hello" -> "hello"
            let colon_index = encoded_value.find(':').unwrap();
            let number_string = &encoded_value[..colon_index];
            let number = number_string.parse::<usize>().unwrap();
            let string = &encoded_value[colon_index + 1..colon_index + 1 + number];

            let first_value = BencodeTypes::ByteString(string.to_string());

            let rest_start_index = colon_index + 1 + number;

            parse_with_rest(first_value, encoded_value, rest_start_index)
        } else if first_char == 'i' {
            // ex.: i42e

            let first_e_index = encoded_value.find("e").unwrap();

            let number_string = encoded_value.get(1..first_e_index).unwrap().to_owned();
            let number = number_string.parse::<isize>().unwrap();

            let first_value = BencodeTypes::Integer(number);

            parse_with_rest(first_value, encoded_value, first_e_index + 1)
        } else if first_char == 'l' {
            // ex.: li42ee

            let len = encoded_value.len();
            let elements_string = encoded_value.get(1..len - 1).unwrap(); // removes the 'l' and 'e'
            let elements = BencodeTypes::parse(elements_string);

            return vec![BencodeTypes::List(elements)];
        } else {
            panic!("Unhandled encoded value: {}", encoded_value)
        }
    }
}

fn parse_with_rest(
    first_value: BencodeTypes,
    encoded_value: &str,
    rest_start_index: usize,
) -> Vec<BencodeTypes> {
    let mut values = vec![first_value];

    if rest_start_index > encoded_value.len() - 1 {
        // return only the first value if there is no rest
        return values;
    }

    let rest_string = &encoded_value[rest_start_index..];
    let mut rest_of_the_values: Vec<BencodeTypes> = BencodeTypes::parse(rest_string);

    values.append(&mut rest_of_the_values);

    values
}
