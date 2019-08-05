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
