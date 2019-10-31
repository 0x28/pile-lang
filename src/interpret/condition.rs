use super::runtime_error;
use super::runtime_value::Function;
use super::Interpreter;
use super::State;

pub fn apply_if(state: &mut State) -> Result<(), String> {
    let stack = &mut state.stack;
    let condition = runtime_error::ensure_bool(stack)?;
    let else_branch = runtime_error::ensure_function(state)?;
    let if_branch = runtime_error::ensure_function(state)?;

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
