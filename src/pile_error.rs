use crate::program_source::ProgramSource;

use std::error::Error;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct PileError {
    source: Rc<ProgramSource>,
    lines: (u64, u64),
    message: String,
}

impl PileError {
    pub fn in_range(
        source: Rc<ProgramSource>,
        lines: (u64, u64),
        message: String,
    ) -> Self {
        PileError {
            source,
            lines,
            message,
        }
    }

    pub fn in_line(
        source: Rc<ProgramSource>,
        line: u64,
        message: String,
    ) -> Self {
        PileError {
            source,
            lines: (line, line),
            message,
        }
    }

    pub fn in_file(source: Rc<ProgramSource>, message: String) -> Self {
        PileError {
            source,
            lines: (0, 0),
            message,
        }
    }
}

impl Error for PileError {}

impl fmt::Display for PileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.source.as_ref() {
            ProgramSource::Repl | ProgramSource::Stdin => (),
            ProgramSource::File(file) => {
                write!(f, "{}:", file.to_string_lossy())?;
            }
        }
        match self.lines {
            (begin, end) if begin == end => {
                write!(f, "{}: {}", begin, self.message)
            }
            (begin, end) => write!(f, "{}-{}: {}", begin, end, self.message),
        }
    }
}
