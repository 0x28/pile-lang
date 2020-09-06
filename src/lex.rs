use crate::pile_error::PileError;
use crate::program_source::ProgramSource;

use std::fmt;
use std::iter::Iterator;
use std::iter::Peekable;
use std::rc::Rc;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Natural(u32),
    Integer(i32),
    Float(f32),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Natural(n) => write!(f, "{}", n),
            Number::Integer(n) => write!(f, "{}", n),
            Number::Float(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    // control flow
    If,
    Dotimes,
    While,
    // arithmetic
    Plus,
    Minus,
    Div,
    Mul,
    // predicates
    Greater,
    GreaterEqual,
    Equal,
    LessEqual,
    Less,
    And,
    Or,
    Not,
    // builtins
    Print,
    Assert,
    Dup,
    Drop,
    Swap,
    // casts
    Natural,
    Integer,
    Float,
    // string operators
    Concat,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::If => "if",
                Operator::Dotimes => "dotimes",
                Operator::While => "while",
                Operator::Plus => "+",
                Operator::Minus => "-",
                Operator::Div => "/",
                Operator::Mul => "*",
                Operator::Greater => ">",
                Operator::GreaterEqual => ">=",
                Operator::Equal => "=",
                Operator::LessEqual => "<=",
                Operator::And => "and",
                Operator::Or => "or",
                Operator::Not => "not",
                Operator::Less => "<",
                Operator::Print => "print",
                Operator::Assert => "assert",
                Operator::Dup => "dup",
                Operator::Drop => "drop",
                Operator::Swap => "swap",
                Operator::Natural => "natural",
                Operator::Integer => "integer",
                Operator::Float => "float",
                Operator::Concat => "concat",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    // keywords
    Begin,
    End,
    Let,
    BracketLeft,
    BracketRight,
    Assign,
    Operator(Operator),
    // values
    Number(Number),
    Identifier(String),
    String(String),
    Boolean(bool),
    // use
    Use,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(Number::Natural(n)) => write!(f, "natural '{}'", n),
            Token::Number(Number::Integer(i)) => write!(f, "integer '{}'", i),
            Token::Number(Number::Float(fl)) => write!(f, "float '{}'", fl),
            Token::Identifier(ident) => write!(f, "identifier '{}'", ident),
            Token::String(s) => write!(f, "string \"{}\"", s),
            Token::Boolean(true) => write!(f, "boolean 'true'"),
            Token::Boolean(false) => write!(f, "boolean 'false'"),
            Token::Begin => write!(f, "token 'begin'"),
            Token::End => write!(f, "token 'end'"),
            Token::Let => write!(f, "token 'let'"),
            Token::BracketLeft => write!(f, "token '['"),
            Token::BracketRight => write!(f, "token ']'"),
            Token::Assign => write!(f, "token '->'"),
            Token::Operator(o) => write!(f, "operator '{}'", o),
            Token::Use => write!(f, "token 'use'"),
        }
    }
}

pub struct Lexer<'a> {
    source: Rc<ProgramSource>,
    input: Peekable<Chars<'a>>,
    line_number: u64,
}

type LexerItem = (u64, Result<Token, PileError>);

impl<'a> Lexer<'a> {
    const DEFAULT_CAPACITY: usize = 16;

    pub fn new(text: &str, source: Rc<ProgramSource>) -> Lexer {
        Lexer {
            source,
            input: text.chars().peekable(),
            line_number: 1,
        }
    }

    pub fn source(&self) -> &Rc<ProgramSource> {
        &self.source
    }

    pub fn line(&self) -> u64 {
        self.line_number
    }

    fn lex_error(&self, msg: &str) -> PileError {
        PileError::new(
            Rc::clone(self.source()),
            (self.line_number, self.line_number),
            msg.to_owned(),
        )
    }

    fn skip<P>(&mut self, predicate: P)
    where
        P: Fn(char) -> bool,
    {
        while let Some(&lookahead) = self.input.peek() {
            if predicate(lookahead) {
                self.consume()
            } else {
                break;
            }
        }
    }

    fn collect_while<P>(&mut self, predicate: P) -> String
    where
        P: Fn(char) -> bool,
    {
        let mut word = String::with_capacity(Lexer::DEFAULT_CAPACITY);

        while let Some(&lookahead) = self.input.peek() {
            if predicate(lookahead) {
                word.push(lookahead);
                self.consume();
            } else {
                break;
            }
        }

        word
    }

    fn skip_comment(&mut self) {
        self.skip(|c| c != '\n');
    }

    fn skip_whitespace(&mut self) {
        self.skip(|c| c.is_whitespace() && c != '\n');
    }

    fn consume(&mut self) {
        self.input.next();
    }

    fn identifier(&mut self) -> Result<Token, PileError> {
        let ident = self
            .collect_while(|c| c.is_alphanumeric() || c == '_')
            .to_lowercase();

        Ok(match ident.as_ref() {
            "begin" => Token::Begin,
            "end" => Token::End,
            "if" => Token::Operator(Operator::If),
            "dotimes" => Token::Operator(Operator::Dotimes),
            "while" => Token::Operator(Operator::While),
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "and" => Token::Operator(Operator::And),
            "or" => Token::Operator(Operator::Or),
            "not" => Token::Operator(Operator::Not),
            "print" => Token::Operator(Operator::Print),
            "assert" => Token::Operator(Operator::Assert),
            "dup" => Token::Operator(Operator::Dup),
            "drop" => Token::Operator(Operator::Drop),
            "swap" => Token::Operator(Operator::Swap),
            "natural" => Token::Operator(Operator::Natural),
            "integer" => Token::Operator(Operator::Integer),
            "float" => Token::Operator(Operator::Float),
            "concat" => Token::Operator(Operator::Concat),
            "use" => Token::Use,
            "let" => Token::Let,
            _ => Token::Identifier(ident),
        })
    }

