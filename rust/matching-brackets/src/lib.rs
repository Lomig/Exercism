#[macro_use]
extern crate lazy_static;

use regex::Regex;

pub fn brackets_are_balanced(string: &str) -> bool {
    let only_brackets = Regex::new(r"[^\(\)\{\}\[\]]+").unwrap();
    let string_with_brackets_only = only_brackets.replace_all(string, "").to_string();

    (rec_brackets(string_with_brackets_only)).is_empty()
}

fn rec_brackets(string: String) -> String {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(\(\)|\{\}|\[\])").unwrap();
    }

    let str_from_string: &str = &string;
    let curated_string = REGEX.replace_all(str_from_string, "");
    match curated_string == string {
        true => return string,
        false => rec_brackets(curated_string.to_string()),
    }
}
