/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_ascii_lowercase()
        .chars()
        .filter(|character| character.is_alphanumeric())
        .map(|character| {
            if character.is_alphabetic() {
                let int_value = character as u8;

                // a = 97, z = 122
                // reverse letter = z - (letter - a)
                // reverse letter = z + a - letter = 219 - letter
                return (219 - int_value) as char;
            };

            character
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .fold(vec![], |result: Vec<String>, chunk| {
            [result, vec![chunk.into_iter().collect()]].concat()
        })
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|character| character.is_alphanumeric())
        .map(|character| {
            if character.is_alphabetic() {
                let int_value = character as u8;
                return (219 - int_value) as char;
            }

            character
        })
        .collect()
}
