pub fn factors(n: u64) -> Vec<u64> {
    match n {
        0 | 1 => vec![],
        _ => {
            let square = 1 + (n as f64).sqrt() as u64;

            let (list_of_factors, remaining) = (2..=square)
                .fold((vec![], n), |(factor_list, remaining), x| {
                    add_factor(&factor_list, &remaining, &x)
                });

            // if the number include a large prime,
            // the square_root optimization prevents the function to catch it.
            match remaining {
                0 | 1 => list_of_factors,
                _ => [list_of_factors, vec![remaining]].concat(),
            }
        }
    }
}

fn add_factor(factor_list: &Vec<u64>, n: &u64, potential_factor: &u64) -> (Vec<u64>, u64) {
    let mut current_factors = vec![];
    let mut factor_list = factor_list.clone();
    let mut remaining = *n;

    while remaining % potential_factor == 0 {
        current_factors.push(*potential_factor);
        remaining = remaining / potential_factor;
    }
    factor_list.extend(&current_factors);
    (factor_list, remaining)
}
