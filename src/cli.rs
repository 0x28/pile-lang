use std::fs;
use std::io::{self, Read};

pub fn read_program<T: AsRef<str>>(args: &[T]) -> Result<String, String> {
    if args.len() == 1 {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|err| format!("stdin: {}", err))?;
        Ok(buffer)
    } else if args.len() == 2 {
        let filename = &args[1];

        fs::read_to_string(filename.as_ref())
            .map_err(|err| format!("{}: {}", filename.as_ref(), err))
    } else {
        Err(format!("Usage: {} [FILE]", args[0].as_ref()))
    }
}

#[cfg(test)]
mod test;
