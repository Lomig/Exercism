extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // Non UTF8 / grapheme characters
    // return input.chars().rev().collect();
    return input.graphemes(true).rev().collect();
}
