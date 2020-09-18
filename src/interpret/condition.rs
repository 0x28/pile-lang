use super::runtime_error;
use super::Interpreter;
use super::State;
use crate::pile_error::PileError;
use crate::program_source::ProgramSource;

use std::rc::Rc;

pub fn apply_if(
    state: &mut State,
    source: &Rc<ProgramSource>,
) -> Result<(), PileError> {
    let lines = state.current_lines;
    let to_pile_error =
        |msg| PileError::in_range(Rc::clone(&source), lines, msg);

    let stack = &mut state.stack;
    let condition = runtime_error::ensure_bool(stack).map_err(to_pile_error)?;
    let else_branch =
        runtime_error::ensure_function(state).map_err(to_pile_error)?;
    let if_branch =
        runtime_error::ensure_function(state).map_err(to_pile_error)?;

    if condition {
        Interpreter::call(&if_branch.exprs, state, &if_branch.source)?
    } else {
        Interpreter::call(&else_branch.exprs, state, &else_branch.source)?
    }

    Ok(())
}
