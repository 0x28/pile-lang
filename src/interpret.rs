use crate::lex::Token;
use crate::parse::Ast;
use crate::pile_error::PileError;
use crate::program_source::ProgramSource;
use crate::using::ResolvedAst;

mod runtime_value;
use runtime_value::*;
mod assert;
mod boolean;
mod cast;
mod condition;
mod dotimes;
mod numeric;
mod print;
mod runtime_error;
mod scoping;
mod stackop;
mod string;
mod tracer;
mod while_loop;
use scoping::ScopeStack;

use std::rc::Rc;

pub struct State {
    stack: Vec<RuntimeValue>,
    lookup: ScopeStack,
    current_lines: (u64, u64),
    trace: bool,
}

pub struct Interpreter {
    program: Ast,
    state: State,
}

impl Interpreter {
    pub fn new(
        program: ResolvedAst,
        initial_size: usize,
        trace: bool,
    ) -> Interpreter {
        Interpreter {
            program: program.as_ast(),
            state: State {
                stack: Vec::with_capacity(initial_size),
                lookup: ScopeStack::new(),
                current_lines: (1, 1),
                trace,
            },
        }
    }

    pub fn empty() -> Interpreter {
        Interpreter {
            program: Ast {
                source: Rc::new(ProgramSource::Repl),
                expressions: vec![],
            },
            state: State {
                stack: vec![],
                lookup: ScopeStack::new(),
                current_lines: (1, 1),
                trace: false,
            },
        }
    }

    pub fn run(&mut self) -> Result<Option<&RuntimeValue>, PileError> {
        Interpreter::call(
            &self.program.expressions,
            &mut self.state,
            &self.program.source,
        )?;
        Ok(self.state.stack.last())
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
            &self.program.source,
        )?;
        Ok(self.state.stack.last())
    }

    fn call(
        expressions: &[Expr],
        state: &mut State,
        source: &Rc<ProgramSource>,
    ) -> Result<(), PileError> {
        for expr in expressions.iter() {
            state.current_lines = expr.lines();

            if state.trace {
                tracer::before_eval(&expr, &state.lookup);
            }

            let result = match expr {
                Expr::Atom { token: atom, .. } => match atom {
                    Token::Operator(op) => {
                        Interpreter::apply(op, state, source)
                    }
                    Token::Number(num) => {
                        Ok(state.stack.push(RuntimeValue::Number(num.clone())))
                    }
                    Token::Identifier(ident) => {
                        Interpreter::resolve(ident, state, source)
                    }
                    Token::String(string) => Ok(state
                        .stack
                        .push(RuntimeValue::String(string.clone()))),
                    Token::Boolean(b) => {
                        Ok(state.stack.push(RuntimeValue::Boolean(*b)))
                    }
                    token => Err(PileError::new(
                        Rc::clone(&source),
                        state.current_lines,
                        format!("Unexpected {}", token),
                    )),
                },
                Expr::Assignment { var, .. } => {
                    Interpreter::assign(var, state, source)
                }
                Expr::Block { expressions, .. } => {
                    Ok(state.stack.push(RuntimeValue::Function(Function {
                        source: Rc::clone(source),
                        exprs: Rc::clone(expressions),
                    })))
                }
                Expr::Use { subprogram, .. } => Interpreter::call(
                    &subprogram.expressions,
                    state,
                    &subprogram.source,
                ),
                Expr::Save { var, .. } => Ok(state.lookup.save(var)),
                Expr::Restore { var, .. } => Ok(state.lookup.restore(var)),
            };

            if state.trace {
                tracer::after_eval(&expr);
            }

            if let Err(e) = result {
                expressions
                    .iter()
                    .rev()
                    .take_while(|expr| matches!(expr, Expr::Restore{..}))
                    .map(|expr| {
                        if let Expr::Restore { var, .. } = expr {
                            if state.trace {
                                tracer::before_eval(expr, &state.lookup);
                            }

                            state.lookup.restore(var);

                            if state.trace {
                                tracer::after_eval(&expr);
                            }
                        }
                    })
                    .count();
                return Err(e);
            }
        }

        Ok(())
    }

    fn apply(
        op: &Operator,
        state: &mut State,
        source: &Rc<ProgramSource>,
    ) -> Result<(), PileError> {
        let stack = &mut state.stack;
        let operation_result = match op {
            Operator::Plus => numeric::apply_plus(stack),
            Operator::Minus => numeric::apply_minus(stack),
            Operator::Mul => numeric::apply_mul(stack),
            Operator::Div => numeric::apply_div(stack),
            Operator::If => return condition::apply_if(state, source),
            Operator::Less => boolean::apply_less(stack),
            Operator::LessEqual => boolean::apply_less_equal(stack),
            Operator::Equal => boolean::apply_equal(stack),
            Operator::Greater => boolean::apply_greater(stack),
            Operator::GreaterEqual => boolean::apply_greater_equal(stack),
            Operator::And => boolean::apply_and(stack),
            Operator::Or => boolean::apply_or(stack),
            Operator::Not => boolean::apply_not(stack),
            Operator::Print => print::apply_print(stack),
            Operator::Assert => assert::apply_assert(stack),
            Operator::Dup => stackop::apply_dup(stack),
            Operator::Drop => stackop::apply_drop(stack),
            Operator::Swap => stackop::apply_swap(stack),
            Operator::Dotimes => return dotimes::apply_dotimes(state, source),
            Operator::While => return while_loop::apply_while(state, source),
            Operator::Natural => cast::apply_natural(stack),
            Operator::Integer => cast::apply_integer(stack),
            Operator::Float => cast::apply_float(stack),
            Operator::Concat => string::apply_concat(stack),
            Operator::Length => string::apply_length(stack),
            Operator::Contains => string::apply_contains(stack),
            Operator::Downcase => string::apply_downcase(stack),
            Operator::Upcase => string::apply_upcase(stack),
            Operator::Trim => string::apply_trim(stack),
        };

        operation_result.map_err(|msg| {
            PileError::new(Rc::clone(&source), state.current_lines, msg)
        })
    }

    fn resolve(
        ident: &str,
        state: &mut State,
        source: &Rc<ProgramSource>,
    ) -> Result<(), PileError> {
        if let Some(value) = state.lookup.resolve(ident) {
            match value {
                RuntimeValue::Function(func) => {
                    let Function { source, exprs } = func;
                    Interpreter::call(&exprs, state, &source)?;
                }
                value => state.stack.push(value),
            }
            Ok(())
        } else {
            Err(PileError::new(
                Rc::clone(&source),
                state.current_lines,
                format!("Unknown variable '{}'", ident),
            ))
        }
    }

    fn assign(
        var: &str,
        state: &mut State,
        source: &Rc<ProgramSource>,
    ) -> Result<(), PileError> {
        let value =
            runtime_error::ensure_element(&mut state.stack).map_err(|msg| {
                PileError::new(Rc::clone(&source), state.current_lines, msg)
            })?;

        state.lookup.assign(var, value);

        Ok(())
    }
}

#[cfg(test)]
mod test;

#[cfg(test)]
mod file_test;
