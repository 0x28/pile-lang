use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum Number {
    Natural(u32),
    Integer(i32),
    Float(f32),
}

#[derive(Debug)]
pub enum Token {
    Begin,
    End,
    Def,
    Dotimes,
    While,
    Loop,
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

    pub fn new(input: Chars) -> Lexer {
        Lexer {
            input: input.peekable(),
            line_number: 1,
        }
    }

    fn skip<P>(&mut self, predicate: P)
    where
        P: Fn(&char) -> bool,
    {
        while let Some(lookahead) = self.input.peek() {
            if predicate(lookahead) {
                self.consume()
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        self.skip(|&c| c != '\n');
    }

    fn skip_whitespace(&mut self) {
        self.skip(|c| c.is_whitespace());
    }

    fn consume(&mut self) {
        self.input.next();
    }

    fn identifier(&mut self) -> Result<Token, String> {
        let mut ident = String::with_capacity(Lexer::DEFAULT_CAPACITY);

        while let Some(lookahead) = self.input.peek() {
            if lookahead.is_alphanumeric() {
                ident.push(*lookahead);
                self.consume();
            } else {
                break;
            }
        }

        let ident = ident.to_lowercase();

        Ok(match ident.as_ref() {
            "begin" => Token::Begin,
            "end" => Token::End,
            "def" => Token::Def,
            "dotimes" => Token::Dotimes,
            "while" => Token::While,
            "loop" => Token::Loop,
            _ => Token::Identifier(ident),
        })
    }

    fn escape_char(&mut self) -> Result<char, String> {
        let &c = match self.input.peek() {
            Some(c) => c,
            _ => {
                return Err(String::from("Missing character after backspace."))
            }
        };
        self.consume();

        match c {
            't' => Ok('\t'),
            'r' => Ok('\r'),
            'n' => Ok('\n'),
            '0' => Ok('\0'),
            '"' => Ok('"'),
            c => Err(format!("Unknown escape char '{}'", c)),
        }
    }

    fn string(&mut self) -> Result<Token, String> {
        let mut string = String::with_capacity(Lexer::DEFAULT_CAPACITY);

        self.consume();
        while let Some(&lookahead) = self.input.peek() {
            self.consume();
            match lookahead {
                '\\' => {
                    let escaped = self.escape_char()?;
                    string.push(escaped)
                }
                '"' => break,
                c => string.push(c),
            }
        }

        Ok(Token::String(string))
    }

    fn number(&mut self) -> Result<Token, String> {
        let mut number = String::with_capacity(Lexer::DEFAULT_CAPACITY);

        while let Some(&lookahead) = self.input.peek() {
            self.consume();
            match lookahead {
                c if c.is_whitespace() => break,
                c => number.push(c),
            }
        }

        if let Ok(nat) = number.parse() {
            Ok(Token::Number(Number::Natural(nat)))
        } else if let Ok(int) = number.parse() {
            Ok(Token::Number(Number::Integer(int)))
        } else if let Ok(float) = number.parse() {
            Ok(Token::Number(Number::Float(float)))
        } else {
            Err(format!("'{}' isn't a number", number))
        }
    }

    fn operator(&mut self) -> Result<Token, String> {
        let mut operator = String::with_capacity(Lexer::DEFAULT_CAPACITY);

        while let Some(&lookahead) = self.input.peek() {
            self.consume();
            match lookahead {
                c if c.is_whitespace() => break,
                c => operator.push(c),
            }
        }

        match operator.as_ref() {
            "+" => Ok(Token::Plus),
            "-" => Ok(Token::Minus),
            "*" => Ok(Token::Mul),
            "/" => Ok(Token::Div),
            n => self
                .number()
                .map_err(|_err| format!("Unknown operator '{}'", n)),
        }
    }

    pub fn next(&mut self) -> (u64, Result<Token, String>) {
        while let Some(lookahead) = self.input.peek() {
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
                c if c.is_alphabetic() => self.identifier(),
                c => Err(format!("Unknown char '{}'", c)),
            };

            return (self.line_number, token);
        }

        (self.line_number, Ok(Token::Fin))
    }
}
