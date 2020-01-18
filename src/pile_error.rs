use crate::cli::ProgramSource;

use std::fmt;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct PileError {
    source: Rc<ProgramSource>,
    lines: (u64, u64),
    message: String,
}

impl PileError {
    pub fn new(
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
}

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
