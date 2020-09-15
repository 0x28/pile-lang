use super::runtime_error;
use super::runtime_value::RuntimeValue;
use crate::lex::Number;

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

pub fn apply_pick(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let index = runtime_error::ensure_element(stack)?;
    let index = match index {
        RuntimeValue::Number(Number::Natural(n)) => n as usize,
        _ => {
            return Err(format!(
                "Can't use {} as stack index",
                index.type_fmt()
            ))
        }
    };

    let index = match stack.len().checked_sub(index + 1) {
        Some(index) => index,
        None => {
            return Err(format!(
                "Invalid index {} for pick into stack of size {}",
                index,
                stack.len()
            ))
        }
    };

    stack.push(stack[index].clone());

    Ok(())
}

pub fn apply_clear(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    stack.clear();
    Ok(())
}

pub fn apply_stacksize(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    stack.push(RuntimeValue::Number(Number::Natural(stack.len() as u64)));

    Ok(())
}

#[cfg(test)]
mod test;
