use std::collections::HashSet;

/// Returns a set of anagrams of word in a vector of possible anagrams.
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = lower(word);
    let mut sorted_lower_word = lower_word.clone();
    sorted_lower_word.sort();

    possible_anagrams
        .into_iter()
        .fold(HashSet::new(), |mut result, possible_anagram| {
            let lower_anagram = lower(*possible_anagram);
            let mut sorted_lower_anagram = lower_anagram.clone();
            sorted_lower_anagram.sort();

            if lower_anagram != lower_word && sorted_lower_word == sorted_lower_anagram {
                result.insert(*possible_anagram);
            }

            result
        })
}

/// Return a lowercase String from any &str.
fn lower(word: &str) -> Vec<String> {
    word.chars()
        .map(|c| c.to_lowercase().to_string())
        .collect::<Vec<String>>()
}
