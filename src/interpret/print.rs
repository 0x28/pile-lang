use super::runtime_value::RuntimeValue;
use super::runtime_error;

pub fn apply_print(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let value = runtime_error::ensure_element(stack)?;
    print!("{}", value);
    Ok(())
}
