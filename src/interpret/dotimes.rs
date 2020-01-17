use super::runtime_error;
use super::runtime_value::Function;
use super::runtime_value::RuntimeValue;
use super::Interpreter;
use super::State;

use crate::lex::Number;
use crate::pile_error::PileError;

pub fn apply_dotimes(state: &mut State) -> Result<(), PileError> {
    let stack = &mut state.stack;
    let count =
        runtime_error::ensure_element(stack).map_err(|msg| state.error(msg))?;
    let body = runtime_error::ensure_function(state)
        .map_err(|msg| state.error(msg))?;

    let count = match count {
        RuntimeValue::Number(Number::Natural(n)) => n,
        RuntimeValue::Number(Number::Integer(i)) if i >= 0 => i as u32,
        val => {
            return Err(state.error(format!(
                "Expected positive number found {}",
                val.type_fmt()
            )))
        }
    };

    match body {
        Function::Composite(block) => {
            for _ in 0..count {
                Interpreter::call(&block, state)?
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
