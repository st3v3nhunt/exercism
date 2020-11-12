pub fn factors(n: u64) -> Vec<u64> {
    // unimplemented!("This should calculate the prime factors of {}", n)
    let mut p_factors: Vec<u64> = Vec::new();
    let mut next = n;
    let mut f = 2;

    while next != 1 {
        if next % f == 0 {
            p_factors.push(f);
            next = next / f;
            f = 2
        } else {
            f = f + 1;
        }
    }
    return p_factors;
}
