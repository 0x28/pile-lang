use super::runtime_error;
use super::runtime_value::RuntimeValue;
use super::Interpreter;
use super::State;
use crate::pile_error::PileError;
use crate::program_source::ProgramSource;
use std::io::{self, BufRead};
use std::rc::Rc;

pub fn apply_readlines(
    state: &mut State,
    source: &Rc<ProgramSource>,
) -> Result<(), PileError> {
    let lines = state.current_lines;
    let to_pile_error =
        |msg| PileError::in_range(Rc::clone(&source), lines, msg);
    let func = runtime_error::ensure_function(state).map_err(to_pile_error)?;

    for line in io::stdin().lock().lines() {
        state.stack.push(RuntimeValue::String(line.unwrap()));
        Interpreter::call(&func.exprs, state, &func.source)?;
        let repeat = runtime_error::ensure_bool(&mut state.stack)
            .map_err(to_pile_error)?;

        if !repeat {
            break;
        }
    }

    Ok(())
}
