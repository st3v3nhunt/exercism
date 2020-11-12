use std::convert::TryFrom;

pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    // prime divisble by itself and one
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
    // create a list of primes x
    // using an infinite loop, check each value for prime add to list and end loop when nth found
    let mut list = vec![];
    let mut i = 2;
    while list.len() < usize::try_from(n).unwrap() + 1 {
        println!("hi*********************");
        let mut j = i - 1;
        let mut is_prime = true;
        while j > 1 {
            if i % j == 0 {
                is_prime = false;
                break;
            }
            j -= 1;
        }
        if is_prime {
            list.push(i)
        }
        i += 1;
    }
    println!("list contains: {:?}", list);
    return list[n as usize]; // pop();
}
