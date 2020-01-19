use super::runtime_error;
use super::runtime_value::Function;
use super::Interpreter;
use super::State;

use crate::cli::ProgramSource;
use crate::pile_error::PileError;

use std::rc::Rc;

pub fn apply_if(
    state: &mut State,
    source: &Rc<ProgramSource>,
) -> Result<(), PileError> {
    let lines = state.current_lines;
    let to_pile_error = |msg| PileError::new(Rc::clone(&source), lines, msg);

    let stack = &mut state.stack;
    let condition = runtime_error::ensure_bool(stack).map_err(to_pile_error)?;
    let else_branch =
        runtime_error::ensure_function(state).map_err(to_pile_error)?;
    let if_branch =
        runtime_error::ensure_function(state).map_err(to_pile_error)?;

    if condition {
        match if_branch {
            Function::Composite(fsource, block) => {
                Interpreter::call(&block, state, &fsource)?
            }
            Function::Builtin(operator) => {
                Interpreter::apply(&operator, state, source)?
            }
        }
    } else {
        match else_branch {
            Function::Composite(fsource, block) => {
                Interpreter::call(&block, state, &fsource)?
            }
            Function::Builtin(operator) => {
                Interpreter::apply(&operator, state, source)?
            }
        }
    }

    Ok(())
}
