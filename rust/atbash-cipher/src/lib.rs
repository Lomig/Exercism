/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    inverse_message(plain)
        .chunks(5)
        .fold(vec![], |result: Vec<String>, chunk| {
            [result, vec![chunk.into_iter().collect()]].concat()
        })
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    inverse_message(cipher)
        .into_iter()
        .collect()
}

/// Utility function to inverse alphabet
fn inverse_message(message: &str) -> Vec<char> {
    message
        .to_ascii_lowercase()
        .chars()
        .filter(|character| character.is_alphanumeric())
        .map(|character| {
            if character.is_alphabetic() {
                let int_value = character as u8;
                return (b'a' + b'z' - int_value) as char;
            };

            character
        })
        .collect::<Vec<char>>()
}
