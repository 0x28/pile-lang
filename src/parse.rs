use crate::lex::Lexer;
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
pub struct ParsedAst(Ast);

impl ParsedAst {
    pub fn as_ast(self) -> Ast {
        self.0
    }
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
        locals: Vec<String>,
        expressions: Rc<Vec<Expr>>,
    },
    Use {
        line: u64,
        subprogram: Ast,
    },
    Save {
        line: u64,
        var: String,
    },
    Restore {
        line: u64,
        var: String,
    },
}

impl Expr {
    pub fn lines(&self) -> (u64, u64) {
        match self {
            Self::Atom { line, .. } => (*line, *line),
            Self::Assignment { line, .. } => (*line, *line),
            Self::Block { begin, end, .. } => (*begin, *end),
            Self::Use { line, .. } => (*line, *line),
            Self::Save { line, .. } => (*line, *line),
            Self::Restore { line, .. } => (*line, *line),
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

    pub fn parse(mut self) -> Result<ParsedAst, PileError> {
        let mut program = vec![];

        loop {
            self.consume()?;

            let expr = match self.lookahead {
                Some((line, Token::End)) => {
                    return Err(self.parse_error(line, "Unmatched 'end'."));
                }
                Some((_, Token::Begin)) => self.block()?,
                Some((_, Token::Let)) => self.block()?,
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

        Ok(ParsedAst(Ast {
            source: Rc::clone(self.lexer.source()),
            expressions: program,
        }))
    }

    fn parse_error(&self, line: u64, msg: &str) -> PileError {
        PileError::new(
            Rc::clone(self.lexer.source()),
            (line, line),
            msg.to_owned(),
        )
    }

    fn locals(&mut self) -> Result<Vec<String>, PileError> {
        self.expect(Token::Let)?;
        self.consume()?;
        self.expect(Token::BracketLeft)?;

        let mut result = vec![];

        loop {
            self.consume()?;

            match &self.lookahead {
                Some((_, Token::BracketRight)) => {
                    break;
                }
                Some((_, Token::Identifier(id))) => result.push(id.clone()),
                None => {
                    return Err(self.parse_error(
                        self.lexer.line(),
                        "Expected ']' found end of file.",
                    ));
                }
                Some((_, token)) => {
                    return Err(self.parse_error(
                        self.lexer.line(),
                        &format!("Expected identifier found {}.", token),
                    ))
                }
            }
        }

        self.expect(Token::BracketRight)?;

        Ok(result)
    }

    fn block(&mut self) -> Result<Expr, PileError> {
        let (begin, locals) = if let Some((line, Token::Let)) = self.lookahead {
            (line, self.locals()?)
        } else {
            self.expect(Token::Begin)?;
            (self.lookahead.as_ref().unwrap().0, vec![])
        };

        let end;

        let mut block = vec![];

        loop {
            self.consume()?;

            match self.lookahead {
                None => {
                    return Err(self.parse_error(
                        self.lexer.line(),
                        "Expected 'end' found end of file.",
                    ));
                }

                Some((line, Token::End)) => {
                    end = line;
                    break;
                }
                Some((_, Token::Begin)) => block.push(self.block()?),
                Some((_, Token::Let)) => block.push(self.block()?),
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
            locals,
            expressions: Rc::new(block),
        })
    }

    fn assign(&mut self) -> Result<Expr, PileError> {
        self.expect(Token::Assign)?;

        self.consume()?;

        match self.lookahead.take() {
            None => Err(self.parse_error(
                self.lexer.line(),
                "Expected identifier found end of file.",
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
        self.lookahead = match self.lexer.next() {
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
