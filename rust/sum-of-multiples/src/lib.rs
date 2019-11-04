pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .fold(Vec::new(), |results, number| {
            if factors.iter().any(|factor| {
                if *factor == 0 {
                    return false;
                };
                is_multiple_of(&number, &factor)
            }) {
                return [results, vec![number]].concat();
            }
            results
        })
        .iter()
        .fold(0, |acc, x| acc + x)
}

fn is_multiple_of(number: &u32, factor: &u32) -> bool {
    number % factor == 0
}
