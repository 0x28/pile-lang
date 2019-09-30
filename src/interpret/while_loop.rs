use super::runtime_error;
use super::runtime_value::Function;
use super::Interpreter;
use super::State;

pub fn apply_while(state: &mut State) -> Result<(), String> {
    let condition = runtime_error::ensure_function(state)?;
    let body = runtime_error::ensure_function(state)?;

    loop {
        match condition {
            Function::Composite(expr) => Interpreter::call(expr, state)?,
            Function::Builtin(op) => Interpreter::apply(op, state)?,
        };

        if runtime_error::ensure_bool(state)? {
            match body {
                Function::Composite(expr) => Interpreter::call(expr, state)?,
                Function::Builtin(op) => Interpreter::apply(op, state)?,
            };
        } else {
            break;
        }
    }

    Ok(())
}
