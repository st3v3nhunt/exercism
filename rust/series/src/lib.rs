pub fn series(digits: &str, len: usize) -> Vec<String> {
    (0..(digits.len() + 1 - len))
        .map(|x| digits.chars().skip(x).take(len).collect())
        .collect()
}
