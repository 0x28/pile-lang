use super::runtime_error;
use super::runtime_value::Function;
use super::Interpreter;
use super::State;

use crate::pile_error::PileError;

pub fn apply_while(state: &mut State) -> Result<(), PileError> {
    let condition = runtime_error::ensure_function(state)
        .map_err(|msg| state.error(msg))?;
    let body = runtime_error::ensure_function(state)
        .map_err(|msg| state.error(msg))?;

    loop {
        match &condition {
            Function::Composite(expr) => Interpreter::call(&expr, state)?,
            Function::Builtin(op) => Interpreter::apply(&op, state)?,
        };

        if runtime_error::ensure_bool(&mut state.stack)
            .map_err(|msg| state.error(msg))?
        {
            match &body {
                Function::Composite(expr) => Interpreter::call(&expr, state)?,
                Function::Builtin(op) => Interpreter::apply(&op, state)?,
            };
        } else {
            break;
        }
    }

    Ok(())
}
