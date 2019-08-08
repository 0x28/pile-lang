use super::runtime_value::RuntimeValue;
use super::runtime_error;
use crate::lex::Number;

fn apply_numeric<N, I, F>(
    op_natural: N,
    op_integer: I,
    op_float: F,
    stack: &mut Vec<RuntimeValue>,
) -> Result<(), String>
where
    N: Fn(u32, u32) -> Result<u32, String>,
    I: Fn(i32, i32) -> Result<i32, String>,
    F: Fn(f32, f32) -> f32,
{
    let right = runtime_error::ensure_element(stack)?;
    let left = runtime_error::ensure_element(stack)?;

    let (left, right) = match (left, right) {
        (RuntimeValue::Number(lhs), RuntimeValue::Number(rhs)) => (lhs, rhs),
        (lhs, rhs) => return Err(format!("Type error: {:?}, {:?}", lhs, rhs)),
    };

    let result = match (left, right) {
        (Number::Natural(lhs), Number::Natural(rhs)) => {
            Number::Natural(op_natural(lhs, rhs)?)
        }
        (Number::Integer(lhs), Number::Integer(rhs)) => {
            Number::Integer(op_integer(lhs, rhs)?)
        }
        (Number::Float(lhs), Number::Float(rhs)) => {
            Number::Float(op_float(lhs, rhs))
        }
        (lhs, rhs) => {
            return Err(format!("Numeric type mismatch: {:?}, {:?}", lhs, rhs))
        }
    };

    stack.push(RuntimeValue::Number(result));

    Ok(())
}

pub fn apply_plus(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    apply_numeric(
        |a, b| {
            a.checked_add(b)
                .ok_or_else(|| "Numeric overflow".to_owned())
        },
        |a, b| {
            a.checked_add(b)
                .ok_or_else(|| "Numeric overflow".to_owned())
        },
        |a, b| a + b,
        stack,
    )
}

pub fn apply_minus(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    apply_numeric(
        |a, b| {
            a.checked_sub(b)
                .ok_or_else(|| "Numeric overflow".to_owned())
        },
        |a, b| {
            a.checked_sub(b)
                .ok_or_else(|| "Numeric overflow".to_owned())
        },
        |a, b| a - b,
        stack,
    )
}

pub fn apply_mul(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    apply_numeric(
        |a, b| {
            a.checked_mul(b)
                .ok_or_else(|| "Numeric overflow".to_owned())
        },
        |a, b| {
            a.checked_mul(b)
                .ok_or_else(|| "Numeric overflow".to_owned())
        },
        |a, b| a * b,
        stack,
    )
}

pub fn apply_div(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    apply_numeric(
        |a, b| {
            a.checked_div(b)
                .ok_or_else(|| "Division by zero".to_owned())
        },
        |a, b| {
            a.checked_div(b)
                .ok_or_else(|| "Division by zero".to_owned())
        },
        |a, b| a / b,
        stack,
    )
}
