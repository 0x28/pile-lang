use super::runtime_error;
use super::runtime_value::Function;
use super::runtime_value::RuntimeValue;
use super::Interpreter;
use super::State;

pub fn apply_if(state: &mut State) -> Result<(), String> {
    let stack = &mut state.stack;
    let condition = runtime_error::ensure_element(stack)?;
    let else_branch = runtime_error::ensure_function(state)?;
    let if_branch = runtime_error::ensure_function(state)?;

    let condition = match condition {
        RuntimeValue::Boolean(b) => b,
        v => return Err(format!("Expected boolean found '{}'", v)),
    };

    if condition {
        match if_branch {
            Function::Composite(block) => Interpreter::call(state, block)?,
            Function::Builtin(operator) => {
                Interpreter::apply(&operator, state)?
            }
        }
    } else {
        match else_branch {
            Function::Composite(block) => Interpreter::call(state, block)?,
            Function::Builtin(operator) => {
                Interpreter::apply(&operator, state)?
            }
        }
    }

    Ok(())
}
