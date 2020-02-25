use super::runtime_value::RuntimeValue;
use super::runtime_value::Function;
pub use crate::lex::Number;
use super::State;

pub fn ensure_element<T>(stack: &mut Vec<T>) -> Result<T, String> {
    stack.pop().ok_or_else(|| "Stack underflow".to_owned())
}

pub fn ensure_function(
    state: &mut State,
) -> Result<Function, String> {
    let func = ensure_element(&mut state.stack)?;

    match func {
        RuntimeValue::Function(f) => Ok(f),
        v => Err(format!("Expected function found {}", v.type_fmt())),
    }
}

pub fn ensure_bool(stack: &mut Vec<RuntimeValue>) -> Result<bool, String> {
    let boolean = ensure_element(stack)?;

    match boolean {
        RuntimeValue::Boolean(b) => Ok(b),
        v => Err(format!("Expected boolean found {}", v.type_fmt())),
    }
}

pub fn ensure_number(stack: &mut Vec<RuntimeValue>) -> Result<Number, String> {
    let value = ensure_element(stack)?;

    match value {
        RuntimeValue::Number(n) => Ok(n),
        v => Err(format!("Expected number found {}", v.type_fmt())),
    }
}
