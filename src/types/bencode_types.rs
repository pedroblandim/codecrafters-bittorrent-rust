use std::str::FromStr;

pub enum BencodeTypes {
    Integer(isize),
    ByteString(String),
}

impl FromStr for BencodeTypes {
    type Err = ();

    fn from_str(encoded_value: &str) -> Result<Self, Self::Err> {
        let first_char = encoded_value.chars().next().unwrap();

        // If encoded_value starts with a digit, it's a number
        if first_char.is_ascii_digit() {
            // Example: "5:hello" -> "hello"
            let colon_index = encoded_value.find(':').unwrap();
            let number_string = &encoded_value[..colon_index];
            let number = number_string.parse::<usize>().unwrap();
            let string = &encoded_value[colon_index + 1..colon_index + 1 + number];
            return Ok(BencodeTypes::ByteString(string.to_string()));
        } else if first_char == 'i' {
            // ex.: i42e
            let len = encoded_value.len();
            let number_string = encoded_value.get(1..len - 1).unwrap(); // removes 'i' and 'e'

            return Ok(BencodeTypes::Integer(
                number_string.parse::<isize>().unwrap(),
            ));
        } else {
            panic!("Unhandled encoded value: {}", encoded_value)
        }
    }
}
