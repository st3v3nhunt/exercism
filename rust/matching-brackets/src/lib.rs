pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in string.chars() {
        // println!("char: {}, vec: {:?}", c, stack);
        match c {
            // push the closing bracket so pop comparison is simpler
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            ']' | '}' | ')' if stack.pop() != Some(c) => return false,
            _ => (),
        }
    }

    stack.is_empty()
}
