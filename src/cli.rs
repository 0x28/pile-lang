use crate::repl;

use std::fs;
use std::io::{self, Read};

use clap::{crate_version, App, Arg};

pub struct CommandLineOptions {
    stack_size: usize,
    program: Option<String>,
    debug: bool,
}

impl CommandLineOptions {
    pub fn read_program(&self) -> Result<String, String> {
        match &self.program {
            None => repl::repl(),
            Some(filename) => match filename.as_ref() {
                "-" => {
                    let mut buffer = String::new();
                    io::stdin()
                        .read_to_string(&mut buffer)
                        .map_err(|err| format!("stdin: {}", err))?;
                    Ok(buffer)
                }
                file => fs::read_to_string(file)
                    .map_err(|err| format!("{}: {}", file, err)),
            },
        }
    }
}

pub fn read_options() -> CommandLineOptions {
    let matches = App::new("pile")
        .version(&crate_version!()[..])
        .author("created by 0x28")
        .about("A simple stack-based programming language")
        .arg(
            Arg::with_name("size")
                .help("The initial size of the stack")
                .short("s")
                .long("stack-size")
                .default_value("100")
                .validator(|value| {
                    if value.chars().all(char::is_numeric) {
                        Ok(())
                    } else {
                        Err("The value must be a natural number".to_owned())
                    }
                }),
        )
        .arg(
            Arg::with_name("debug")
                .help("Enable stack traces")
                .short("d")
                .long("debug"),
        )
        .arg(
            Arg::with_name("FILE")
                .help("The program to run. Use '-' for stdin."),
        )
        .get_matches();

    let stack_size: usize = matches.value_of("size").unwrap().parse().unwrap();
    let program = matches.value_of("FILE");
    let debug = matches.is_present("debug");

    CommandLineOptions {
        stack_size,
        program: program.map(str::to_owned),
        debug,
    }
}

#[cfg(test)]
mod test;
