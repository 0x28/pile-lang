use super::runtime_error;
use super::runtime_value::RuntimeValue;

use std::cmp::Ordering;

fn apply_ordering(
    orderings: &[Ordering],
    stack: &mut Vec<RuntimeValue>,
) -> Result<(), String> {
    let right = runtime_error::ensure_element(stack)?;
    let left = runtime_error::ensure_element(stack)?;

    let compare_result = match left.partial_cmp(&right) {
        None => {
            return Err(format!(
                "Can't compare {} and {}",
                left.type_fmt(),
                right.type_fmt()
            ))
        }
        Some(ordering) => ordering,
    };

    stack.push(RuntimeValue::Boolean(orderings.contains(&compare_result)));

    Ok(())
}

pub fn apply_less(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    apply_ordering(&[Ordering::Less], stack)
}

pub fn apply_less_equal(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    apply_ordering(&[Ordering::Less, Ordering::Equal], stack)
}

pub fn apply_equal(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    apply_ordering(&[Ordering::Equal], stack)
}

pub fn apply_greater(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    apply_ordering(&[Ordering::Greater], stack)
}

pub fn apply_greater_equal(
    stack: &mut Vec<RuntimeValue>,
) -> Result<(), String> {
    apply_ordering(&[Ordering::Greater, Ordering::Equal], stack)
}

pub fn apply_and(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let right = runtime_error::ensure_bool(stack)?;
    let left = runtime_error::ensure_bool(stack)?;

    stack.push(RuntimeValue::Boolean(left && right));

    Ok(())
}

pub fn apply_or(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let right = runtime_error::ensure_bool(stack)?;
    let left = runtime_error::ensure_bool(stack)?;

    stack.push(RuntimeValue::Boolean(left || right));

    Ok(())
}

pub fn apply_not(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let value = runtime_error::ensure_bool(stack)?;

    stack.push(RuntimeValue::Boolean(!value));

    Ok(())
}
