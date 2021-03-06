use std::collections::HashMap;
use std::collections::HashSet;

/// Returns a set of anagrams of word in a vector of possible anagrams.
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lexicon = lower(word).fold(HashMap::new(), |mut result, character| {
        let count = result.entry(character).or_insert(0);
        *count += 1;
        result
    });

    possible_anagrams
        .into_iter()
        .filter_map(|possible_anagram| {
            if word.len() != possible_anagram.len() {
                return None;
            };
            if lower(possible_anagram).eq(lower(word)) {
                return None;
            };

            let mut current_lexicon = lexicon.clone();

            for character in lower(possible_anagram) {
                let count = current_lexicon.entry(character).or_insert(0);

                if *count == 0 {
                    return None;
                }
                *count -= 1;
            }

            Some(*possible_anagram)
        })
        .collect()
}

/// Return a lowercase String from any &str.
fn lower(word: &str) -> impl Iterator<Item = char> {
    word.chars()
        .flat_map(|c| c.to_lowercase())
        .collect::<Vec<char>>()
        .into_iter()
}
