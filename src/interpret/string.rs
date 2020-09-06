use super::runtime_error;
use super::runtime_value::RuntimeValue;

pub fn apply_concat(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let right = runtime_error::ensure_string(stack)?;
    let mut left = runtime_error::ensure_string(stack)?;

    left += &right;

    stack.push(RuntimeValue::String(left));

    Ok(())
}
