use std::fmt;

pub use crate::lex::Number;
pub use crate::lex::Operator;
pub use crate::parse::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Function<'a> {
    Composite(&'a [Expr]),
    Builtin(&'a Operator),
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeValue<'a> {
    Function(Function<'a>),
    Number(Number),
    Identifier(String),
    String(String),
    Boolean(bool),
}

impl<'a> fmt::Display for RuntimeValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeValue::Function(Function::Composite(block)) => {
                let address = block as *const _ as usize;

                write!(f, "function @ 0x{}", address)
            }
            RuntimeValue::Function(Function::Builtin(o)) => {
                write!(f, "function '{}'", o)
            }
            RuntimeValue::Number(Number::Natural(n)) => write!(f, "{}", n),
            RuntimeValue::Number(Number::Integer(i)) => write!(f, "{}", i),
            RuntimeValue::Number(Number::Float(fl)) => write!(f, "{}", fl),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Boolean(true) => write!(f, "true"),
            RuntimeValue::Boolean(false) => write!(f, "false"),
            RuntimeValue::Identifier(_) => Err(std::fmt::Error),
        }
    }
}
