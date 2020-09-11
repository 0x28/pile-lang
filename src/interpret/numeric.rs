use super::runtime_error;
use super::runtime_value::RuntimeValue;
use crate::lex::Number;

fn apply_numeric<N, I, F>(
    op_natural: N,
    op_integer: I,
    op_float: F,
    stack: &mut Vec<RuntimeValue>,
) -> Result<(), String>
where
    N: Fn(&u64, &u64) -> Result<u64, String>,
    I: Fn(&i64, &i64) -> Result<i64, String>,
    F: Fn(&f64, &f64) -> f64,
{
    let right = runtime_error::ensure_element(stack)?;
    let left = runtime_error::ensure_element(stack)?;

    let (num_left, num_right) = match (&left, &right) {
        (RuntimeValue::Number(lhs), RuntimeValue::Number(rhs)) => (lhs, rhs),
        (lhs, rhs) => {
            return Err(format!(
                "Type error: {}, {}",
                lhs.type_fmt(),
                rhs.type_fmt()
            ))
        }
    };

    let result = match (num_left, num_right) {
        (Number::Natural(lhs), Number::Natural(rhs)) => {
            Number::Natural(op_natural(lhs, rhs)?)
        }
        (Number::Integer(lhs), Number::Integer(rhs)) => {
            Number::Integer(op_integer(lhs, rhs)?)
        }
        (Number::Float(lhs), Number::Float(rhs)) => {
            Number::Float(op_float(lhs, rhs))
        }
        (_, _) => {
            return Err(format!(
                "Numeric type mismatch: {}, {}",
                left.type_fmt(),
                right.type_fmt(),
            ))
        }
    };

    stack.push(RuntimeValue::Number(result));

    Ok(())
}

macro_rules! num_op {
    ($name: ident, $checked_op: ident, $float_op: expr, $err: literal) => {
        pub fn $name(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
            apply_numeric(
                |a, b| a.$checked_op(*b).ok_or_else(|| format!($err, a, b)),
                |a, b| a.$checked_op(*b).ok_or_else(|| format!($err, a, b)),
                $float_op,
                stack,
            )
        }
    };
}

num_op!(
    apply_plus,
    checked_add,
    |a, b| a + b,
    "Numeric overflow while adding '{}' and '{}'"
);

num_op!(
    apply_minus,
    checked_sub,
    |a, b| a - b,
    "Numeric overflow while subtracting '{}' and '{}'"
);

num_op!(
    apply_mul,
    checked_mul,
    |a, b| a * b,
    "Numeric overflow while multiplying '{}' and '{}'"
);

num_op!(
    apply_div,
    checked_div,
    |a, b| a / b,
    "Division by zero while dividing '{}' and '{}'"
);
