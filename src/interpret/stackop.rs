use super::runtime_error;
use super::runtime_value::RuntimeValue;

pub fn apply_dup(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let element = runtime_error::ensure_element(stack)?;

    stack.push(element.clone());
    stack.push(element);

    Ok(())
}

pub fn apply_drop(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    runtime_error::ensure_element(stack)?;

    Ok(())
}

pub fn apply_swap(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let top = runtime_error::ensure_element(stack)?;
    let other = runtime_error::ensure_element(stack)?;

    stack.push(top);
    stack.push(other);

    Ok(())
}
