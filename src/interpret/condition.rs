use super::runtime_error;
use super::runtime_value::Function;
use super::Interpreter;
use super::State;
use crate::pile_error::PileError;

pub fn apply_if(state: &mut State) -> Result<(), PileError> {
    let stack = &mut state.stack;
    let condition =
        runtime_error::ensure_bool(stack).map_err(|msg| state.error(msg))?;
    let else_branch = runtime_error::ensure_function(state)
        .map_err(|msg| state.error(msg))?;
    let if_branch = runtime_error::ensure_function(state)
        .map_err(|msg| state.error(msg))?;

    if condition {
        match if_branch {
            Function::Composite(block) => Interpreter::call(&block, state)?,
            Function::Builtin(operator) => {
                Interpreter::apply(&operator, state)?
            }
        }
    } else {
        match else_branch {
            Function::Composite(block) => Interpreter::call(&block, state)?,
            Function::Builtin(operator) => {
                Interpreter::apply(&operator, state)?
            }
        }
    }

    Ok(())
}
