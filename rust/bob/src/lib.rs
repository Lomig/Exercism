use regex::Regex;

pub fn reply(message: &str) -> &str {
    let question = Regex::new(r".+\?[[:blank:]]*$").unwrap();
    let shout = Regex::new(r"^[[:^lower:]]*[[:upper:]][[:^lower:]]*+$").unwrap();
    let silence = Regex::new(r"^([[:blank:]]|[[:cntrl:]])*$").unwrap();

    if shout.is_match(message) && !question.is_match(message) {
        "Whoa, chill out!"
    } else if shout.is_match(message) {
        "Calm down, I know what I'm doing!"
    } else if question.is_match(message) {
        "Sure."
    } else if silence.is_match(message) {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}
