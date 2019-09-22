use crate::lex::Lexer;
use crate::lex::Token;

#[derive(Debug, PartialEq)]
pub struct Ast {
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
        expressions: Vec<Expr>,
    },
}

impl Expr {
    pub fn lines(&self) -> (u64, u64) {
        match self {
            Self::Atom { line, .. } => (*line, *line),
            Self::Quoted { line, .. } => (*line, *line),
            Self::Block { begin, end, .. } => (*begin, *end),
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

    pub fn parse(mut self) -> Result<Ast, String> {
        let mut program = vec![];

        loop {
            self.consume()?;

            match self.lookahead {
                Some((_, Token::Fin)) => break,
                Some((line, Token::End)) => {
                    return Err(format!("Line {}: Unmatched 'end'.", line))
                }
                Some((_, Token::Begin)) => program.push(self.block()?),
                Some((_, Token::Quote)) => program.push(self.quote()?),
                Some((line, _)) => program.push(Expr::Atom {
                    line,
                    token: self.lookahead.take().unwrap().1,
                }),
                None => continue,
            }
        }

        Ok(Ast {
            expressions: program,
        })
    }

    fn block(&mut self) -> Result<Expr, String> {
        self.expect(Token::Begin)?;

        let begin = self.lookahead.as_ref().unwrap().0;
        let end;

        let mut block = vec![];

        loop {
            self.consume()?;

            match self.lookahead {
                Some((line, Token::Fin)) => {
                    return Err(format!(
                        "Line {}: Expected 'end' found {}.",
                        line,
                        Token::Fin
                    ))
                }
                Some((line, Token::End)) => {
                    end = line;
                    break;
                }
                Some((_, Token::Begin)) => block.push(self.block()?),
                Some((_, Token::Quote)) => block.push(self.quote()?),
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
            expressions: block,
        })
    }

    fn quote(&mut self) -> Result<Expr, String> {
        self.expect(Token::Quote)?;

        self.consume()?;

        match &self.lookahead {
            Some((line, Token::Fin)) => {
                Err(format!("Line {}: Unexpected {}", line, Token::Fin))
            }
            Some((_, Token::Begin)) => self.block(),
            Some((line, Token::End)) => {
                Err(format!("Line {}: Unexpected {}", line, Token::End))
            }
            Some((line, _)) => Ok(Expr::Quoted {
                line: *line,
                token: self.lookahead.take().unwrap().1,
            }),
            None => Err(String::from("No lookahead found.")),
        }
    }

    fn consume(&mut self) -> Result<(), String> {
        let (line, current_token) = self.lexer.next();
        let current_token = current_token?;

        self.lookahead = Some((line, current_token));

        Ok(())
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        match &self.lookahead {
            Some((line, current_token)) => {
                if *current_token != expected {
                    Err(format!(
                        "Line {}: expected {} found {}.",
                        line, expected, current_token
                    ))
                } else {
                    Ok(())
                }
            }
            None => Err(String::from("No lookahead found.")),
        }
    }
}

#[cfg(test)]
mod test;