    fn escape_char(c: char) -> Result<char, char> {
        match c {
            't' => Ok('\t'),
            'r' => Ok('\r'),
            'n' => Ok('\n'),
            '0' => Ok('\0'),
            '"' => Ok('"'),
            c => Err(c),
        }
    }

    fn string(&mut self) -> Result<Token, PileError> {
        let mut string = String::with_capacity(Lexer::DEFAULT_CAPACITY);
        let mut unknown_escapes = vec![];

        self.consume();
        while let Some(&lookahead) = self.input.peek() {
            self.consume();
            match (lookahead, self.input.peek()) {
                ('\\', Some(&c)) => {
                    self.consume();
                    match Lexer::escape_char(c) {
                        Ok(escaped) => string.push(escaped),
                        Err(unknown) => unknown_escapes.push(unknown),
                    }
                }
                ('\\', None) => {
                    return Err(
                        self.lex_error("Missing character after backslash.")
                    )
                }
                ('"', _) => break,
                (c, _) => string.push(c),
            }
        }

        if !unknown_escapes.is_empty() {
            let mut error = String::from("Unknown escape chars:");
            for unknown in unknown_escapes {
                error.push_str(format!(" '\\{}'", unknown).as_ref());
            }
            Err(self.lex_error(&error))
        } else {
            Ok(Token::String(string))
        }
    }

    fn parse_number(&self, s: &str) -> Result<Token, PileError> {
        let digits_only = |s: &str| s.chars().all(|c| c.is_digit(10));

        if digits_only(s) || s.starts_with('+') && digits_only(&s[1..]) {
            match s.parse() {
                Ok(nat) => Ok(Token::Number(Number::Natural(nat))),
                Err(_) => Err(self.lex_error(&format!(
                    "'{}' is too large to be represented as a number",
                    s
                ))),
            }
        } else if s.starts_with('-') && digits_only(&s[1..]) {
            match s.parse() {
                Ok(int) => Ok(Token::Number(Number::Integer(int))),
                Err(_) => Err(self.lex_error(&format!(
                    "'{}' is too small to be represented as a number",
                    s
                ))),
            }
        } else {
            match s.parse() {
                Ok(float) => Ok(Token::Number(Number::Float(float))),
                Err(_) => {
                    Err(self.lex_error(&format!("'{}' isn't a number", s)))
                }
            }
        }
    }

    fn is_separating(c: char) -> bool {
        match c {
            '#' | '[' | ']' => true,
            _ => false,
        }
    }

    fn number(&mut self) -> Result<Token, PileError> {
        let number = self
            .collect_while(|c| !c.is_whitespace() && !Lexer::is_separating(c));

        self.parse_number(number.as_ref())
    }

    fn operator(&mut self) -> Result<Token, PileError> {
        let operator = self
            .collect_while(|c| !c.is_whitespace() && !Lexer::is_separating(c));

        if operator.chars().any(|c| c.is_digit(10)) {
            return self.parse_number(operator.as_ref());
        }

        match operator.as_ref() {
            "+" => Ok(Token::Operator(Operator::Plus)),
            "-" => Ok(Token::Operator(Operator::Minus)),
            "*" => Ok(Token::Operator(Operator::Mul)),
            "/" => Ok(Token::Operator(Operator::Div)),
            ">" => Ok(Token::Operator(Operator::Greater)),
            ">=" => Ok(Token::Operator(Operator::GreaterEqual)),
            "=" => Ok(Token::Operator(Operator::Equal)),
            "<=" => Ok(Token::Operator(Operator::LessEqual)),
            "<" => Ok(Token::Operator(Operator::Less)),
            "->" => Ok(Token::Assign),
            o => Err(self.lex_error(&format!("Unknown operator '{}'", o))),
        }
    }

    fn next(&mut self) -> Option<LexerItem> {
        while let Some(&lookahead) = self.input.peek() {
            let token = match lookahead {
                '#' => {
                    self.skip_comment();
                    continue;
                }
                '\n' => {
                    self.line_number += 1;
                    self.consume();
                    continue;
                }
                c if c.is_whitespace() => {
                    self.skip_whitespace();
                    continue;
                }
                '"' => self.string(),
                '0'..='9' => self.number(),
                '+' | '-' | '*' | '/' | '=' | '<' | '>' => self.operator(),
                c if c.is_alphabetic() || c == '_' => self.identifier(),
                '[' => {
                    self.consume();
                    Ok(Token::BracketLeft)
                }
                ']' => {
                    self.consume();
                    Ok(Token::BracketRight)
                }
                c => {
                    self.consume();
                    Err(self.lex_error(&format!("Unknown char '{}'", c)))
                }
            };

            return Some((self.line_number, token));
        }

        None
    }
}

pub struct LexerIter<'a> {
    lexer: Lexer<'a>,
}

impl<'a> LexerIter<'a> {
    pub fn source(&self) -> &Rc<ProgramSource> {
        &self.lexer.source()
    }

    pub fn line(&self) -> u64 {
        self.lexer.line()
    }
}

impl<'a> Iterator for LexerIter<'a> {
    type Item = LexerItem;

    fn next(&mut self) -> Option<LexerItem> {
        self.lexer.next()
    }
}

impl<'a> IntoIterator for Lexer<'a> {
    type Item = LexerItem;
    type IntoIter = LexerIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        LexerIter { lexer: self }
    }
}

#[cfg(test)]
mod test;
