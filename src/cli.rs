use crate::program_source::ProgramSource;
use crate::repl;

use std::ffi::OsString;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::rc::Rc;

use atty::Stream;
use clap::{crate_version, App, Arg};

#[derive(Debug, PartialEq)]
pub struct CommandLineOptions {
    stack_size: usize,
    source: Rc<ProgramSource>,
    trace: bool,
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

    pub fn trace(&self) -> bool {
        self.trace
    }

    pub fn source(&self) -> Rc<ProgramSource> {
        Rc::clone(&self.source)
    }
}

pub fn read_options<I, T>(itr: I) -> Result<CommandLineOptions, String>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
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
            Arg::with_name("trace")
                .help("Enable program tracing")
                .short("t")
                .long("trace"),
        )
        .arg(
            Arg::with_name("FILE")
                .help("The program to run. Use '-' for stdin."),
        )
        .get_matches_from_safe(itr)
        .map_err(|e| e.to_string())?;

    let stack_size: usize = matches.value_of("size").unwrap().parse().unwrap();
    let file = matches.value_of("FILE");
    let trace = matches.is_present("trace");

    Ok(CommandLineOptions {
        stack_size,
        source: Rc::new(match file {
            None => {
                if atty::is(Stream::Stdin) {
                    ProgramSource::Repl
                } else {
                    ProgramSource::Stdin
                }
            }
            Some("-") => ProgramSource::Stdin,
            Some(file) => ProgramSource::File(PathBuf::from(file)),
        }),
        trace,
    })
}

#[cfg(test)]
mod test;
