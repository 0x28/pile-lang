use super::runtime_value::RuntimeValue;
use super::runtime_value::Function;
use super::Interpreter;

pub fn apply_if<'s, 'e: 's>(
    stack: &'s mut Vec<RuntimeValue<'e>>,
) -> Result<(), String> {
    let condition = Interpreter::ensure_element(stack)?;
    let else_branch = Interpreter::ensure_element(stack)?;
    let if_branch = Interpreter::ensure_element(stack)?;

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
            Function::Composite(block) => Interpreter::call(stack, block)?,
            Function::Builtin(operator) => {
                Interpreter::apply(&operator, stack)?
            }
        }
    } else {
        match else_branch {
            Function::Composite(block) => Interpreter::call(stack, block)?,
            Function::Builtin(operator) => {
                Interpreter::apply(&operator, stack)?
            }
        }
    }

    Ok(())
}
