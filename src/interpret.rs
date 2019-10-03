use crate::lex::Token;
use crate::parse::Ast;

mod runtime_value;
use runtime_value::*;
mod boolean;
mod condition;
mod def;
mod dotimes;
mod numeric;
mod print;
mod runtime_error;
mod while_loop;

use runtime_error::RuntimeError;
use std::collections::HashMap;

pub struct State<'a> {
    stack: Vec<RuntimeValue<'a>>,
    lookup: HashMap<String, RuntimeValue<'a>>,
    current_lines: (u64, u64),
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
                current_lines: (1, 1),
            },
        }
    }

    pub fn reserve(program: Ast, initial_size: usize) -> Interpreter<'a> {
        Interpreter {
            program,
            state: State {
                stack: Vec::with_capacity(initial_size),
                lookup: HashMap::new(),
                current_lines: (1, 1),
            },
        }
    }

    pub fn run(&'a mut self) -> Result<Option<RuntimeValue<'a>>, RuntimeError> {
        Interpreter::call(&self.program.expressions, &mut self.state)
            .map_err(|msg| RuntimeError::new(self.state.current_lines, msg))?;
        Ok(self.state.stack.pop())
    }

    fn call<'e>(
        expressions: &'e [Expr],
        state: &mut State<'e>,
    ) -> Result<(), String> {
        for expr in expressions.iter() {
            state.current_lines = expr.lines();
            match expr {
                Expr::Atom { token: atom, .. } => match atom {
                    Token::Operator(op) => Interpreter::apply(op, state)?,
                    Token::Number(num) => {
                        state.stack.push(RuntimeValue::Number(num.clone()));
                    }
                    Token::Identifier(ident) => {
                        Interpreter::resolve(ident, state)?
                    }
                    Token::String(string) => {
                        state.stack.push(RuntimeValue::String(string.clone()))
                    }
                    Token::Boolean(b) => {
                        state.stack.push(RuntimeValue::Boolean(*b))
                    }
                    token => {
                        return Err(format!("Unexpected {}", token))
                    }
                },
                Expr::Quoted { token: atom, .. } => match atom {
                    Token::Operator(op) => state
                        .stack
                        .push(RuntimeValue::Function(Function::Builtin(op))),
                    Token::Number(num) => {
                        state.stack.push(RuntimeValue::Number(num.clone()));
                    }
                    Token::Identifier(ident) => {
                        state
                            .stack
                            .push(RuntimeValue::Identifier(ident.clone()));
                    }
                    Token::String(string) => {
                        state.stack.push(RuntimeValue::String(string.clone()))
                    }
                    Token::Boolean(b) => {
                        state.stack.push(RuntimeValue::Boolean(*b))
                    }
                    token => {
                        return Err(format!("Unexpected {}", token))
                    }
                },
                Expr::Block { expressions, .. } => state.stack.push(
                    RuntimeValue::Function(Function::Composite(expressions)),
                ),
            }
        }

        Ok(())
    }

    fn apply(op: &Operator, state: &mut State) -> Result<(), String> {
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
            Operator::And => boolean::apply_and(stack),
            Operator::Or => boolean::apply_or(stack),
            Operator::Not => boolean::apply_not(stack),
            Operator::Print => print::apply_print(stack),
            Operator::Dotimes => dotimes::apply_dotimes(state),
            Operator::Def => def::apply_def(state),
            Operator::While => while_loop::apply_while(state),
        }
    }

    fn resolve(ident: &str, state: &mut State) -> Result<(), String> {
        if let Some(value) = state.lookup.get(ident) {
            match value.clone() {
                RuntimeValue::Function(func) => match func {
                    Function::Composite(block) => {
                        Interpreter::call(block, state)?;
                    }
                    Function::Builtin(op) => {
                        Interpreter::apply(op, state)?;
                    }
                },
                value => state.stack.push(value),
            }
            Ok(())
        } else {
            Err(format!("Unknown variable '{}'", ident))
        }
    }
}

#[cfg(test)]
mod test;
