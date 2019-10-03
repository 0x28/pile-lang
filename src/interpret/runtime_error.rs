use super::runtime_value::Function;
use super::runtime_value::RuntimeValue;
use super::State;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct RuntimeError {
    lines: (u64, u64),
    message: String,
}

impl RuntimeError {
    pub fn new(lines: (u64, u64), message: String) -> RuntimeError {
        RuntimeError { lines, message }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.lines {
            (begin, end) if begin == end => {
                write!(f, "Line {}: {}", begin, self.message)
            }
            (begin, end) => {
                write!(f, "Lines {}-{}: {}", begin, end, self.message)
            }
        }
    }
}

pub fn ensure_element<T>(stack: &mut Vec<T>) -> Result<T, String> {
    stack.pop().ok_or_else(|| "Stack underflow".to_owned())
}

pub fn ensure_function<'e>(
    state: &mut State<'e>,
) -> Result<Function<'e>, String> {
    let func = ensure_element(&mut state.stack)?;

    match func {
        RuntimeValue::Function(f) => Ok(f),
        RuntimeValue::Identifier(i) => match state.lookup.get(&i) {
            Some(RuntimeValue::Function(f)) => Ok(f.clone()),
            Some(v) => Err(format!("Expected function found '{}'", v)),
            None => Err(format!("Unknown variable '{}'", i)),
        },
        v => Err(format!("Expected function found '{}'", v)),
    }
}

pub fn ensure_bool(
    stack: &mut Vec<RuntimeValue>,
) -> Result<bool, String> {
    let boolean = ensure_element(stack)?;

    match boolean {
        RuntimeValue::Boolean(b) => Ok(b),
        v => Err(format!("Expected boolean found '{}'", v)),
    }
}
