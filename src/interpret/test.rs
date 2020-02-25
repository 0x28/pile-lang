use super::*;
use crate::lex::Lexer;
use crate::lex::Number;
use crate::parse::Parser;
use crate::program_source::ProgramSource;

fn expect_value(prog: &str, value: Result<&RuntimeValue, PileError>) {
    let lexer = Lexer::new(prog, Rc::new(ProgramSource::Stdin));
    let parser = Parser::new(lexer);
    let mut interpreter =
        Interpreter::new(parser.parse().expect("invalid program"), 10, false);

    let result = match interpreter.run() {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (0, 0),
            "No value returned!".to_owned(),
        )),
        Err(e) => Err(e),
    };

    if value != result {
        eprintln!("program: {}", prog);
        assert_eq!(value, result);
    }
}

#[test]
fn test_arith1() {
    expect_value("1 2 +", Ok(&RuntimeValue::Number(Number::Natural(3))));
}

#[test]
fn test_arith2() {
    expect_value("2 0 *", Ok(&RuntimeValue::Number(Number::Natural(0))));
}

#[test]
fn test_arith3() {
    expect_value("10 5 /", Ok(&RuntimeValue::Number(Number::Natural(2))));
}

#[test]
fn test_arith4() {
    expect_value("200 100 -", Ok(&RuntimeValue::Number(Number::Natural(100))));
}

#[test]
fn test_arith5() {
    expect_value("1 2 3 + +", Ok(&RuntimeValue::Number(Number::Natural(6))));
}

#[test]
fn test_arith6() {
    expect_value("1 2 3 * *", Ok(&RuntimeValue::Number(Number::Natural(6))));
}

#[test]
fn test_float_arith() {
    expect_value("1.0 2.0 *", Ok(&RuntimeValue::Number(Number::Float(2.0))));
    expect_value("1.0 2.0 /", Ok(&RuntimeValue::Number(Number::Float(0.5))));
    expect_value("1.0 2.0 +", Ok(&RuntimeValue::Number(Number::Float(3.0))));
    expect_value("1.0 2.0 -", Ok(&RuntimeValue::Number(Number::Float(-1.0))));
}

#[test]
fn test_if1() {
    expect_value(
        "begin 1 end begin 2 end true if",
        Ok(&RuntimeValue::Number(Number::Natural(1))),
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
        Ok(&RuntimeValue::Number(Number::Natural(15))),
    );
}
#[test]
fn test_if3() {
    expect_value(
        "begin 2 2 * end begin 4 4 * end true if",
        Ok(&RuntimeValue::Number(Number::Natural(4))),
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
        Ok(&RuntimeValue::Number(Number::Natural(21))),
    )
}

#[test]
fn test_if5() {
    expect_value(
        "20 20 begin + end begin - end false if",
        Ok(&RuntimeValue::Number(Number::Natural(0))),
    );
}

#[test]
fn test_less1() {
    expect_value("1 2 <", Ok(&RuntimeValue::Boolean(true)));
    expect_value("2 1 <", Ok(&RuntimeValue::Boolean(false)));
    expect_value("1 1 <", Ok(&RuntimeValue::Boolean(false)));
}

#[test]
fn test_less2() {
    expect_value("-1 -2 <", Ok(&RuntimeValue::Boolean(false)));
    expect_value("-2 -1 <", Ok(&RuntimeValue::Boolean(true)));
    expect_value("-1 -1 <", Ok(&RuntimeValue::Boolean(false)));
}

#[test]
fn test_less3() {
    expect_value("3.14 4.0 <", Ok(&RuntimeValue::Boolean(true)));
    expect_value("4.0 3.14 <", Ok(&RuntimeValue::Boolean(false)));
    expect_value("3.14 3.14 <", Ok(&RuntimeValue::Boolean(false)));
}

