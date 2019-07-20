use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Natural(u32),
    Integer(i32),
    Float(f32),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    // control flow
    If,
    Def,
    Dotimes,
    While,
    Loop,
    Quote,
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
}

#[derive(Debug, PartialEq)]
pub enum Token {
    // keywords
    Begin,
    End,
    Operator(Operator),
    // values
    Number(Number),
    Identifier(String),
    String(String),
    // eof
    Fin,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(Number::Natural(n)) => write!(f, "natural '{}'", n),
            Token::Number(Number::Integer(n)) => write!(f, "integer '{}'", n),
            Token::Number(Number::Float(n)) => write!(f, "float '{}'", n),
            Token::Identifier(ident) => write!(f, "identifier '{}'", ident),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Begin => write!(f, "'begin'"),
            Token::End => write!(f, "'end'"),
            Token::Operator(Operator::If) => write!(f, "'if'"),
            Token::Operator(Operator::Def) => write!(f, "'def'"),
            Token::Operator(Operator::Dotimes) => write!(f, "'dotimes'"),
            Token::Operator(Operator::While) => write!(f, "'while'"),
            Token::Operator(Operator::Loop) => write!(f, "'loop'"),
            Token::Operator(Operator::Quote) => write!(f, "'quote'"),
            Token::Operator(Operator::Plus) => write!(f, "'+'"),
            Token::Operator(Operator::Minus) => write!(f, "'-'"),
            Token::Operator(Operator::Div) => write!(f, "'/'"),
            Token::Operator(Operator::Mul) => write!(f, "'*'"),
            Token::Operator(Operator::Greater) => write!(f, "'>'"),
            Token::Operator(Operator::GreaterEqual) => write!(f, "'>='"),
            Token::Operator(Operator::Equal) => write!(f, "'='"),
            Token::Operator(Operator::LessEqual) => write!(f, "'<='"),
            Token::Operator(Operator::Less) => write!(f, "'<'"),
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
            "loop" => Token::Operator(Operator::Loop),
            "quote" => Token::Operator(Operator::Quote),
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
                        "Missing character after backspace.",
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
        if let Ok(nat) = s.parse() {
            Ok(Token::Number(Number::Natural(nat)))
        } else if let Ok(int) = s.parse() {
            Ok(Token::Number(Number::Integer(int)))
        } else if let Ok(float) = s.parse() {
            Ok(Token::Number(Number::Float(float)))
        } else {
            Err(format!("'{}' isn't a number", s))
        }
    }

    fn number(&mut self) -> Result<Token, String> {
        let number = self.collect_while(|c| !c.is_whitespace() && c != '#');

        Lexer::parse_number(number.as_ref())
    }

    fn operator(&mut self) -> Result<Token, String> {
        let operator = self.collect_while(|c| !c.is_whitespace() && c != '#');

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
            n => Lexer::parse_number(operator.as_ref())
                .map_err(|_err| format!("Unknown operator '{}'", n)),
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
                '0'...'9' => self.number(),
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
