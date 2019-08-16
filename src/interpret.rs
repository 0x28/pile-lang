use crate::lex::Token;
use crate::parse::Ast;

mod runtime_value;
use runtime_value::*;
mod boolean;
mod condition;
mod dotimes;
mod numeric;
mod print;
mod runtime_error;

use std::collections::HashMap;

pub struct State<'a> {
    stack: Vec<RuntimeValue<'a>>,
    lookup: HashMap<String, RuntimeValue<'a>>,
}

pub struct Interpreter<'a> {
    program: Ast,
    state: State<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(program: Ast) -> Interpreter<'a> {
        Interpreter {
            program,
            state: State {
                stack: vec![],
                lookup: HashMap::new(),
            },
        }
    }

    pub fn reserve(program: Ast, initial_size: usize) -> Interpreter<'a> {
        Interpreter {
            program,
            state: State {
                stack: Vec::with_capacity(initial_size),
                lookup: HashMap::new(),
            },
        }
    }

    pub fn run(&'a mut self) -> Result<RuntimeValue<'a>, String> {
        Interpreter::call(&mut self.state, &self.program.expressions)?;
        runtime_error::ensure_element(&mut self.state.stack)
    }

    fn call<'s, 'e: 's>(
        state: &'s mut State<'e>,
        expressions: &'e [Expr],
    ) -> Result<(), String> {
        for expr in expressions {
            match expr {
                Expr::Atom { token: atom, .. } => match atom {
                    Token::Operator(op) => Interpreter::apply(op, state)?,
                    Token::Number(num) => {
                        state.stack.push(RuntimeValue::Number(num.clone()));
                    }
                    Token::Identifier(ident) => match state.lookup.get(ident) {
                        Some(value) => state.stack.push(value.clone()),
                        None => {
                            return Err(format!("Unknown variable '{}'", ident))
                        }
                    },
                    Token::String(string) => {
                        state.stack.push(RuntimeValue::String(string.clone()))
                    }
                    Token::Boolean(b) => {
                        state.stack.push(RuntimeValue::Boolean(*b))
                    }
                    token => {
                        return Err(format!("Unexpected token '{}'", token))
                    }
                },
                Expr::Block(expr) => state
                    .stack
                    .push(RuntimeValue::Function(Function::Composite(&expr))),
            }
            println!("stack: {:?}", stack);
        }

        Ok(())
    }

    fn apply<'s, 'e: 's>(
        op: &'s Operator,
        state: &'s mut State<'e>,
    ) -> Result<(), String> {
        let stack = &mut state.stack;
        match op {
            Operator::Plus => numeric::apply_plus(stack),
            Operator::Minus => numeric::apply_minus(stack),
            Operator::Mul => numeric::apply_mul(stack),
            Operator::Div => numeric::apply_div(stack),
            Operator::If => condition::apply_if(state),
            Operator::Less => boolean::apply_less(stack),
            Operator::LessEqual => boolean::apply_less_equal(stack),
            Operator::Equal => boolean::apply_equal(stack),
            Operator::Greater => boolean::apply_greater(stack),
            Operator::GreaterEqual => boolean::apply_greater_equal(stack),
            Operator::Print => print::apply_print(stack),
            Operator::Dotimes => dotimes::apply_dotimes(state),
            _ => Err(String::from("Unknown operation")), // TODO all operations
        }
    }
}

#[cfg(test)]
mod test;
