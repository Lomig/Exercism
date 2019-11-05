pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factor_list = vec![];
    let square = 1 + (n as f64).sqrt() as u64;

    let mut potential_factors = 2..;
    let mut potential_factor = potential_factors.next().unwrap();

    // To speed up the algorithm, don't check for factors greater than the square root of n
    // It can be a problem if the factors include a large prime
    // We deal with that afterwards in a pattern match.
    while potential_factor <= square {
        while n % potential_factor == 0 {
            factor_list.push(potential_factor);
            n = n / potential_factor;
        }

        potential_factor = potential_factors.next().unwrap();
    }

    match n {
        0 | 1 => factor_list,
        _ => [factor_list, vec![n]].concat(),
    }
}
