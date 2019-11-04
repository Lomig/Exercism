pub fn raindrops(n: u32) -> String {
    match (n % 3, n % 5, n % 7) {
        (0, 0, 0) => format!("PlingPlangPlong"),
        (0, 0, _) => format!("PlingPlang"),
        (0, _, 0) => format!("PlingPlong"),
        (0, _, _) => format!("Pling"),
        (_, 0, 0) => format!("PlangPlong"),
        (_, 0, _) => format!("Plang"),
        (_, _, 0) => format!("Plong"),
        _ => format!("{}", n)
    }
}
