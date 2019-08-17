use super::runtime_error;
use super::runtime_value::RuntimeValue;
use super::State;

pub fn apply_def(state: &mut State) -> Result<(), String> {
    let ident = match runtime_error::ensure_element(&mut state.stack)? {
        RuntimeValue::Identifier(ident) => ident,
        v => return Err(format!("Expected identifier found '{}'", v))
    };
    let value = runtime_error::ensure_element(&mut state.stack)?;

    state.lookup.insert(ident, value);
    Ok(())
}
