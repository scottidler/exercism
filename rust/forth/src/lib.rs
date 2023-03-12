use std::result::Result;
use std::collections::HashMap;

pub type Value = i32;
pub type Env = HashMap<String, Value>;

pub struct Forth {
    stack: Vec<Value>,
    env: Env,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
}

#[macro_export]
macro_rules! args {
    ($($x:expr),*) => (&[$($x),*]);
}

pub trait IdentOrNum {
    fn is_alphabetic(&self) -> bool;
    fn is_numeric(&self) -> bool;
}

impl IdentOrNum for &str {
    fn is_alphabetic(&self) -> bool {
        self.chars().all(|c| c.is_alphabetic())
    }

    fn is_numeric(&self) -> bool {
        self.chars().all(|c| c.is_numeric())
    }
}

impl Forth {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            env: Env::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack[..]
    }


    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        for item in input.split(" ") {
            match item {
                "+" => self.add()?,
                "-" => self.sub()?,
                "*" => self.mul()?,
                "/" => self.div()?,
                "dup" => self.dup()?,
                "drop" => self.drop()?,
                "swap" => self.swap()?,
                "over" => self.over()?,
                ":" => {
                    let mut iter = input.split(" ").skip(1);
                    let name = iter.next().unwrap();
                    let mut ops = Vec::new();
                    while let Some(item) = iter.next() {
                        if item == ";" {
                            break;
                        }
                        ops.push(item);
                    }
                    self.env.insert(name.to_string(), ops.len() as Value);
                },

                i if i.is_alphabetic() => {
                    if let Some(&value) = self.env.get(i) {
                        self.stack.push(value);
                    } else {
                        return Err(Error::UnknownWord);
                    }
                },
                i if i.is_numeric() => {
                    if let Ok(value) = i.parse::<Value>() {
                        self.stack.push(value);
                    } else {
                        return Err(Error::InvalidWord);
                    }
                },
                _ => return Err(Error::InvalidWord),
            }
        }
        Ok(())
    }

    fn pop(&mut self, n: usize) -> Result<(Value, Value), Error> {
        if self.stack.len() < n {
            Err(Error::StackUnderflow)
        } else {
            let a = self.stack.pop().unwrap();
            let b = self.stack.pop().unwrap();
            Ok((a, b))
        }
    }

    fn push(&mut self, values: &[Value]) {
        for value in values {
            self.stack.push(*value);
        }
    }

    fn add(&mut self) -> Result<(), Error> {
        let (a, b) = self.pop(2)?;
        self.push(args!(a + b));
        Ok(())
    }

    fn sub(&mut self) -> Result<(), Error> {
        let (a, b) = self.pop(2)?;
        self.push(args!(b - a));
        Ok(())
    }

    fn mul(&mut self) -> Result<(), Error> {
        let (a, b) = self.pop(2)?;
        self.push(args!(a * b));
        Ok(())
    }

    fn div(&mut self) -> Result<(), Error> {
        let (a, b) = self.pop(2)?;
        if a == 0 {
            Err(Error::DivisionByZero)
        } else {
            self.push(args!((b / a)));
            Ok(())
        }
    }

    fn dup(&mut self) -> Result<(), Error> {
        let (a, _) = self.pop(1)?;
        self.push(args!(a, a));
        Ok(())
    }

    fn drop(&mut self) -> Result<(), Error> {
        self.pop(1)?;
        Ok(())
    }

    fn swap(&mut self) -> Result<(), Error> {
        let (a, b) = self.pop(2)?;
        self.push(args!(a, b));
        Ok(())
    }

    fn over(&mut self) -> Result<(), Error> {
        let (a, b) = self.pop(2)?;
        self.push(args!(b, a, b));
        Ok(())
    }
}
