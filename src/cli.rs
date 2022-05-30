use crate::program_source::ProgramSource;
use crate::repl;

use std::ffi::OsString;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::rc::Rc;

use atty::Stream;
use clap::{crate_version, App, Arg, ErrorKind};

#[derive(Debug, PartialEq)]
pub struct CompletionOptions {
    pub prefix: String,
    pub line: u64,
}

#[derive(Debug, PartialEq)]
pub struct CommandLineOptions {
    stack_size: usize,
    source: Rc<ProgramSource>,
    trace: bool,
    format: bool,
    completion: Option<CompletionOptions>,
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

    pub fn format(&self) -> bool {
        self.format
    }

    pub fn source(&self) -> Rc<ProgramSource> {
        Rc::clone(&self.source)
    }

    pub fn completion(&self) -> &Option<CompletionOptions> {
        &self.completion
    }
}

pub fn read_options<I, T>(itr: I) -> Result<CommandLineOptions, String>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = App::new("pile")
        .version(crate_version!())
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
        .arg(
            Arg::with_name("complete")
                .help(
                    "Print symbols that start with <prefix> in context <line>",
                )
                .short("c")
                .long("complete")
                .value_names(&["prefix", "line"])
                .requires("FILE"),
        )
        .arg(
            Arg::with_name("format")
                .help("Format the given program.")
                .short("f")
                .long("format")
                .requires("FILE"),
        )
        .get_matches_from_safe(itr);

    let matches = match matches {
        Err(e) => match e.kind {
            ErrorKind::HelpDisplayed | ErrorKind::VersionDisplayed => {
                println!("{}", e.message);
                std::process::exit(0);
            }
            _ => return Err(e.to_string()),
        },
        Ok(m) => m,
    };

    let stack_size: usize = matches.value_of("size").unwrap().parse().unwrap();
    let file = matches.value_of("FILE");
    let trace = matches.is_present("trace");
    let format = matches.is_present("format");
    let source = Rc::new(match file {
        None => {
            if atty::is(Stream::Stdin) {
                ProgramSource::Repl
            } else {
                ProgramSource::Stdin
            }
        }
        Some("-") => ProgramSource::Stdin,
        Some(file) => ProgramSource::File(PathBuf::from(file)),
    });
    let completion: Option<CompletionOptions> =
        match matches.values_of("complete") {
            Some(values) => {
                let values: Vec<&str> = values.collect();
                Some(CompletionOptions {
                    prefix: values[0].to_owned(),
                    line: values[1].parse().map_err(|e| {
                        format!("error parsing <line> in '--complete': {}", e)
                    })?,
                })
            }
            None => None,
        };

    Ok(CommandLineOptions {
        stack_size,
        source,
        trace,
        format,
        completion,
    })
}

#[cfg(test)]
mod test;
