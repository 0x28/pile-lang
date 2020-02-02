use super::runtime_error;
use super::runtime_value::Function;
use super::Interpreter;
use super::State;

use crate::program_source::ProgramSource;
use crate::pile_error::PileError;

use std::rc::Rc;

pub fn apply_while(
    state: &mut State,
    source: &Rc<ProgramSource>,
) -> Result<(), PileError> {
    let lines = state.current_lines;
    let to_pile_error =
        |msg| PileError::new(Rc::clone(&source), lines, msg);

    let condition =
        runtime_error::ensure_function(state).map_err(to_pile_error)?;
    let body = runtime_error::ensure_function(state).map_err(to_pile_error)?;

    loop {
        match &condition {
            Function::Composite(fsource, expr) => {
                Interpreter::call(&expr, state, &fsource)?
            }
            Function::Builtin(op) => Interpreter::apply(&op, state, source)?,
        };

        if runtime_error::ensure_bool(&mut state.stack)
            .map_err(to_pile_error)?
        {
            match &body {
                Function::Composite(fsource, expr) => {
                    Interpreter::call(&expr, state, &fsource)?
                }
                Function::Builtin(op) => {
                    Interpreter::apply(&op, state, source)?
                }
            };
        } else {
            break;
        }
    }

    Ok(())
}
