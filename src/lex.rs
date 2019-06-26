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
}

impl<'a> Lexer<'a> {
    const DEFAULT_CAPACITY: usize = 16;

    pub fn new(input: Chars) -> Lexer {
        Lexer {
            input: input.peekable(),
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
            o => Err(format!("Unkown operator '{}'", o)),
        }
    }

    pub fn next(&mut self) -> Result<Token, String> {
        while let Some(lookahead) = self.input.peek() {
            match lookahead {
                '#' => self.skip_comment(),
                c if c.is_whitespace() => self.skip_whitespace(),
                '"' => return self.string(),
                '+' | '-' | '*' | '/' => return self.operator(),
                c if c.is_alphabetic() => return self.identifier(),
                c => return Err(format!("Unknown char '{}'", c)),
            }
        }

        Ok(Token::Fin)
    }
}
