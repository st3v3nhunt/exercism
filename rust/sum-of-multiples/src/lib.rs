pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // unimplemented!(
    //     "Sum the multiples of all of {:?} which are less than {}",
    //     factors,
    //     limit
    // )
    // factors.into_iter().filter()
    let mut sum = vec![];
    let mut i = 1;

    for f in factors.iter() {
        let mut ff = *f;
        if ff == 0 {
            break;
        }
        while ff < limit {
            sum.push(ff);
            // sum = sum + ff;
            i = i + 1;
            ff = f * i;
        }
        i = 0;
    }

    // println!("{:?}", sum);
    sum.sort();
    sum.dedup();
    sum.into_iter().sum()
}
