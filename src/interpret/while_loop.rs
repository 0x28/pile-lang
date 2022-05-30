use super::runtime_error;
use super::Interpreter;
use super::State;
use crate::pile_error::PileError;
use crate::program_source::ProgramSource;

use std::rc::Rc;

pub fn apply_while(
    state: &mut State,
    source: &Rc<ProgramSource>,
) -> Result<(), PileError> {
    let lines = state.current_lines;
    let to_pile_error =
        |msg| PileError::in_range(Rc::clone(source), lines, msg);

    let condition =
        runtime_error::ensure_function(state).map_err(to_pile_error)?;
    let body = runtime_error::ensure_function(state).map_err(to_pile_error)?;

    loop {
        Interpreter::call(&condition.exprs, state, &condition.source)?;

        if runtime_error::ensure_bool(&mut state.stack)
            .map_err(to_pile_error)?
        {
            Interpreter::call(&body.exprs, state, &body.source)?
        } else {
            break;
        }
    }

    Ok(())
}
