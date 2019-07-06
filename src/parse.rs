use crate::lex::Lexer;
use crate::lex::Token;

#[derive(Debug)]
pub enum Ast {
    Program(Vec<Expr>)
}

#[derive(Debug)]
pub enum Expr {
    Atom(Token),
    Block(Vec<Expr>),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    // TODO lookahead: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Result<Ast, String> {
        // TODO self.expect(Token::Begin)?;
        Ok(Ast::Program(vec![]))
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        let (line, current_token) = self.lexer.next();
        let current_token = current_token?;

        if current_token != expected {
            Err(format!(
                "Line {}: expected {} found {}.",
                line, expected, current_token
            ))
        } else {
            Ok(())
        }
    }
}
