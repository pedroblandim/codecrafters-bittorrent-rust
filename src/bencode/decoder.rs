use std::collections::HashMap;

use crate::bencode::bencode_types::BencodeTypes;

impl BencodeTypes {
    pub fn decode(encoded_value: Vec<u8>) -> Self {
        decode(encoded_value).unwrap().0
    }
}

fn decode(encoded_value: Vec<u8>) -> Option<(BencodeTypes, usize)> {
    let first_char = encoded_value.iter().next().unwrap();

    if first_char.is_ascii_digit() {
        // ex.: 5:hello -> hello
        let colon_index = encoded_value.find(':' as u8).unwrap() as usize;

        let number_string_u8 = &encoded_value[..colon_index]; // must be a valid UTF-8 here
        let number_string = str::from_utf8(number_string_u8).unwrap();

        let number = number_string.parse::<usize>().unwrap();

        let string_bytes = &encoded_value[colon_index + 1..colon_index + 1 + number];

        let string_vec_bytes: Vec<u8> = string_bytes.to_vec();

        let value = BencodeTypes::ByteString(string_vec_bytes);
        let len = number_string.len() + string_bytes.len() + 1; // + 1 for the ':' separator

        Some((value, len))
    } else if *first_char == ('i' as u8) {
        // ex.: i42e -> 42

        let first_e_index = encoded_value.find('e' as u8).unwrap() as usize;

        let number_string_u8 = &encoded_value[1..first_e_index]; // must be a valid UTF-8 here
        let number_string = str::from_utf8(number_string_u8).unwrap();

        let number = number_string.parse::<i64>().unwrap();

        let value = BencodeTypes::Integer(number);
        let len = number_string.len() + 2; // + 2 for the 'i' and 'e' separators

        Some((value, len))
    } else if *first_char == ('l' as u8) {
        // ex.: li42ee -> [42]
        let (values, total_len) = parse_elements_from_string(encoded_value);

        Some((BencodeTypes::List(values), total_len))
    } else if *first_char == ('d' as u8) {
        // ex.: d3:foo3:bar5:helloi52ee -> {"hello": 52, "foo":"bar"}
        let (values, total_len) = parse_elements_from_string(encoded_value);

        if values.len() % 2 != 0 {
            panic!("Invalid encoded value")
        }

        let mut keys: Vec<Vec<u8>> = vec![];
        let mut dict_values: Vec<BencodeTypes> = vec![];

        for (idx, element) in values.into_iter().enumerate() {
            if idx % 2 == 0 {
                // is key
                match element {
                    BencodeTypes::ByteString(string) => keys.push(string.to_owned()),
                    _ => {
                        let m = format!("Invalid type for dictionary key");
                        panic!("{m}");
                    }
                }
            } else {
                dict_values.push(element);
            }
        }

        let mut dict: HashMap<Vec<u8>, BencodeTypes> = HashMap::new();
        for (k, v) in keys.iter().zip(dict_values.into_iter()) {
            dict.insert(k.clone(), v);
        }

        Some((BencodeTypes::Dictionary(dict), total_len))
    } else if *first_char == ('e' as u8) {
        None
    } else {
        panic!(
            "Unhandled encoded value: {}",
            String::from_utf8_lossy(&encoded_value)
        )
    }
}

fn parse_elements_from_string(encoded_value: Vec<u8>) -> (Vec<BencodeTypes>, usize) {
    // should be used with lists or dictionaries
    // ex.: d3:foo3:bar5:helloi52ee -> {"hello": 52, "foo":"bar"}
    // ex.: li42ee -> [42]

    let mut elements_string_option = encoded_value.get(1..); // removes the 'l' or 'd' at the start
    let mut values = vec![];
    let mut total_len = 2; // count the removed 'l'/'d' at the start and the 'e' at the end

    loop {
        let Some(elements_string) = elements_string_option else {
            break;
        };

        let Some((value, len)) = decode(elements_string.to_vec()) else {
            break;
        };

        values.push(value);
        total_len += len;

        elements_string_option = elements_string.get(len..); // go to next element on the list
    }

    (values, total_len)
}

impl Find for Vec<u8> {
    fn find(&self, c: u8) -> Option<u8> {
        let mut result: Option<u8> = None;

        let mut idx = 0;

        self.iter().for_each(|u| {
            if c == *u && result.is_none() {
                result = Some(idx);
            }

            idx += 1;
        });

        result
    }
}

trait Find {
    fn find(&self, c: u8) -> Option<u8>;
}