#[test]
fn test_less4() {
    expect_value("\"hello\" \"world\" <", Ok(&RuntimeValue::Boolean(true)));
    expect_value("\"world\" \"hello\" <", Ok(&RuntimeValue::Boolean(false)));
    expect_value("\"world\" \"world\" <", Ok(&RuntimeValue::Boolean(false)));
}

#[test]
fn test_less_equal() {
    expect_value("1 2 <=", Ok(&RuntimeValue::Boolean(true)));
    expect_value("-2 -1 <=", Ok(&RuntimeValue::Boolean(true)));
    expect_value("4.0 3.14 <=", Ok(&RuntimeValue::Boolean(false)));
    expect_value("\"world\" \"world\" <=", Ok(&RuntimeValue::Boolean(true)));
}

#[test]
fn test_equal() {
    expect_value("42 21 =", Ok(&RuntimeValue::Boolean(false)));
    expect_value("21 21 =", Ok(&RuntimeValue::Boolean(true)));
    // don't do this at home
    expect_value("1.1 1.1 =", Ok(&RuntimeValue::Boolean(true)));
    expect_value("-10 -20 =", Ok(&RuntimeValue::Boolean(false)));
    expect_value("\"abc\" \"abc\" =", Ok(&RuntimeValue::Boolean(true)));
}

#[test]
fn test_greater() {
    expect_value("12 13 >", Ok(&RuntimeValue::Boolean(false)));
    expect_value("-1 -2 >", Ok(&RuntimeValue::Boolean(true)));
    expect_value("12.0 13.0 >", Ok(&RuntimeValue::Boolean(false)));
    expect_value("\"abc\" \"xyz\" >", Ok(&RuntimeValue::Boolean(false)));
}

#[test]
fn test_greater_equal() {
    expect_value("1 1 >=", Ok(&RuntimeValue::Boolean(true)));
    expect_value("2 1 >=", Ok(&RuntimeValue::Boolean(true)));
    expect_value("-1 -1 >=", Ok(&RuntimeValue::Boolean(true)));
    expect_value("-2 -1 >=", Ok(&RuntimeValue::Boolean(false)));
    expect_value("12.0 13.0 >=", Ok(&RuntimeValue::Boolean(false)));
    expect_value("\"abc\" \"xyz\" >=", Ok(&RuntimeValue::Boolean(false)));
}

#[test]
fn test_dotimes() {
    expect_value(
        "0 begin 1 + end 10 dotimes",
        Ok(&RuntimeValue::Number(Number::Natural(10))),
    );
    expect_value(
        "1 begin 2 * end 5 dotimes",
        Ok(&RuntimeValue::Number(Number::Natural(32))),
    );
    expect_value(
        "1 begin 2 * end 0 dotimes",
        Ok(&RuntimeValue::Number(Number::Natural(1))),
    );
    expect_value(
        "100 begin 2 - end 10 dotimes",
        Ok(&RuntimeValue::Number(Number::Natural(80))),
    );
    expect_value(
        "
0
begin
  1
  begin
    5
    *
  end
  3 dotimes
  +
end
10 dotimes",
        Ok(&RuntimeValue::Number(Number::Natural(1250))),
    );

    expect_value(
        "1 1 1 1 1 1 1 1 1 1 begin - end 9 dotimes",
        Ok(&RuntimeValue::Number(Number::Natural(0))),
    );

    // count is an integer instead of a natural
    expect_value(
        "0 begin \"hi\" print end -10 -20 - dotimes",
        Ok(&RuntimeValue::Number(Number::Natural(0))),
    );
}

#[test]
fn test_assign() {
    expect_value(
        "42 -> answer answer answer +",
        Ok(&RuntimeValue::Number(Number::Natural(84))),
    );

    expect_value(
        "
0 -> x
begin
  x 2 +
  -> x
end
10 dotimes
x",
        Ok(&RuntimeValue::Number(Number::Natural(20))),
    );

    expect_value(
        "
begin
  1 +
end
-> inc
100 inc
",
        Ok(&RuntimeValue::Number(Number::Natural(101))),
    );

    expect_value(
        "
3
begin 100 * end -> x
begin x end
begin 200 * end
2 1 > if",
        Ok(&RuntimeValue::Number(Number::Natural(300))),
    );

    expect_value(
        "var var +",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Unknown variable 'var'".to_string(),
        )),
    );
}

