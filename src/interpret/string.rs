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
    let length = string.chars().count();
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

pub fn apply_upcase(stack: &mut [RuntimeValue]) -> Result<(), String> {
    let string = runtime_error::ensure_string_ref(stack)?;

    *string = string.to_uppercase();

    Ok(())
}

pub fn apply_downcase(stack: &mut [RuntimeValue]) -> Result<(), String> {
    let string = runtime_error::ensure_string_ref(stack)?;

    *string = string.to_lowercase();

    Ok(())
}

pub fn apply_trim(stack: &mut [RuntimeValue]) -> Result<(), String> {
    let string = runtime_error::ensure_string_ref(stack)?;

    *string = string.trim().to_owned();

    Ok(())
}

pub fn apply_format(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let format = runtime_error::ensure_string(stack)?;
    let mut split = format.rsplit("{}");
    let mut result = vec![String::from(split.next().unwrap())];

    for part in split {
        let value = runtime_error::ensure_element(stack)?;
        result.push(value.to_string());
        result.push(part.to_string());
    }

    result.reverse();
    stack.push(RuntimeValue::String(result.join("")));

    Ok(())
}

pub fn apply_index(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let index = runtime_error::ensure_element(stack)?;
    let index = match index {
        RuntimeValue::Number(Number::Natural(n)) => n,
        _ => {
            return Err(format!(
                "Can't use {} as string index",
                index.type_fmt()
            ))
        }
    };
    let string = runtime_error::ensure_string_ref(stack)?;
    let c = match string.chars().nth(index as usize) {
        Some(c) => c,
        None => {
            return Err(format!(
                "Invalid index {} for string \"{}\"",
                index, string
            ));
        }
    };

    stack.push(RuntimeValue::String(c.to_string()));

    Ok(())
}

#[cfg(test)]
mod test;
