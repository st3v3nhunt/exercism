pub fn square(s: u32) -> u64 {
    match s {
        0 => panic!("Square must be between 1 and 64"),
        1..=64 => (2 as u64).pow(s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..=64).into_iter().fold(0, |acc, x| acc + square(x))
}
