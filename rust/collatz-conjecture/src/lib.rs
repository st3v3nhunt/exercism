pub fn collatz(n: u64) -> Option<u64> {
    let mut x = n;
    let mut acc = 0;

    while x != 1 {
        acc += 1;
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = x * 3 + 1
        }
    }
    Some(acc)
}
