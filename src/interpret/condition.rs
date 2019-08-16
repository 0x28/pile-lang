use super::runtime_value::RuntimeValue;
use super::runtime_value::Function;
use super::Interpreter;
use super::State;
use super::runtime_error;

pub fn apply_if<'s, 'e: 's>(
    state: &'s mut State<'e>,
) -> Result<(), String> {
    let stack = &mut state.stack;
    let condition = runtime_error::ensure_element(stack)?;
    let else_branch = runtime_error::ensure_element(stack)?;
    let if_branch = runtime_error::ensure_element(stack)?;

    let if_branch = match if_branch {
        RuntimeValue::Function(body) => body,
        v => return Err(format!("Expected function found {:?}", v)),
    };

    let else_branch = match else_branch {
        RuntimeValue::Function(body) => body,
        v => return Err(format!("Expected function found {:?}", v)),
    };

    let condition = match condition {
        RuntimeValue::Boolean(b) => b,
        v => return Err(format!("Expected boolean found {:?}", v)),
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
