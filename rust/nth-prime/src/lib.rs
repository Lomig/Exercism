pub fn nth(n: u32) -> u32 {
    let mut current_nth = 0;
    let mut current_prime = 2;

    while current_nth < n {
        current_prime = seek_next_prime(&current_prime);
        current_nth += 1;
    }

    current_prime
}

fn seek_next_prime(number: &u32) -> u32 {
    let mut prime = false;
    let mut candidate = *number;

    while !prime {
        candidate += 1;
        prime = is_prime(&candidate);
    }

    candidate
}

fn is_prime(n: &u32) -> bool {
    let square = 1 + (*n as f32).sqrt() as u32;

    match *n {
        2 => true,
        _ => !(2..square).any(|x| *n % x == 0),
    }
}
