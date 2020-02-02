use super::*;

use crate::program_source::ProgramSource;
use crate::lex::Lexer;
use crate::lex::*;
use crate::parse::Parser;
use crate::using;

use std::fs;
use std::path::PathBuf;

impl Interpreter {
    fn stack(&self) -> &Vec<RuntimeValue> {
        &self.state.stack
    }
}

fn expect_stack(filename: &str, expected: &Vec<RuntimeValue>) {
    let prog = fs::read_to_string(filename)
        .expect(&format!("{}: can't read file", filename));
    let lexer = Lexer::new(
        &prog,
        Rc::new(ProgramSource::File(PathBuf::from(filename))),
    );
    let parser = Parser::new(lexer);

    let ast = using::resolve(parser.parse().expect("invalid program"))
        .expect("invalid 'use'");

    let mut interpreter = Interpreter::new(ast, 10, false);

    match interpreter.run() {
        Err(e) => panic!("interpreter failed: {}", e),
        _ => (),
    }

    assert_eq!(interpreter.stack(), expected);
}

fn expect_error(filename: &str, expected: PileError) {
    let prog = fs::read_to_string(filename)
        .expect(&format!("{}: can't read file", filename));
    let lexer = Lexer::new(
        &prog,
        Rc::new(ProgramSource::File(PathBuf::from(filename))),
    );
    let parser = Parser::new(lexer);
    let ast = using::resolve(parser.parse().expect("invalid program"))
        .expect("invalid 'use'");

    let mut interpreter = Interpreter::new(ast, 10, false);

    assert_eq!(interpreter.run(), Err(expected));
}

fn test_file(filename: &str) -> String {
    env!("CARGO_MANIFEST_DIR").to_owned()
        + "/src/interpret/file_test/"
        + filename
}

#[test]
fn proj_simple() {
    expect_stack(
        &test_file("proj_simple/main.pile"),
        &vec![
            RuntimeValue::Number(Number::Natural(11)),
            RuntimeValue::Number(Number::Natural(19)),
        ],
    )
}

#[test]
fn proj_fib() {
    expect_stack(
        &test_file("proj_fibonacci/main.pile"),
        &vec![
            RuntimeValue::Number(Number::Natural(1)),
            RuntimeValue::Number(Number::Natural(1)),
            RuntimeValue::Number(Number::Natural(5)),
            RuntimeValue::Number(Number::Natural(55)),
        ],
    )
}

#[test]
fn proj_factorial() {
    expect_stack(
        &test_file("proj_factorial/main.pile"),
        &vec![
            RuntimeValue::Number(Number::Natural(1)),
            RuntimeValue::Number(Number::Natural(1)),
            RuntimeValue::Number(Number::Natural(2)),
            RuntimeValue::Number(Number::Natural(6)),
            RuntimeValue::Number(Number::Natural(24)),
            RuntimeValue::Number(Number::Natural(120)),
            RuntimeValue::Number(Number::Natural(720)),
            RuntimeValue::Number(Number::Natural(5040)),
            RuntimeValue::Number(Number::Natural(40320)),
            RuntimeValue::Number(Number::Natural(362880)),
        ],
    )
}

#[test]
fn proj_error_in_function() {
    let main_file = test_file("proj_error1/main.pile");
    let faulty_file = test_file("proj_error1/faulty_function.pile");
    expect_error(
        &main_file,
        PileError::new(
            Rc::new(ProgramSource::File(PathBuf::from(&faulty_file))),
            (2, 2),
            "Type error: string 'hello', natural '100'".to_owned(),
        ),
    )
}

#[test]
fn proj_error_eval() {
    let main_file = test_file("proj_error2/main.pile");
    let bad_file = test_file("proj_error2/bad.pile");
    expect_error(
        &main_file,
        PileError::new(
            Rc::new(ProgramSource::File(PathBuf::from(&bad_file))),
            (1, 1),
            "Division by zero".to_owned(),
        ),
    )
}
