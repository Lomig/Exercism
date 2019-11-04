pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|number| {
            factors
                .iter()
                .filter(|factor| *factor > &0)
                .any(|factor| is_multiple_of(&number, &factor))
        })
        .fold(0, |acc, x| acc + x)
}

fn is_multiple_of(number: &u32, factor: &u32) -> bool {
    number % factor == 0
}
