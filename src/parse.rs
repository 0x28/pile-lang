use crate::lex::Lexer;
use crate::lex::Token;
use crate::pile_error::PileError;
use crate::cli::ProgramSource;

use std::rc::Rc;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub source: ProgramSource,
    pub expressions: Vec<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Atom {
        line: u64,
        token: Token,
    },
    Quoted {
        line: u64,
        token: Token,
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
            Self::Quoted { line, .. } => (*line, *line),
            Self::Block { begin, end, .. } => (*begin, *end),
            Self::Use { line, .. } => (*line, *line),
        }
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    lookahead: Option<(u64, Token)>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
            lookahead: None,
        }
    }

    pub fn parse(mut self) -> Result<Ast, PileError> {
        let mut program = vec![];

        loop {
            self.consume()?;

            match self.lookahead {
                Some((_, Token::Fin)) => break,
                Some((line, Token::End)) => {
                    return Err(self.parse_error(line, "Unmatched 'end'."));
                }
                Some((_, Token::Begin)) => program.push(self.block()?),
                Some((_, Token::Quote)) => program.push(self.quote()?),
                Some((_, Token::Use)) => program.push(self.using()?),
                Some((line, _)) => program.push(Expr::Atom {
                    line,
                    token: self.lookahead.take().unwrap().1,
                }),
                None => continue,
            }
        }

        Ok(Ast {
            source: self.lexer.source(),
            expressions: program,
        })
    }

    fn parse_error(&self, line: u64, msg: &str) -> PileError {
        PileError::new(self.lexer.source(), (line, line), msg.to_owned())
    }

    fn block(&mut self) -> Result<Expr, PileError> {
        self.expect(Token::Begin)?;

        let begin = self.lookahead.as_ref().unwrap().0;
        let end;

        let mut block = vec![];

        loop {
            self.consume()?;

            match self.lookahead {
                Some((line, Token::Fin)) => {
                    return Err(self.parse_error(
                        line,
                        &format!("Expected 'end' found {}.", Token::Fin),
                    ));
                }
                Some((line, Token::End)) => {
                    end = line;
                    break;
                }
                Some((_, Token::Begin)) => block.push(self.block()?),
                Some((_, Token::Quote)) => block.push(self.quote()?),
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
                None => continue,
            }
        }

        Ok(Expr::Block {
            begin,
            end,
            expressions: Rc::new(block),
        })
    }

    fn quote(&mut self) -> Result<Expr, PileError> {
        self.expect(Token::Quote)?;

        self.consume()?;

        match &self.lookahead {
            Some((line, Token::Fin)) => {
                Err(self
                    .parse_error(*line, &format!("Unexpected {}", Token::Fin)))
            }
            Some((_, Token::Begin)) => self.block(),
            Some((line, Token::End)) => {
                Err(self
                    .parse_error(*line, &format!("Unexpected {}", Token::End)))
            }
            Some((line, Token::Use)) => {
                Err(self
                    .parse_error(*line, "'use' isn't allowed inside quotes."))
            }
            Some((line, _)) => Ok(Expr::Quoted {
                line: *line,
                token: self.lookahead.take().unwrap().1,
            }),
            None => Err(self.parse_error(0, "No lookahead found.")),
        }
    }

    fn using(&mut self) -> Result<Expr, PileError> {
        self.consume()?;

        match &self.lookahead {
            Some((line, Token::String(string))) => Ok(Expr::Use {
                line: *line,
                subprogram: Ast {
                    source: ProgramSource::File(PathBuf::from(string)),
                    expressions: vec![],
                }
            }),
            Some((line, token)) => Err(self.parse_error(
                *line,
                &format!("Expected string found {}.", token),
            )),
            None => Err(self.parse_error(0, "No lookahead found.")),
        }
    }

    fn consume(&mut self) -> Result<(), PileError> {
        let (line, current_token) = self.lexer.next();
        let current_token = current_token?;

        self.lookahead = Some((line, current_token));

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
