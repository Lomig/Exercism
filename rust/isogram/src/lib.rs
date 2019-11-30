pub fn check(candidate: &str) -> bool {
    let mut sorted_candidate: Vec<char> = candidate
        .to_ascii_lowercase()
        .chars()
        .filter(|character| character.is_alphanumeric())
        .collect();
    sorted_candidate.sort();

    let mut deduped_candidate = sorted_candidate.clone();
    deduped_candidate.dedup();

    sorted_candidate == deduped_candidate
}
