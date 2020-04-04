use super::runtime_error;
use super::runtime_value::RuntimeValue;

pub fn apply_assert(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let value = runtime_error::ensure_bool(stack)?;

    if value {
        Ok(())
    } else {
        Err("Assertion failed".to_owned())
    }
}
