pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    };

    let song: Vec<String> = list
        .windows(2)
        .map(|tuple| { format!("For want of a {} the {} was lost.", tuple[0], tuple[1]) })
        .collect::<Vec<String>>();

    [
        song,
        vec![format!("And all for the want of a {}.", list[0])],
    ]
    .concat()
    .join("\n")
}
