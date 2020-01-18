use crate::repl;

use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::rc::Rc;

use clap::{crate_version, App, Arg};

pub struct CommandLineOptions {
    stack_size: usize,
    source: Rc<ProgramSource>,
    debug: bool,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ProgramSource {
    Repl,
    Stdin,
    File(PathBuf),
}

impl CommandLineOptions {
    pub fn read_program(&self) -> Result<String, String> {
        match self.source.as_ref() {
            ProgramSource::Repl => repl::repl(),
            ProgramSource::Stdin => {
                let mut buffer = String::new();
                io::stdin()
                    .read_to_string(&mut buffer)
                    .map_err(|err| format!("stdin: {}", err))?;
                Ok(buffer)
            }
            ProgramSource::File(file) => fs::read_to_string(file)
                .map_err(|err| format!("{}: {}", file.to_string_lossy(), err)),
        }
    }

    pub fn stack_size(&self) -> usize {
        self.stack_size
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn source(&self) -> Rc<ProgramSource> {
        Rc::clone(&self.source)
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
    let file = matches.value_of("FILE");
    let debug = matches.is_present("debug");

    CommandLineOptions {
        stack_size,
        source: Rc::new(match file {
            None => ProgramSource::Repl,
            Some("-") => ProgramSource::Stdin,
            Some(file) => ProgramSource::File(PathBuf::from(file)),
        }),
        debug,
    }
}

#[cfg(test)]
mod test;
