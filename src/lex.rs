use std::fmt;
use std::iter::Peekable;
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
            Number::Natural(n) => write!(f, "natural '{}'", n),
            Number::Integer(n) => write!(f, "integer '{}'", n),
            Number::Float(n) => write!(f, "float '{}'", n),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    // control flow
    If,
    Def,
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
    // casts
    Natural,
    Integer,
    Float,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::If => write!(f, "if"),
            Operator::Def => write!(f, "def"),
            Operator::Dotimes => write!(f, "dotimes"),
            Operator::While => write!(f, "while"),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Div => write!(f, "/"),
            Operator::Mul => write!(f, "*"),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterEqual => write!(f, ">="),
            Operator::Equal => write!(f, "="),
            Operator::LessEqual => write!(f, "<="),
            Operator::And => write!(f, "and"),
            Operator::Or => write!(f, "or"),
            Operator::Not => write!(f, "not"),
            Operator::Less => write!(f, "<"),
            Operator::Print => write!(f, "print"),
            Operator::Natural => write!(f, "natural"),
            Operator::Integer => write!(f, "integer"),
            Operator::Float => write!(f, "float"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    // keywords
    Begin,
    End,
    Quote,
    Operator(Operator),
    // values
    Number(Number),
    Identifier(String),
    String(String),
    Boolean(bool),
    // eof
    Fin,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Identifier(ident) => write!(f, "identifier '{}'", ident),
            Token::String(s) => write!(f, "string \"{}\"", s),
            Token::Boolean(true) => write!(f, "boolean 'true'"),
            Token::Boolean(false) => write!(f, "boolean 'false'"),
            Token::Begin => write!(f, "token 'begin'"),
            Token::End => write!(f, "token 'end'"),
            Token::Quote => write!(f, "token 'quote'"),
            Token::Operator(o) => write!(f, "operator '{}'", o),
            Token::Fin => write!(f, "'EOF'"),
        }
    }
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line_number: u64,
}

impl<'a> Lexer<'a> {
    const DEFAULT_CAPACITY: usize = 16;

    pub fn new(s: &str) -> Lexer {
        Lexer {
            input: s.chars().peekable(),
            line_number: 1,
        }
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

    fn identifier(&mut self) -> Result<Token, String> {
        let ident = self
            .collect_while(|c| c.is_alphanumeric() || c == '_')
            .to_lowercase();

        Ok(match ident.as_ref() {
            "begin" => Token::Begin,
            "end" => Token::End,
            "if" => Token::Operator(Operator::If),
            "def" => Token::Operator(Operator::Def),
            "dotimes" => Token::Operator(Operator::Dotimes),
            "while" => Token::Operator(Operator::While),
            "quote" => Token::Quote,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "and" => Token::Operator(Operator::And),
            "or" => Token::Operator(Operator::Or),
            "not" => Token::Operator(Operator::Not),
            "print" => Token::Operator(Operator::Print),
            "natural" => Token::Operator(Operator::Natural),
            "integer" => Token::Operator(Operator::Integer),
            "float" => Token::Operator(Operator::Float),
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

    fn string(&mut self) -> Result<Token, String> {
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
                    return Err(String::from(
                        "Missing character after backslash.",
                    ))
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
            Err(error)
        } else {
            Ok(Token::String(string))
        }
    }

    fn parse_number(s: &str) -> Result<Token, String> {
        let digits_only = |s: &str| s.chars().all(|c| c.is_digit(10));

        if digits_only(s) || s.starts_with('+') && digits_only(&s[1..]) {
            match s.parse() {
                Ok(nat) => Ok(Token::Number(Number::Natural(nat))),
                Err(_) => Err(format!(
                    "'{}' is too large to be represented as a number",
                    s
                )),
            }
        } else if s.starts_with('-') && digits_only(&s[1..]) {
            match s.parse() {
                Ok(int) => Ok(Token::Number(Number::Integer(int))),
                Err(_) => Err(format!(
                    "'{}' is too small to be represented as a number",
                    s
                )),
            }
        } else {
            match s.parse() {
                Ok(float) => Ok(Token::Number(Number::Float(float))),
                Err(_) => Err(format!("'{}' isn't a number", s)),
            }
        }
    }

    fn number(&mut self) -> Result<Token, String> {
        let number = self.collect_while(|c| !c.is_whitespace() && c != '#');

        Lexer::parse_number(number.as_ref())
    }

    fn operator(&mut self) -> Result<Token, String> {
        let operator = self.collect_while(|c| !c.is_whitespace() && c != '#');

        if operator.chars().any(|c| c.is_digit(10)) {
            return Lexer::parse_number(operator.as_ref());
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
            o => Err(format!("Unknown operator '{}'", o)),
        }
    }

    pub fn next(&mut self) -> (u64, Result<Token, String>) {
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
                c => {
                    self.consume();
                    Err(format!("Unknown char '{}'", c))
                }
            };

            return (self.line_number, token);
        }

        (self.line_number, Ok(Token::Fin))
    }
}

#[cfg(test)]
mod test;
