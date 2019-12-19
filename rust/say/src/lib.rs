use std::process::Command;

pub fn encode(n: u64) -> String {
    let result = number_to_text(n).replace("-zero", "").replace(" zero", "");

    let _output = Command::new("say").arg(result.clone()).status();

    result
}

fn number_to_text(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20..=99 => medium_numbers(n),
        100..=999 => big_numbers(n, 2, "hundred"),
        1_000..=999_999 => big_numbers(n, 3, "thousand"),
        1_000_000..=999_999_999 => big_numbers(n, 6, "million"),
        1_000_000_000..=999_999_999_999 => big_numbers(n, 9, "billion"),
        1_000_000_000_000..=999_999_999_999_999 => big_numbers(n, 12, "trillion"),
        1_000_000_000_000_000..=999_999_999_999_999_999 => big_numbers(n, 15, "quadrillion"),
        1_000_000_000_000_000_000..=std::u64::MAX => big_numbers(n, 18, "quintillion"),
    }
}

fn medium_numbers(n: u64) -> String {
    let tens = n / 10;
    let units = n % 10;

    let tens_to_string = match tens {
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => panic!("Incorrect Number!"),
    };

    format!("{}-{}", tens_to_string, number_to_text(units))
}

fn big_numbers(n: u64, divider: u32, name: &str) -> String {
    let division = n / 10u64.pow(divider);
    let rest = n % 10u64.pow(divider);

    format!(
        "{} {} {}",
        number_to_text(division),
        name,
        number_to_text(rest)
    )
}
