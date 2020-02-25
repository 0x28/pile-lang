use std::cmp::Ordering;
use std::fmt;
use std::rc::Rc;

pub use crate::lex::Number;
pub use crate::lex::Operator;
pub use crate::parse::Expr;

use crate::program_source::ProgramSource;

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub source: Rc<ProgramSource>,
    pub exprs: Rc<Vec<Expr>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeValue {
    Function(Function),
    Number(Number),
    String(String),
    Boolean(bool),
}

impl RuntimeValue {
    pub fn type_fmt(&self) -> String {
        match self {
            RuntimeValue::Function { .. } => "function".to_string(),
            RuntimeValue::Number(Number::Natural(n)) => {
                format!("natural '{}'", n)
            }
            RuntimeValue::Number(Number::Integer(i)) => {
                format!("integer '{}'", i)
            }
            RuntimeValue::Number(Number::Float(fl)) => {
                format!("float '{}'", fl)
            }
            RuntimeValue::String(s) => format!("string '{}'", s),
            RuntimeValue::Boolean(true) => format!("boolean '{}'", true),
            RuntimeValue::Boolean(false) => format!("boolean '{}'", false),
        }
    }
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeValue::Function(Function { exprs, .. }) => {
                write!(f, "function @ {:p}", exprs)
            }
            RuntimeValue::Number(Number::Natural(n)) => write!(f, "{}", n),
            RuntimeValue::Number(Number::Integer(i)) => write!(f, "{}", i),
            RuntimeValue::Number(Number::Float(fl)) => write!(f, "{}", fl),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Boolean(true) => write!(f, "true"),
            RuntimeValue::Boolean(false) => write!(f, "false"),
        }
    }
}

impl PartialOrd for RuntimeValue {
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
