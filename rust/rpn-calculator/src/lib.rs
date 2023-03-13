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
    for input in inputs {
        match input {
            CalculatorInput::Add => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a + b);
            }
            CalculatorInput::Subtract => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b - a);
            }
            CalculatorInput::Multiply => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a * b);
            }
            CalculatorInput::Divide => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b / a);
            }
            CalculatorInput::Value(n) => stack.push(*n),
        }
    }
    if stack.len() == 1 {
        Some(stack.pop()?)
    } else {
        None
    }
}
