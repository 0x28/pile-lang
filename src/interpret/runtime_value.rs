use std::cmp::Ordering;
use std::fmt;

pub use crate::lex::Number;
pub use crate::lex::Operator;
pub use crate::parse::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Function<'a> {
    Composite(&'a [Expr]),
    Builtin(&'a Operator),
}

impl<'a> fmt::Display for Function<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Function::Composite(block) => write!(f, "function @ {:p}", block),
            Function::Builtin(o) => write!(f, "function '{}'", o),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeValue<'a> {
    Function(Function<'a>),
    Number(Number),
    Identifier(String),
    String(String),
    Boolean(bool),
}

impl<'a> RuntimeValue<'a> {
    pub fn type_fmt(&self) -> String {
        match self {
            RuntimeValue::Function(func) => format!("{}", func),
            RuntimeValue::Number(n) => format!("{}", n),
            RuntimeValue::String(s) => format!("string '{}'", s),
            RuntimeValue::Boolean(true) => format!("boolean '{}'", true),
            RuntimeValue::Boolean(false) => format!("boolean '{}'", false),
            RuntimeValue::Identifier(ident) => {
                format!("identifier '{}'", ident)
            }
        }
    }
}

impl<'a> fmt::Display for RuntimeValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeValue::Function(func) => write!(f, "{}", func),
            RuntimeValue::Number(Number::Natural(n)) => write!(f, "{}", n),
            RuntimeValue::Number(Number::Integer(i)) => write!(f, "{}", i),
            RuntimeValue::Number(Number::Float(fl)) => write!(f, "{}", fl),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Boolean(true) => write!(f, "true"),
            RuntimeValue::Boolean(false) => write!(f, "false"),
            RuntimeValue::Identifier(ident) => write!(f, "{}", ident),
        }
    }
}

impl<'a> PartialOrd for RuntimeValue<'a> {
    fn partial_cmp(&self, other: &RuntimeValue) -> Option<Ordering> {
        match (self, other) {
            (
                RuntimeValue::Number(Number::Natural(left)),
                RuntimeValue::Number(Number::Natural(right)),
            ) => Some(left.cmp(right)),
            (
                RuntimeValue::Number(Number::Integer(left)),
                RuntimeValue::Number(Number::Integer(right)),
            ) => Some(left.cmp(right)),
            (
                RuntimeValue::Number(Number::Float(left)),
                RuntimeValue::Number(Number::Float(right)),
            ) => left.partial_cmp(right),
            (RuntimeValue::String(left), RuntimeValue::String(right)) => {
                Some(left.cmp(right))
            }
            _ => None,
        }
    }
}
