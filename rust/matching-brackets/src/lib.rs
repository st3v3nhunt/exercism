pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = vec![];

    for c in string.chars() {
        println!("char: {}, vec: {:?}", c, brackets);
        match c {
            '[' => brackets.push(c),
            '{' => brackets.push(c),
            '(' => brackets.push(c),
            ']' => {
                if brackets.len() > 0 {
                    let a = brackets.pop().unwrap();
                    if a != '[' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            '}' => {
                if brackets.len() > 0 {
                    let a = brackets.pop().unwrap();
                    if a != '{' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            ')' => {
                if brackets.len() > 0 {
                    let a = brackets.pop().unwrap();
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

    brackets.is_empty()
}
