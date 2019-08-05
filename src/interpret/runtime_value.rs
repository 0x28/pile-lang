use std::fmt;

pub use crate::lex::Number;
pub use crate::lex::Operator;
pub use crate::parse::Expr;

#[derive(Debug, PartialEq)]
pub enum RuntimeValue<'a> {
    Function(&'a [Expr]),
    Operator(Operator),
    Number(Number),
    Identifier(String),
    String(String),
    Boolean(bool),
}

impl<'a> fmt::Display for RuntimeValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeValue::Function(block) => {
                let address = block as *const _ as usize;

                write!(f, "function @ 0x{}", address)
            }
            RuntimeValue::Operator(o) => write!(f, "function '{}'", o),
            RuntimeValue::Number(Number::Natural(n)) => write!(f, "{}", n),
            RuntimeValue::Number(Number::Integer(i)) => write!(f, "{}", i),
            RuntimeValue::Number(Number::Float(fl)) => write!(f, "{}", fl),
            RuntimeValue::String(s) => writeln!(f, "{}", s),
            RuntimeValue::Boolean(true) => writeln!(f, "true"),
            RuntimeValue::Boolean(false) => writeln!(f, "false"),
            RuntimeValue::Identifier(_) => Err(std::fmt::Error),
        }
    }
}