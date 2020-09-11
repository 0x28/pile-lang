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
                format!("Conversion from integer '{}' to natural is invalid", i)
            })?),
        )),
        Number::Float(f) => {
            stack.push(RuntimeValue::Number(Number::Natural(f as u64)))
        }
    }

    Ok(())
}

pub fn apply_integer(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let element = runtime_error::ensure_number(stack)?;

    match element {
        Number::Natural(n) => stack.push(RuntimeValue::Number(
            Number::Integer(n.try_into().map_err(|_| {
                format!("Conversion from natural '{}' to integer is invalid", n)
            })?),
        )),
        Number::Integer(i) => {
            stack.push(RuntimeValue::Number(Number::Integer(i)))
        }
        Number::Float(f) => {
            stack.push(RuntimeValue::Number(Number::Integer(f as i64)))
        }
    }

    Ok(())
}

pub fn apply_float(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let element = runtime_error::ensure_number(stack)?;

    match element {
        Number::Natural(n) => {
            stack.push(RuntimeValue::Number(Number::Float(n as f64)))
        }
        Number::Integer(i) => {
            stack.push(RuntimeValue::Number(Number::Float(i as f64)))
        }
        Number::Float(f) => stack.push(RuntimeValue::Number(Number::Float(f))),
    }

    Ok(())
}
