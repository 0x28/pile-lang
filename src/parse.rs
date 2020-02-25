use crate::lex::Lexer;
use crate::lex::LexerIter;
use crate::lex::Token;
use crate::pile_error::PileError;
use crate::program_source::ProgramSource;

use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub source: Rc<ProgramSource>,
    pub expressions: Vec<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Atom {
        line: u64,
        token: Token,
    },
    Assignment {
        line: u64,
        var: String,
    },
    Block {
        begin: u64,
        end: u64,
        expressions: Rc<Vec<Expr>>,
    },
    Use {
        line: u64,
        subprogram: Ast,
    },
}

impl Expr {
    pub fn lines(&self) -> (u64, u64) {
        match self {
            Self::Atom { line, .. } => (*line, *line),
            Self::Assignment { line, .. } => (*line, *line),
            Self::Block { begin, end, .. } => (*begin, *end),
            Self::Use { line, .. } => (*line, *line),
        }
    }
}

pub struct Parser<'a> {
    lex_iter: LexerIter<'a>,
    lookahead: Option<(u64, Token)>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lex_iter: lexer.into_iter(),
            lookahead: None,
        }
    }

    pub fn parse(mut self) -> Result<Ast, PileError> {
        let mut program = vec![];

        loop {
            self.consume()?;

            let expr = match self.lookahead {
                Some((line, Token::End)) => {
                    return Err(self.parse_error(line, "Unmatched 'end'."));
                }
                Some((_, Token::Begin)) => self.block()?,
                Some((_, Token::Assign)) => self.assign()?,
                Some((_, Token::Use)) => self.using()?,
                Some((line, _)) => Expr::Atom {
                    line,
                    token: self.lookahead.take().unwrap().1,
                },
                None => break,
            };

            program.push(expr);
        }

        Ok(Ast {
            source: Rc::clone(self.lex_iter.source()),
            expressions: program,
        })
    }

    fn parse_error(&self, line: u64, msg: &str) -> PileError {
        PileError::new(
            Rc::clone(self.lex_iter.source()),
            (line, line),
            msg.to_owned(),
        )
    }

    fn block(&mut self) -> Result<Expr, PileError> {
        self.expect(Token::Begin)?;

        let begin = self.lookahead.as_ref().unwrap().0;
        let end;

        let mut block = vec![];

        loop {
            self.consume()?;

            match self.lookahead {
                None => {
                    return Err(self.parse_error(
                        self.lex_iter.line(),
                        &"Expected 'end' found end of file.".to_string(),
                    ));
                }

                Some((line, Token::End)) => {
                    end = line;
                    break;
                }
                Some((_, Token::Begin)) => block.push(self.block()?),
                Some((_, Token::Assign)) => block.push(self.assign()?),
                Some((line, Token::Use)) => {
                    return Err(self.parse_error(
                        line,
                        "'use' isn't allowed inside blocks.",
                    ));
                }
                Some((line, _)) => block.push(Expr::Atom {
                    line,
                    token: self.lookahead.take().unwrap().1,
                }),
            }
        }

        Ok(Expr::Block {
            begin,
            end,
            expressions: Rc::new(block),
        })
    }

    fn assign(&mut self) -> Result<Expr, PileError> {
        self.expect(Token::Assign)?;

        self.consume()?;

        match self.lookahead.take() {
            None => Err(self.parse_error(
                self.lex_iter.line(),
                &"Expected identifier found end of file.".to_string(),
            )),
            Some((line, Token::Identifier(var))) => {
                Ok(Expr::Assignment { line, var })
            }
            Some((line, token)) => Err(self.parse_error(
                line,
                &format!("Expected identifier found {}.", token),
            )),
        }
    }

    fn using(&mut self) -> Result<Expr, PileError> {
        self.consume()?;

        match &self.lookahead {
            Some((line, Token::String(string))) => Ok(Expr::Use {
                line: *line,
                subprogram: Ast {
                    source: Rc::new(ProgramSource::File(PathBuf::from(string))),
                    expressions: vec![],
                },
            }),
            Some((line, token)) => Err(self.parse_error(
                *line,
                &format!("Expected string found {}.", token),
            )),
            None => Err(self.parse_error(0, "No lookahead found.")),
        }
    }

    fn consume(&mut self) -> Result<(), PileError> {
        self.lookahead = match self.lex_iter.next() {
            Some((line, current_token)) => Some((line, current_token?)),
            None => None,
        };

        Ok(())
    }

    fn expect(&mut self, expected: Token) -> Result<(), PileError> {
        match &self.lookahead {
            Some((line, current_token)) => {
                if *current_token != expected {
                    Err(self.parse_error(
                        *line,
                        &format!(
                            "Expected {} found {}.",
                            expected, current_token
                        ),
                    ))
                } else {
                    Ok(())
                }
            }
            None => Err(self.parse_error(0, "No lookahead found.")),
        }
    }
}

#[cfg(test)]
mod test;
