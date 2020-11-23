pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz(n / 2).map(|x| x + 1),
        n => collatz(n * 3 + 1).map(|x| x + 1),
    }
    // let mut x = n;
    // let mut acc = 0;

    // while x != 1 {
    //     acc += 1;
    //     if x % 2 == 0 {
    //         x = x / 2;
    //     } else {
    //         x = x * 3 + 1
    //     }
    // }
    // Some(acc)
}
