use crate::cli::ProgramSource;
use crate::lex::Token;
use crate::parse::Ast;
use crate::pile_error::PileError;

mod runtime_value;
use runtime_value::*;
mod boolean;
mod cast;
mod condition;
mod def;
mod dotimes;
mod numeric;
mod print;
mod runtime_error;
mod while_loop;

use core::mem;
use std::collections::HashMap;
use std::rc::Rc;

pub struct State {
    stack: Vec<RuntimeValue>,
    lookup: HashMap<String, RuntimeValue>,
    current_lines: (u64, u64),
    current_source: ProgramSource,
}

impl State {
    fn error(&self, msg: String) -> PileError {
        PileError::new(self.current_source.clone(), self.current_lines, msg)
    }
}

pub struct Interpreter {
    program: Ast,
    state: State,
}

impl Interpreter {
    pub fn new(program: Ast, initial_size: usize) -> Interpreter {
        let source = program.source.clone();

        Interpreter {
            program,
            state: State {
                stack: Vec::with_capacity(initial_size),
                lookup: HashMap::new(),
                current_lines: (1, 1),
                current_source: source,
            },
        }
    }

    pub fn empty() -> Interpreter {
        Interpreter {
            program: Ast {
                source: ProgramSource::Repl,
                expressions: vec![],
            },
            state: State {
                stack: vec![],
                lookup: HashMap::new(),
                current_lines: (1, 1),
                current_source: ProgramSource::Repl,
            },
        }
    }

    pub fn run(&mut self) -> Result<Option<RuntimeValue>, PileError> {
        Interpreter::call(&self.program.expressions, &mut self.state)?;
        Ok(self.state.stack.pop())
    }

    pub fn eval(
        &mut self,
        mut expressions: Vec<Expr>,
    ) -> Result<Option<&RuntimeValue>, PileError> {
        let old_size = self.program.expressions.len();
        self.program.expressions.append(&mut expressions);

        Interpreter::call(
            &self.program.expressions[old_size..],
            &mut self.state,
        )?;
        Ok(self.state.stack.last())
    }

    fn call(expressions: &[Expr], state: &mut State) -> Result<(), PileError> {
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
                        return Err(state.error(format!("Unexpected {}", token)))
                    }
                },
                Expr::Quoted { token: atom, .. } => match atom {
                    Token::Operator(op) => state.stack.push(
                        RuntimeValue::Function(Function::Builtin(op.clone())),
                    ),
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
                        return Err(state.error(format!("Unexpected {}", token)))
                    }
                },
                Expr::Block { expressions, .. } => {
                    state.stack.push(RuntimeValue::Function(
                        Function::Composite(Rc::clone(expressions)),
                    ))
                }
                Expr::Use { subprogram, .. } => {
                    let mut sub_source = subprogram.source.clone();

                    mem::swap(&mut sub_source, &mut state.current_source);
                    Interpreter::call(&subprogram.expressions, state)?;
                    mem::swap(&mut sub_source, &mut state.current_source);
                }
            }
        }

        Ok(())
    }

    fn apply(op: &Operator, state: &mut State) -> Result<(), PileError> {
        let stack = &mut state.stack;
        let operation_result = match op {
            Operator::Plus => numeric::apply_plus(stack),
            Operator::Minus => numeric::apply_minus(stack),
            Operator::Mul => numeric::apply_mul(stack),
            Operator::Div => numeric::apply_div(stack),
            Operator::If => return condition::apply_if(state),
            Operator::Less => boolean::apply_less(stack),
            Operator::LessEqual => boolean::apply_less_equal(stack),
            Operator::Equal => boolean::apply_equal(stack),
            Operator::Greater => boolean::apply_greater(stack),
            Operator::GreaterEqual => boolean::apply_greater_equal(stack),
            Operator::And => boolean::apply_and(stack),
            Operator::Or => boolean::apply_or(stack),
            Operator::Not => boolean::apply_not(stack),
            Operator::Print => print::apply_print(stack),
            Operator::Dotimes => return dotimes::apply_dotimes(state),
            Operator::Def => def::apply_def(state),
            Operator::While => return while_loop::apply_while(state),
            Operator::Natural => cast::apply_natural(stack),
            Operator::Integer => cast::apply_integer(stack),
            Operator::Float => cast::apply_float(stack),
        };

        operation_result.map_err(|msg| state.error(msg))
    }

    fn resolve(ident: &str, state: &mut State) -> Result<(), PileError> {
        if let Some(value) = state.lookup.get(ident) {
            match value.clone() {
                RuntimeValue::Function(func) => match func {
                    Function::Composite(block) => {
                        Interpreter::call(&block, state)?;
                    }
                    Function::Builtin(op) => {
                        Interpreter::apply(&op, state)?;
                    }
                },
                value => state.stack.push(value),
            }
            Ok(())
        } else {
            Err(state.error(format!("Unknown variable '{}'", ident)))
        }
    }
}

#[cfg(test)]
mod test;
