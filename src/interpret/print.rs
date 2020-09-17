use super::runtime_error;
use super::runtime_value::RuntimeValue;

pub fn apply_print(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    let value = runtime_error::ensure_element(stack)?;
    print!("{}", value);
    Ok(())
}

pub fn apply_showstack(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
    for (index, element) in stack.iter().rev().enumerate().rev() {
        print!(" [{}]:\t", index);

        match element {
            RuntimeValue::String(string) => {
                println!("\"{}\"", string)
            }
            value => println!("{}", value)
        }
    }

    Ok(())
}
