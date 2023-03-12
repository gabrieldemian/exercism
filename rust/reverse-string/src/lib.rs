pub fn reverse(input: &str) -> String {
    let mut reversed = "".to_string();

    input
        .chars()
        .rev()
        .for_each(|c| reversed.push_str(&c.to_string()));

    reversed
}
