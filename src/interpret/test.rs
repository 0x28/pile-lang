use super::*;
use crate::lex::Lexer;
use crate::lex::Number;
use crate::parse::Parser;

fn expect_value(prog: &str, value: Result<RuntimeValue, String>) {
    let lexer = Lexer::new(prog);
    let parser = Parser::new(lexer);
    let mut interpreter =
        Interpreter::new(parser.parse().expect("invalid program"));

    assert_eq!(value, interpreter.run());
}

#[test]
fn test_arith1() {
    expect_value("1 2 +", Ok(RuntimeValue::Number(Number::Natural(3))));
}

#[test]
fn test_arith2() {
    expect_value("2 0 *", Ok(RuntimeValue::Number(Number::Natural(0))));
}

#[test]
fn test_arith3() {
    expect_value("10 5 /", Ok(RuntimeValue::Number(Number::Natural(2))));
}

#[test]
fn test_arith4() {
    expect_value("200 100 -", Ok(RuntimeValue::Number(Number::Natural(100))));
}

#[test]
fn test_arith5() {
    expect_value("1 2 3 + +", Ok(RuntimeValue::Number(Number::Natural(6))));
}

#[test]
fn test_arith6() {
    expect_value("1 2 3 * *", Ok(RuntimeValue::Number(Number::Natural(6))));
}

#[test]
fn test_if1() {
    expect_value(
        "begin 1 end begin 2 end true if",
        Ok(RuntimeValue::Number(Number::Natural(1))),
    );
}

#[test]
fn test_if2() {
    expect_value(
        "
10
5
begin - end
begin + end
false
if",
        Ok(RuntimeValue::Number(Number::Natural(15))),
    );
}
#[test]
fn test_if3() {
    expect_value(
        "begin 2 2 * end begin 4 4 * end true if",
        Ok(RuntimeValue::Number(Number::Natural(4))),
    );
}

#[test]
fn test_if4() {
    expect_value(
        "
3 5
begin
  begin
    -123
  end
  begin
    2 + *
  end
  false
  if
end
begin
  0
end
true
if
",
        Ok(RuntimeValue::Number(Number::Natural(21))),
    )
}
