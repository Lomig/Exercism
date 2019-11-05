extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // Non UTF8 / grapheme characters
    // input.chars().rev().collect()
    input.graphemes(true).rev().collect()
}
