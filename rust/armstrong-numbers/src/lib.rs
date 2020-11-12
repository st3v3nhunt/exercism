pub fn is_armstrong_number(num: u32) -> bool {
    // true if all of the digits in the number raised to the power of the number of digits added
    // together is equal to the number
    let num_str = num.to_string();
    let power = num_str.len() as u32;

    let r = num_str
        .chars()
        .fold(0, |acc, c| acc + c.to_digit(10).unwrap().pow(power));

    return r == num;
}
