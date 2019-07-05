use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Number {
    Natural(u32),
    Integer(i32),
    Float(f32),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    If,
    Begin,
    End,
    Def,
    Dotimes,
    While,
    Loop,
    Quote,
    Plus,
    Minus,
    Div,
    Mul,
    Number(Number),
    Identifier(String),
    String(String),
    Fin,
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
            "if" => Token::If,
            "begin" => Token::Begin,
            "end" => Token::End,
            "def" => Token::Def,
            "dotimes" => Token::Dotimes,
            "while" => Token::While,
            "loop" => Token::Loop,
            "quote" => Token::Quote,
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

        if unknown_escapes.len() > 0 {
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
        let number = self.collect_while(|c| !c.is_whitespace());

        Lexer::parse_number(number.as_ref())
    }

    fn operator(&mut self) -> Result<Token, String> {
        let operator = self.collect_while(|c| !c.is_whitespace());

        match operator.as_ref() {
            "+" => Ok(Token::Plus),
            "-" => Ok(Token::Minus),
            "*" => Ok(Token::Mul),
            "/" => Ok(Token::Div),
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
                '+' | '-' | '*' | '/' => self.operator(),
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
