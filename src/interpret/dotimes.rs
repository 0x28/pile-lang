use super::runtime_error;
use super::Interpreter;
use super::runtime_value::Function;
use super::runtime_value::RuntimeValue;
use crate::lex::Number;

pub fn apply_dotimes(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let count = runtime_error::ensure_element(stack)?;
    let body = runtime_error::ensure_element(stack)?;

    let count = match count {
        RuntimeValue::Number(Number::Natural(n)) => n,
        RuntimeValue::Number(Number::Integer(i)) if i >= 0 => i as u32,
        val => return Err(format!("Expected positive number found {}", val)),
    };

    let body = match body {
        RuntimeValue::Function(func) => func,
        val => return Err(format!("Expected function found {}", val)),
    };

    match body {
        Function::Composite(block) => {
            for _ in 0..count {
                Interpreter::call(stack, block)?
            }
        }
        Function::Builtin(operator) => {
            for _ in 0..count {
                Interpreter::apply(&operator, stack)?
            }
        }
    }

    Ok(())
}
