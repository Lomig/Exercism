pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    };

    let song: Vec<String> = list
        .iter()
        .enumerate()
        .fold(vec![], |verses, (index, word)| {
            if index == 0 {
                return verses;
            };

            [
                verses,
                vec![format!("For want of a {} the {} was lost.", list[index - 1], word)],
            ]
            .concat()
        });

    [
        song,
        vec![format!("And all for the want of a {}.", list[0])],
    ]
    .concat()
    .join("\n")
}
