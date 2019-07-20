use crate::lex::Number;
use crate::lex::Operator;
use crate::lex::Token;
use crate::parse::Ast;
use crate::parse::Expr;

#[derive(Debug)]
pub enum RuntimeValue {
    Function(usize),
    Operator(Operator),
    Number(Number),
    Identifier(String),
    String(String),
}

pub struct Interpreter {
    program: Ast,
    stack: Vec<RuntimeValue>,
    position: usize,
}

impl Interpreter {
    pub fn new(program: Ast) -> Interpreter {
        Interpreter {
            program,
            stack: vec![],
            position: 0,
        }
    }

    pub fn run(&mut self) -> Result<&RuntimeValue, String> {
        while self.position < self.program.expressions.len() {
            match &self.program.expressions[self.position] {
                Expr::Atom { token: atom, .. } => match atom {
                    Token::Operator(op) => {
                        Interpreter::apply(op, &mut self.stack)?
                    }
                    Token::Number(num) => {
                        self.stack.push(RuntimeValue::Number(num.clone()));
                    }
                    Token::Identifier(ident) => (),
                    Token::String(string) => {
                        self.stack.push(RuntimeValue::String(string.clone()))
                    }
                    token => {
                        return Err(format!("Unexpected token \'{}\'", token))
                    }
                },
                Expr::Block(_) => {
                    self.stack.push(RuntimeValue::Function(self.position))
                }
            }
            self.position += 1;
            println!("stack: {:?}", self.stack);
        }

        match self.stack.last() {
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
        N: Fn(u32, u32) -> Option<u32>,
        I: Fn(i32, i32) -> Option<i32>,
        F: Fn(f32, f32) -> f32,
    {
        let right = match stack.pop() {
            Some(value) => value,
            None => return Err(String::from("Stack underflow"))
        };
        let left = match stack.pop() {
            Some(value) => value,
            None => return Err(String::from("Stack underflow"))
        };

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
                let value = match op_natural(lhs, rhs) {
                    Some(value) => value,
                    None => return Err(format!("Numeric overflow"))
                };
                Number::Natural(value)
            }
            (Number::Integer(lhs), Number::Integer(rhs)) => {
                let value = match op_integer(lhs, rhs) {
                    Some(value) => value,
                    None => return Err(format!("Numeric overflow"))
                };
                Number::Integer(value)
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
        match op {
            Operator::Plus => {
                return Interpreter::apply_numeric(
                    |a, b| a.checked_add(b),
                    |a, b| a.checked_add(b),
                    |a, b| a + b,
                    stack,
                )
            }
            Operator::Minus => {
                return Interpreter::apply_numeric(
                    |a, b| a.checked_sub(b),
                    |a, b| a.checked_sub(b),
                    |a, b| a - b,
                    stack,
                )
            }
            Operator::Mul => {
                return Interpreter::apply_numeric(
                    |a, b| a.checked_mul(b),
                    |a, b| a.checked_mul(b),
                    |a, b| a * b,
                    stack,
                )
            }
            Operator::Div => {
                return Interpreter::apply_numeric(
                    |a, b| a.checked_div(b),
                    |a, b| a.checked_div(b),
                    |a, b| a / b,
                    stack,
                )
            }

            _ => Err(String::from("Unknown operation")),
        }
    }
}