#[test]
fn test_alias_assign() {
    expect_value(
        "begin + end -> plus 11 11 plus",
        Ok(&RuntimeValue::Number(Number::Natural(22))),
    )
}

#[test]
fn test_while() {
    expect_value(
        "
1 -> x
begin
  x 2 * -> x
end
begin
  x 10 <
end
while
x",
        Ok(&RuntimeValue::Number(Number::Natural(16))),
    );

    expect_value(
        "
1 2 3 862 73 954 62 38 363 939 9484 3
begin
end
begin
  62 = not
end
while
",
        Ok(&RuntimeValue::Number(Number::Natural(954))),
    );

    expect_value(
        "10 \"x\" \"y\" 1 1 1 1 1 1
         begin print end
         begin = end
         while",
        Ok(&RuntimeValue::Number(Number::Natural(10))),
    )
}

#[test]
fn test_and() {
    expect_value("true true and", Ok(&RuntimeValue::Boolean(true)));
    expect_value("true false and", Ok(&RuntimeValue::Boolean(false)));
    expect_value("false true and", Ok(&RuntimeValue::Boolean(false)));
    expect_value("false false and", Ok(&RuntimeValue::Boolean(false)));
}

#[test]
fn test_or() {
    expect_value("true true or", Ok(&RuntimeValue::Boolean(true)));
    expect_value("true false or", Ok(&RuntimeValue::Boolean(true)));
    expect_value("false true or", Ok(&RuntimeValue::Boolean(true)));
    expect_value("false false or", Ok(&RuntimeValue::Boolean(false)));
}

#[test]
fn test_not() {
    expect_value("true not", Ok(&RuntimeValue::Boolean(false)));
    expect_value("false not", Ok(&RuntimeValue::Boolean(true)));
}

#[test]
fn test_print() {
    expect_value("true \"hello\" print", Ok(&RuntimeValue::Boolean(true)));
    expect_value("true 100 print", Ok(&RuntimeValue::Boolean(true)));
    expect_value("true -100 print", Ok(&RuntimeValue::Boolean(true)));
    expect_value("true 32.32 print", Ok(&RuntimeValue::Boolean(true)));
    expect_value("true true print", Ok(&RuntimeValue::Boolean(true)));
    expect_value("true false print", Ok(&RuntimeValue::Boolean(true)));
    expect_value("true begin x end print", Ok(&RuntimeValue::Boolean(true)));
}

#[test]
fn test_cast_to_natural() {
    expect_value("1 natural", Ok(&RuntimeValue::Number(Number::Natural(1))));
    expect_value(
        "-1 -2 - natural",
        Ok(&RuntimeValue::Number(Number::Natural(1))),
    );
    expect_value("1.2 natural", Ok(&RuntimeValue::Number(Number::Natural(1))));
    expect_value(
        "-1 natural",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Conversion from integer '-1' to natural is invalid".to_string(),
        )),
    );
}

#[test]
fn test_cast_to_integer() {
    expect_value("-1 integer", Ok(&RuntimeValue::Number(Number::Integer(-1))));
    expect_value(
        "100 integer",
        Ok(&RuntimeValue::Number(Number::Integer(100))),
    );
    expect_value(
        "-32.12 integer",
        Ok(&RuntimeValue::Number(Number::Integer(-32))),
    );
    expect_value(
        "4290000000 integer",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Conversion from natural '4290000000' to integer is invalid"
                .to_string(),
        )),
    )
}

