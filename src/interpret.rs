use crate::lex::Number;
use crate::lex::Operator;
use crate::lex::Token;
use crate::parse::Ast;
use crate::parse::Expr;

#[derive(Debug, PartialEq)]
pub enum RuntimeValue<'a> {
    Function(&'a [Expr]),
    Operator(Operator),
    Number(Number),
    Identifier(String),
    String(String),
}

pub struct Interpreter<'a> {
    program: Ast,
    stack: Vec<RuntimeValue<'a>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(program: Ast) -> Interpreter<'a> {
        Interpreter {
            program,
            stack: vec![],
        }
    }

    pub fn run(&mut self) -> Result<&'a RuntimeValue, String> {
        Interpreter::call(&mut self.stack, &self.program.expressions)
    }

    fn call(
        stack: &'a mut Vec<RuntimeValue<'a>>,
        expressions: &'a [Expr],
    ) -> Result<&'a RuntimeValue<'a>, String> {
        for expr in expressions {
            match expr {
                Expr::Atom { token: atom, .. } => match atom {
                    Token::Operator(op) => Interpreter::apply(op, stack)?,
                    Token::Number(num) => {
                        stack.push(RuntimeValue::Number(num.clone()));
                    }
                    Token::Identifier(ident) => (),
                    Token::String(string) => {
                        stack.push(RuntimeValue::String(string.clone()))
                    }
                    token => {
                        return Err(format!("Unexpected token \'{}\'", token))
                    }
                },
                Expr::Block(expr) => stack.push(RuntimeValue::Function(&expr)),
            }
            println!("stack: {:?}", stack);
        }

        match stack.last() {
            Some(value) => Ok(value),
            None => Err(String::from("Stack underflow")),
        }
    }

    fn apply_numeric<N, I, F>(
        op_natural: N,
        op_integer: I,
        op_float: F,
        stack: &mut Vec<RuntimeValue>,
    ) -> Result<(), String>
    where
        N: Fn(u32, u32) -> Result<u32, String>,
        I: Fn(i32, i32) -> Result<i32, String>,
        F: Fn(f32, f32) -> f32,
    {
        let right = stack.pop().ok_or("Stack underflow".to_owned())?;
        let left = stack.pop().ok_or("Stack underflow".to_owned())?;

        let (left, right) = match (left, right) {
            (RuntimeValue::Number(lhs), RuntimeValue::Number(rhs)) => {
                (lhs, rhs)
            }
            (lhs, rhs) => {
                return Err(format!("Type error: {:?}, {:?}", lhs, rhs))
            }
        };

        let result = match (left, right) {
            (Number::Natural(lhs), Number::Natural(rhs)) => {
                Number::Natural(op_natural(lhs, rhs)?)
            }
            (Number::Integer(lhs), Number::Integer(rhs)) => {
                Number::Integer(op_integer(lhs, rhs)?)
            }
            (Number::Float(lhs), Number::Float(rhs)) => {
                Number::Float(op_float(lhs, rhs))
            }
            (lhs, rhs) => {
                return Err(format!(
                    "Numeric type mismatch: {:?}, {:?}",
                    lhs, rhs
                ))
            }
        };

        stack.push(RuntimeValue::Number(result));

        Ok(())
    }

    fn apply(
        op: &Operator,
        stack: &mut Vec<RuntimeValue>,
    ) -> Result<(), String> {
        return match op {
            Operator::Plus => Interpreter::apply_numeric(
                |a, b| a.checked_add(b).ok_or("Numeric overflow".to_owned()),
                |a, b| a.checked_add(b).ok_or("Numeric overflow".to_owned()),
                |a, b| a + b,
                stack,
            ),
            Operator::Minus => Interpreter::apply_numeric(
                |a, b| a.checked_sub(b).ok_or("Numeric overflow".to_owned()),
                |a, b| a.checked_sub(b).ok_or("Numeric overflow".to_owned()),
                |a, b| a - b,
                stack,
            ),
            Operator::Mul => Interpreter::apply_numeric(
                |a, b| a.checked_mul(b).ok_or("Numeric overflow".to_owned()),
                |a, b| a.checked_mul(b).ok_or("Numeric overflow".to_owned()),
                |a, b| a * b,
                stack,
            ),
            Operator::Div => Interpreter::apply_numeric(
                |a, b| a.checked_div(b).ok_or("Division by zero".to_owned()),
                |a, b| a.checked_div(b).ok_or("Division by zero".to_owned()),
                |a, b| a / b,
                stack,
            ),

            _ => Err(String::from("Unknown operation")),
        };
    }
}

#[cfg(test)]
mod test;
