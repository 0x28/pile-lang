use super::runtime_value::Function;
use super::runtime_value::RuntimeValue;
use super::State;

pub fn ensure_element<T>(stack: &mut Vec<T>) -> Result<T, String> {
    stack.pop().ok_or_else(|| "Stack underflow".to_owned())
}

pub fn ensure_function<'e>(
    state: &mut State<'e>,
) -> Result<Function<'e>, String> {
    let func = ensure_element(&mut state.stack)?;

    match func {
        RuntimeValue::Function(f) => Ok(f),
        RuntimeValue::Identifier(i) => match state.lookup.get(&i) {
            Some(RuntimeValue::Function(f)) => Ok(f.clone()),
            Some(v) => Err(format!("Expected function found '{}'", v)),
            None => Err(format!("Unknown variable '{}'", i)),
        },
        v => Err(format!("Expected function found '{}'", v)),
    }
}
