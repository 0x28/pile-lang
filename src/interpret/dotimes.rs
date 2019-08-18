use super::runtime_error;
use super::Interpreter;
use super::State;
use super::runtime_value::Function;
use super::runtime_value::RuntimeValue;
use crate::lex::Number;

pub fn apply_dotimes(state: &mut State) -> Result<(), String> {
    let stack = &mut state.stack;
    let count = runtime_error::ensure_element(stack)?;
    let body = runtime_error::ensure_function(state)?;

    let count = match count {
        RuntimeValue::Number(Number::Natural(n)) => n,
        RuntimeValue::Number(Number::Integer(i)) if i >= 0 => i as u32,
        val => return Err(format!("Expected positive number found '{}'", val)),
    };

    match body {
        Function::Composite(block) => {
            for _ in 0..count {
                Interpreter::call(state, block)?
            }
        }
        Function::Builtin(operator) => {
            for _ in 0..count {
                Interpreter::apply(&operator, state)?
            }
        }
    }

    Ok(())
}
