pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|x| is_prime(*x))
        .nth(n as usize)
        .unwrap()
}

fn is_prime(n: u32) -> bool {
    let square = 1 + (n as f32).sqrt() as u32;

    match n {
        2 => true,
        _ => !(2..square).any(|x| n % x == 0),
    }
}
