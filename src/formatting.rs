use crate::lex::{Lexer, Token};
use crate::pile_error::PileError;
use crate::program_source::ProgramSource;

use io::Write;
use std::cmp;
use std::fs::File;
use std::io;
use std::rc::Rc;

const FILE_BUFFER_SIZE: usize = 1000;

pub fn format(lexer: Lexer) -> Result<(), PileError> {
    let mut content = Vec::<u8>::with_capacity(FILE_BUFFER_SIZE);
    let source = Rc::clone(lexer.source());

    match lexer.source().as_ref() {
        ProgramSource::Repl => panic!("Can't format repl program!"),
        ProgramSource::Stdin => {
            write_formatting(&mut content, lexer)?;
            io::stdout()
                .write_all(&content)
                .map_err(|e| PileError::in_file(source, e.to_string()))?;
            Ok(())
        }
        ProgramSource::File(file) => {
            let file_name = file.clone();

            write_formatting(&mut content, lexer)?;

            let mut file = File::create(file_name.clone()).map_err(|e| {
                PileError::in_file(Rc::clone(&source), e.to_string())
            })?;

            file.write_all(&content)
                .map_err(|e| PileError::in_file(source, e.to_string()))?;

            println!("Formatted file '{}'.", file_name.to_string_lossy());

            Ok(())
        }
    }
}

fn write_formatting<W>(writer: &mut W, lexer: Lexer) -> Result<(), PileError>
where
    W: io::Write,
{
    let mut indent_level = 0u64;
    let mut prev_line = 0;
    let max_newlines = 2; // max one empty line
    let indent_width = 4;
    let source = Rc::clone(lexer.source());
    let mut previous_token = None;

    let to_pile_err =
        |e: io::Error| PileError::in_file(Rc::clone(&source), e.to_string());

    for (line, token) in lexer {
        let token = token?;

        if let Token::End = token {
            indent_level = indent_level.checked_sub(1).unwrap_or_default();
        }

        if prev_line < line {
            for _ in 0..cmp::min(line - prev_line, max_newlines) {
                // NOTE: no newlines at the beginning of the file
                if prev_line != 0 {
                    writeln!(writer).map_err(to_pile_err)?;
                }
            }

            for _ in 0..indent_level {
                write!(writer, "{}", " ".repeat(indent_width))
                    .map_err(to_pile_err)?;
            }
            prev_line = line;
        } else if previous_token != Some(Token::BracketLeft)
            && token != Token::BracketRight
        {
            write!(writer, " ").map_err(to_pile_err)?;
        }

        if let Token::Begin | Token::Let = token {
            indent_level += 1;
        }

        write!(writer, "{}", &token).map_err(to_pile_err)?;
        previous_token = Some(token);
    }

    writeln!(writer).map_err(to_pile_err)?;

    Ok(())
}

#[cfg(test)]
mod test;
