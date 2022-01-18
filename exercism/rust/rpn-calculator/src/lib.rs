#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut values = Vec::new();
    let mut result = 0;
    if inputs.is_empty() {
        return None;
    }
    for input in inputs {
        match input {
            CalculatorInput::Add => {
                if values.len() < 2 {
                    return None;
                }
                result = values.pop()? + values.pop()?;
                values.push(result);
            }
            CalculatorInput::Subtract => {
                if values.len() < 2 {
                    return None;
                }
                let y = values.pop()?;
                let x = values.pop()?;
                result = x - y;
                values.push(result);
            }
            CalculatorInput::Multiply => {
                if values.len() < 2 {
                    return None;
                }
                result = values.pop()? * values.pop()?;
                values.push(result);
            }
            CalculatorInput::Divide => {
                if values.len() < 2 {
                    return None;
                }
                let y = values.pop()?;
                let x = values.pop()?;
                result = x / y;
                values.push(result);
            }
            CalculatorInput::Value(value) => {
                values.push(*value);
            }
        }
    }
    if values.len() > 1 {
        None
    } else if values.len() == 1 {
        values.pop()
    } else {
        Some(result)
    }
}

pub fn evaluate2(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.len() == 0 {
        return None;
    }
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(number) => stack.push(*number),
            _ => {
                if stack.len() < 2 {
                    return None;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                match input {
                    CalculatorInput::Add => stack.push(a + b),
                    CalculatorInput::Multiply => stack.push(a * b),
                    CalculatorInput::Subtract => stack.push(a - b),
                    CalculatorInput::Divide => stack.push(a / b),
                    _ => return None
                }
            }
        }
    }
    if stack.len() != 1 {
        return None;
    }
    return stack.pop()
}

