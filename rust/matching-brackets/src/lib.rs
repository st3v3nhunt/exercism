pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in string.chars() {
        println!("char: {}, vec: {:?}", c, stack);
        match c {
            '[' | '{' | '(' => stack.push(c),
            ']' => {
                if stack.len() > 0 {
                    let a = stack.pop().unwrap();
                    if a != '[' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            '}' => {
                if stack.len() > 0 {
                    let a = stack.pop().unwrap();
                    if a != '{' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            ')' => {
                if stack.len() > 0 {
                    let a = stack.pop().unwrap();
                    if a != '(' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}
