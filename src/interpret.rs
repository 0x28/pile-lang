use crate::lex::Token;
use crate::parse::Ast;

mod runtime_value;
use runtime_value::*;
mod runtime_error;
mod numeric;
mod condition;
mod boolean;
mod print;

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
        runtime_error::ensure_element(&mut self.stack)
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

    fn apply<'s, 'e: 's>(
        op: &'s Operator,
        stack: &'s mut Vec<RuntimeValue<'e>>,
    ) -> Result<(), String> {
        match op {
            Operator::Plus => numeric::apply_plus(stack),
            Operator::Minus => numeric::apply_minus(stack),
            Operator::Mul => numeric::apply_mul(stack),
            Operator::Div => numeric::apply_div(stack),
            Operator::If => condition::apply_if(stack),
            Operator::Less => boolean::apply_less(stack),
            Operator::LessEqual => boolean::apply_less_equal(stack),
            Operator::Equal => boolean::apply_equal(stack),
            Operator::Greater => boolean::apply_greater(stack),
            Operator::GreaterEqual => boolean::apply_greater_equal(stack),
            Operator::Print => print::apply_print(stack),
            _ => Err(String::from("Unknown operation")), // TODO all operations
        }
    }
}

#[cfg(test)]
mod test;
