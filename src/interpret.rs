use crate::lex::Token;
use crate::parse::Ast;

mod runtime_value;
use runtime_value::*;

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
        let right = Interpreter::ensure_element(stack)?;
        let left = Interpreter::ensure_element(stack)?;

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

    fn ensure_element<T>(stack: &mut Vec<T>) -> Result<T, String> {
        stack.pop().ok_or_else(|| "Stack underflow".to_owned())
    }

    fn apply_if<'s, 'e: 's>(
        stack: &'s mut Vec<RuntimeValue<'e>>,
    ) -> Result<(), String> {
        let condition = Interpreter::ensure_element(stack)?;
        let else_branch = Interpreter::ensure_element(stack)?;
        let if_branch = Interpreter::ensure_element(stack)?;

        let if_branch = match if_branch {
            RuntimeValue::Function(body) => body,
            v => return Err(format!("Expected function found {:?}", v)),
        };

        let else_branch = match else_branch {
            RuntimeValue::Function(body) => body,
            v => return Err(format!("Expected function found {:?}", v)),
        };

        let condition = match condition {
            RuntimeValue::Boolean(b) => b,
            v => return Err(format!("Expected boolean found {:?}", v)),
        };

        if condition {
            match if_branch {
                Function::Composite(block) => Interpreter::call(stack, block)?,
                Function::Builtin(operator) => {
                    Interpreter::apply(&operator, stack)?
                }
            }
        } else {
            match else_branch {
                Function::Composite(block) => Interpreter::call(stack, block)?,
                Function::Builtin(operator) => {
                    Interpreter::apply(&operator, stack)?
                }
            }
        }

        Ok(())
    }

    fn apply_bool<N, I, F, S>(
        nat_nat_cmp: N,
        int_int_cmp: I,
        float_float_cmp: F,
        str_str_cmp: S,
        stack: &mut Vec<RuntimeValue>,
    ) -> Result<(), String>
    where
        N: Fn(u32, u32) -> bool,
        I: Fn(i32, i32) -> bool,
        F: Fn(f32, f32) -> bool,
        S: Fn(&str, &str) -> bool,
    {
        let right = Interpreter::ensure_element(stack)?;
        let left = Interpreter::ensure_element(stack)?;

        let compare_result = match (left, right) {
            (
                RuntimeValue::Number(Number::Natural(num_left)),
                RuntimeValue::Number(Number::Natural(num_right)),
            ) => nat_nat_cmp(num_left, num_right),
            (
                RuntimeValue::Number(Number::Integer(num_left)),
                RuntimeValue::Number(Number::Integer(num_right)),
            ) => int_int_cmp(num_left, num_right),
            (
                RuntimeValue::Number(Number::Float(num_left)),
                RuntimeValue::Number(Number::Float(num_right)),
            ) => float_float_cmp(num_left, num_right),
            (
                RuntimeValue::String(str_left),
                RuntimeValue::String(str_right),
            ) => str_str_cmp(str_left.as_ref(), str_right.as_ref()),
            (left, right) => {
                return Err(format!("Can't compare {:?} and {:?}", left, right))
            }
        };

        stack.push(RuntimeValue::Boolean(compare_result));

        Ok(())
    }

    fn apply_less(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
        Interpreter::apply_bool(
            |n1, n2| n1 < n2,
            |i1, i2| i1 < i2,
            |f1, f2| f1 < f2,
            |s1, s2| s1 < s2,
            stack,
        )
    }

    fn apply_less_equal(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
        Interpreter::apply_bool(
            |n1, n2| n1 <= n2,
            |i1, i2| i1 <= i2,
            |f1, f2| f1 <= f2,
            |s1, s2| s1 <= s2,
            stack,
        )
    }

    fn apply_equal(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
        Interpreter::apply_bool(
            |n1, n2| n1 == n2,
            |i1, i2| i1 == i2,
            |f1, f2| f1 == f2,
            |s1, s2| s1 == s2,
            stack,
        )
    }

    fn apply_greater(stack: &mut Vec<RuntimeValue>) -> Result<(), String> {
        Interpreter::apply_bool(
            |n1, n2| n1 > n2,
            |i1, i2| i1 > i2,
            |f1, f2| f1 > f2,
            |s1, s2| s1 > s2,
            stack,
        )
    }

    fn apply_greater_equal(
        stack: &mut Vec<RuntimeValue>,
    ) -> Result<(), String> {
        Interpreter::apply_bool(
            |n1, n2| n1 >= n2,
            |i1, i2| i1 >= i2,
            |f1, f2| f1 >= f2,
            |s1, s2| s1 >= s2,
            stack,
        )
    }

    fn apply<'s, 'e: 's>(
        op: &'s Operator,
        stack: &'s mut Vec<RuntimeValue<'e>>,
    ) -> Result<(), String> {
        match op {
            Operator::Plus => Interpreter::apply_numeric(
                |a, b| {
                    a.checked_add(b)
                        .ok_or_else(|| "Numeric overflow".to_owned())
                },
                |a, b| {
                    a.checked_add(b)
                        .ok_or_else(|| "Numeric overflow".to_owned())
                },
                |a, b| a + b,
                stack,
            ),
            Operator::Minus => Interpreter::apply_numeric(
                |a, b| {
                    a.checked_sub(b)
                        .ok_or_else(|| "Numeric overflow".to_owned())
                },
                |a, b| {
                    a.checked_sub(b)
                        .ok_or_else(|| "Numeric overflow".to_owned())
                },
                |a, b| a - b,
                stack,
            ),
            Operator::Mul => Interpreter::apply_numeric(
                |a, b| {
                    a.checked_mul(b)
                        .ok_or_else(|| "Numeric overflow".to_owned())
                },
                |a, b| {
                    a.checked_mul(b)
                        .ok_or_else(|| "Numeric overflow".to_owned())
                },
                |a, b| a * b,
                stack,
            ),
            Operator::Div => Interpreter::apply_numeric(
                |a, b| {
                    a.checked_div(b)
                        .ok_or_else(|| "Division by zero".to_owned())
                },
                |a, b| {
                    a.checked_div(b)
                        .ok_or_else(|| "Division by zero".to_owned())
                },
                |a, b| a / b,
                stack,
            ),
            Operator::If => Interpreter::apply_if(stack),
            Operator::Less => Interpreter::apply_less(stack),
            Operator::LessEqual => Interpreter::apply_less_equal(stack),
            Operator::Equal => Interpreter::apply_equal(stack),
            Operator::Greater => Interpreter::apply_greater(stack),
            Operator::GreaterEqual => Interpreter::apply_greater_equal(stack),
            _ => Err(String::from("Unknown operation")), // TODO all operations
        }
    }
}

#[cfg(test)]
mod test;
