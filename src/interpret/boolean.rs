use super::runtime_error;
use super::runtime_value::RuntimeValue;
use crate::lex::Number;

fn apply_bool<N, I, F, S>(
    nat_nat_cmp: N,
    int_int_cmp: I,
    float_float_cmp: F,
    str_str_cmp: S,
    stack: &mut Vec<RuntimeValue>,
) -> Result<(), String>
where
    N: Fn(u32, u32) -> bool,
    I: Fn(i32, i32) -> bool,
    F: Fn(f32, f32) -> bool,
    S: Fn(&str, &str) -> bool,
{
    let right = runtime_error::ensure_element(stack)?;
    let left = runtime_error::ensure_element(stack)?;

    let compare_result = match (left, right) {
        (
            RuntimeValue::Number(Number::Natural(num_left)),
            RuntimeValue::Number(Number::Natural(num_right)),
        ) => nat_nat_cmp(num_left, num_right),
        (
            RuntimeValue::Number(Number::Integer(num_left)),
            RuntimeValue::Number(Number::Integer(num_right)),
        ) => int_int_cmp(num_left, num_right),
        (
            RuntimeValue::Number(Number::Float(num_left)),
            RuntimeValue::Number(Number::Float(num_right)),
        ) => float_float_cmp(num_left, num_right),
        (RuntimeValue::String(str_left), RuntimeValue::String(str_right)) => {
            str_str_cmp(str_left.as_ref(), str_right.as_ref())
        }
        (left, right) => {
            return Err(format!(
                "Can't compare {} and {}",
                left.type_fmt(),
                right.type_fmt()
            ))
        }
    };

    stack.push(RuntimeValue::Boolean(compare_result));

    Ok(())
}

pub fn apply_less(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    apply_bool(
        |n1, n2| n1 < n2,
        |i1, i2| i1 < i2,
        |f1, f2| f1 < f2,
        |s1, s2| s1 < s2,
        stack,
    )
}

pub fn apply_less_equal(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    apply_bool(
        |n1, n2| n1 <= n2,
        |i1, i2| i1 <= i2,
        |f1, f2| f1 <= f2,
        |s1, s2| s1 <= s2,
        stack,
    )
}

#[allow(clippy::float_cmp)]
pub fn apply_equal(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    apply_bool(
        |n1, n2| n1 == n2,
        |i1, i2| i1 == i2,
        |f1, f2| f1 == f2,
        |s1, s2| s1 == s2,
        stack,
    )
}

pub fn apply_greater(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    apply_bool(
        |n1, n2| n1 > n2,
        |i1, i2| i1 > i2,
        |f1, f2| f1 > f2,
        |s1, s2| s1 > s2,
        stack,
    )
}

pub fn apply_greater_equal(
    stack: &mut Vec<RuntimeValue>,
) -> Result<(), String> {
    apply_bool(
        |n1, n2| n1 >= n2,
        |i1, i2| i1 >= i2,
        |f1, f2| f1 >= f2,
        |s1, s2| s1 >= s2,
        stack,
    )
}

pub fn apply_and(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let left = runtime_error::ensure_bool(stack)?;
    let right = runtime_error::ensure_bool(stack)?;

    stack.push(RuntimeValue::Boolean(left && right));

    Ok(())
}

pub fn apply_or(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let left = runtime_error::ensure_bool(stack)?;
    let right = runtime_error::ensure_bool(stack)?;

    stack.push(RuntimeValue::Boolean(left || right));

    Ok(())
}

pub fn apply_not(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let value = runtime_error::ensure_bool(stack)?;

    stack.push(RuntimeValue::Boolean(!value));

    Ok(())
}
