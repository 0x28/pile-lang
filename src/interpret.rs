use crate::lex::Token;
use crate::parse::Ast;

mod runtime_value;
use runtime_value::*;
mod numeric;
use numeric::*;
mod condition;
use condition::apply_if;
mod boolean;
use boolean::*;

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

    pub fn reserve(program: Ast, initial_size: usize) -> Interpreter<'a> {
        Interpreter {
            program,
            stack: Vec::with_capacity(initial_size),
        }
    }

    pub fn run(&'a mut self) -> Result<RuntimeValue<'a>, String> {
        Interpreter::call(&mut self.stack, &self.program.expressions)?;
        Interpreter::ensure_element(&mut self.stack)
    }

    fn call<'s, 'e: 's>(
        stack: &'s mut Vec<RuntimeValue<'e>>,
        expressions: &'e [Expr],
    ) -> Result<(), String> {
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
                    Token::Boolean(b) => stack.push(RuntimeValue::Boolean(*b)),
                    token => {
                        return Err(format!("Unexpected token \'{}\'", token))
                    }
                },
                Expr::Block(expr) => stack
                    .push(RuntimeValue::Function(Function::Composite(&expr))),
            }
            println!("stack: {:?}", stack);
        }

        Ok(())
    }

    fn ensure_element<T>(stack: &mut Vec<T>) -> Result<T, String> {
        stack.pop().ok_or_else(|| "Stack underflow".to_owned())
    }

    fn apply<'s, 'e: 's>(
        op: &'s Operator,
        stack: &'s mut Vec<RuntimeValue<'e>>,
    ) -> Result<(), String> {
        match op {
            Operator::Plus => apply_plus(stack),
            Operator::Minus => apply_minus(stack),
            Operator::Mul => apply_mul(stack),
            Operator::Div => apply_div(stack),
            Operator::If => apply_if(stack),
            Operator::Less => apply_less(stack),
            Operator::LessEqual => apply_less_equal(stack),
            Operator::Equal => apply_equal(stack),
            Operator::Greater => apply_greater(stack),
            Operator::GreaterEqual => apply_greater_equal(stack),
            _ => Err(String::from("Unknown operation")), // TODO all operations
        }
    }
}

#[cfg(test)]
mod test;
