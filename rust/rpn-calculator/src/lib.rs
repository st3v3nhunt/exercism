#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    for input in inputs.iter() {
        match input {
            CalculatorInput::Value(number) => stack.push(*number),
            _ => {
                if stack.len() < 2 {
                    return None;
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();

                match input {
                    CalculatorInput::Add => stack.push(a + b),
                    CalculatorInput::Subtract => stack.push(b - a),
                    CalculatorInput::Multiply => stack.push(b * a),
                    CalculatorInput::Divide => stack.push(b / a),
                    _ => return None,
                }
            }
        }
    }

    if stack.len() == 1 {
        Some(stack[0])
    } else {
        None
    }
}
