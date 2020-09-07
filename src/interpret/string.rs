use super::runtime_error;
use super::runtime_value::RuntimeValue;
use runtime_error::Number;

use std::convert::TryInto;

pub fn apply_concat(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let right = runtime_error::ensure_string(stack)?;
    let left = runtime_error::ensure_string_ref(stack)?;

    *left += &right;

    Ok(())
}

pub fn apply_length(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let string = runtime_error::ensure_string_ref(stack)?;
    let length = string.len();
    let length = length.try_into().map_err(|_| {
        format!("Can't convert string length '{}' to natural", length)
    })?;

    stack.push(RuntimeValue::Number(Number::Natural(length)));

    Ok(())
}

pub fn apply_contains(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let right = runtime_error::ensure_string(stack)?;
    let left = runtime_error::ensure_string(stack)?;

    stack.push(RuntimeValue::Boolean(left.contains(&right)));

    Ok(())
}
