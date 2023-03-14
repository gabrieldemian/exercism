pub fn raindrops(n: u32) -> String {
    let mut res = "".to_string();
    [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .for_each(|(x, s)| {
            if n % x == 0 {
                res.push_str(s)
            }
        });
    match res {
        s if !s.is_empty() => s,
        _ => n.to_string(),
    }
}
