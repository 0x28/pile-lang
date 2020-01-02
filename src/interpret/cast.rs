use super::runtime_error;
use super::runtime_value::RuntimeValue;
use crate::lex::Number;

use std::convert::TryInto;

pub fn apply_natural(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let element = runtime_error::ensure_number(stack)?;

    match element {
        Number::Natural(n) => {
            stack.push(RuntimeValue::Number(Number::Natural(n)))
        }
        Number::Integer(i) => stack.push(RuntimeValue::Number(
            Number::Natural(i.try_into().map_err(|_| {
                format!(
                    "Conversion from {} to natural is invalid",
                    Number::Integer(i)
                )
            })?),
        )),
        Number::Float(f) => {
            stack.push(RuntimeValue::Number(Number::Natural(f as u32)))
        }
    }

    Ok(())
}

pub fn apply_integer(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let element = runtime_error::ensure_number(stack)?;

    match element {
        Number::Natural(n) => stack.push(RuntimeValue::Number(
            Number::Integer(n.try_into().map_err(|_| {
                format!(
                    "Conversion from {} to integer is invalid",
                    Number::Natural(n)
                )
            })?),
        )),
        Number::Integer(i) => {
            stack.push(RuntimeValue::Number(Number::Integer(i)))
        }
        Number::Float(f) => {
            stack.push(RuntimeValue::Number(Number::Integer(f as i32)))
        }
    }

    Ok(())
}

pub fn apply_float(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let element = runtime_error::ensure_number(stack)?;

    match element {
        Number::Natural(n) => {
            stack.push(RuntimeValue::Number(Number::Float(n as f32)))
        }
        Number::Integer(i) => {
            stack.push(RuntimeValue::Number(Number::Float(i as f32)))
        }
        Number::Float(f) => stack.push(RuntimeValue::Number(Number::Float(f))),
    }

    Ok(())
}