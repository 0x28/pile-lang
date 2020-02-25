use super::runtime_error;
use super::runtime_value::RuntimeValue;
use super::Interpreter;
use super::State;
use crate::lex::Number;
use crate::pile_error::PileError;
use crate::program_source::ProgramSource;

use std::rc::Rc;

pub fn apply_dotimes(
    state: &mut State,
    source: &Rc<ProgramSource>,
) -> Result<(), PileError> {
    let lines = state.current_lines;
    let to_pile_error = |msg| PileError::new(Rc::clone(&source), lines, msg);

    let stack = &mut state.stack;
    let count = runtime_error::ensure_element(stack).map_err(to_pile_error)?;
    let body = runtime_error::ensure_function(state).map_err(to_pile_error)?;

    let count = match count {
        RuntimeValue::Number(Number::Natural(n)) => n,
        RuntimeValue::Number(Number::Integer(i)) if i >= 0 => i as u32,
        val => {
            return Err(PileError::new(
                Rc::clone(&source),
                state.current_lines,
                format!("Expected positive number found {}", val.type_fmt()),
            ))
        }
    };

    for _ in 0..count {
        Interpreter::call(&body.exprs, state, &body.source)?
    }

    Ok(())
}