#[test]
fn test_cast_to_float() {
    expect_value("100 float", Ok(&RuntimeValue::Number(Number::Float(100.0))));
    expect_value(
        "-100 float",
        Ok(&RuntimeValue::Number(Number::Float(-100.0))),
    );
    expect_value(
        "42.42 float",
        Ok(&RuntimeValue::Number(Number::Float(42.42))),
    );
}

#[test]
fn test_numeric_overflow() {
    expect_value(
        "4294967295 1 +",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Numeric overflow".to_string(),
        )),
    );
    expect_value(
        "0 1 -",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Numeric overflow".to_string(),
        )),
    );
    expect_value(
        "1000000000 100000 *",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Numeric overflow".to_string(),
        )),
    );
    expect_value(
        "-2000000005 -2000000005 +",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Numeric overflow".to_string(),
        )),
    );
    expect_value(
        "-2000000005 -2000000005 *",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Numeric overflow".to_string(),
        )),
    );
}

#[test]
fn test_div_by_zero() {
    expect_value(
        "0 0 /",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Division by zero".to_string(),
        )),
    );
    expect_value(
        "-0 -0 /",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Division by zero".to_string(),
        )),
    );
}

#[test]
fn test_type_errors() {
    expect_value(
        "42 \"hi\" *",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Type error: natural \'42\', string \'hi\'".to_string(),
        )),
    );

    expect_value(
        "12.34 4 +",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Numeric type mismatch: float \'12.34\', natural \'4\'".to_string(),
        )),
    );

    expect_value(
        "begin end begin end \"...\" if",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Expected boolean found string \'...\'".to_string(),
        )),
    );

    expect_value(
        "0 begin \"hi\" print end \"...\" dotimes",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Expected positive number found string \'...\'".to_string(),
        )),
    );

    expect_value(
        "10 -> x
         x 10 dotimes",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (2, 2),
            "Expected function found natural \'10\'".to_string(),
        )),
    );

    expect_value(
        "10 10 dotimes",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Expected function found natural \'10\'".to_string(),
        )),
    );

    expect_value(
        "begin unknown_func end 10 dotimes",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Unknown variable 'unknown_func'".to_string(),
        )),
    );

    expect_value(
        "begin end begin end \"true\" if",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Expected boolean found string 'true'".to_string(),
        )),
    );
}

#[test]
fn test_runtime_error_fmt() {
    assert_eq!(
        format!(
            "{}",
            PileError::new(
                Rc::new(ProgramSource::Stdin),
                (1000, 1000),
                "Serious error!!!".to_string(),
            )
        ),
        "1000: Serious error!!!"
    );

    assert_eq!(
        format!(
            "{}",
            PileError::new(
                Rc::new(ProgramSource::Stdin),
                (10, 20),
                "This is really bad...".to_string(),
            )
        ),
        "10-20: This is really bad..."
    );
}

#[test]
fn test_eval() -> Result<(), PileError> {
    fn read(input: &str) -> Vec<Expr> {
        Parser::new(Lexer::new(input, Rc::new(ProgramSource::Stdin)))
            .parse()
            .unwrap()
            .expressions
    }

    let mut interpreter = Interpreter::empty();
    assert_eq!(
        interpreter.eval(read("1 2 +"))?,
        Some(&RuntimeValue::Number(Number::Natural(3)))
    );
    assert_eq!(
        interpreter.eval(read("2 *"))?,
        Some(&RuntimeValue::Number(Number::Natural(6)))
    );
    assert_eq!(
        interpreter.eval(read("6 /"))?,
        Some(&RuntimeValue::Number(Number::Natural(1)))
    );
    assert_eq!(interpreter.eval(read("print"))?, None);
    assert_eq!(interpreter.eval(read("begin 1 + end -> inc"))?, None);
    assert_eq!(
        interpreter.eval(read("20 inc"))?,
        Some(&RuntimeValue::Number(Number::Natural(21)))
    );

    Ok(())
}
