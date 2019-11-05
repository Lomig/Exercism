pub fn is_armstrong_number(num: u32) -> bool {
    let digits = to_digits(&num);
    let power = digits.len() as u32;

    num == digits.iter().map(|i| i.pow(power)).sum()
}

fn to_digits(num: &u32) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|character| character.to_digit(10).unwrap())
        .collect()
}
