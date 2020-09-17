use super::*;
use crate::lex::Lexer;
use crate::lex::Number;
use crate::locals;
use crate::parse::Parser;
use crate::program_source::ProgramSource;
use crate::using;

fn expect_value(prog: &str, value: Result<&RuntimeValue, PileError>) {
    let lexer = Lexer::new(prog, Rc::new(ProgramSource::Stdin));
    let parser = Parser::new(lexer);
    let ast = parser.parse().expect("invalid program");
    let ast = locals::translate(ast);
    let ast = using::resolve(ast).expect("resolve failed");
    let mut interpreter = Interpreter::new(ast, 10, false);

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
fn test_invalid_compare() {
    expect_value(
        "1 \"str\" <",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Can't compare natural '1' and string 'str'".to_string(),
        )),
    );
    expect_value(
        "1.1 1 =",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Can't compare float '1.1' and natural '1'".to_string(),
        )),
    );
    expect_value(
        "-1 1 >=",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Can't compare integer '-1' and natural '1'".to_string(),
        )),
    );
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
fn test_showstack() {
    expect_value(
        "true \"hello\" showstack",
        Ok(&RuntimeValue::String("hello".to_owned())),
    );
    expect_value(
        "1 2 showstack stacksize",
        Ok(&RuntimeValue::Number(Number::Natural(2))),
    );
}

#[test]
fn test_assert() {
    expect_value(
        "0 1 > assert",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Assertion failed".to_string(),
        )),
    );
    expect_value("true 1 1 = assert", Ok(&RuntimeValue::Boolean(true)))
}

#[test]
fn test_dup() {
    expect_value("10 dup +", Ok(&RuntimeValue::Number(Number::Natural(20))));
    expect_value("0 dup drop", Ok(&RuntimeValue::Number(Number::Natural(0))));
    expect_value(
        "\"hello dup\" dup drop",
        Ok(&RuntimeValue::String("hello dup".to_owned())),
    );
}

#[test]
fn test_swap() {
    expect_value("true false swap", Ok(&RuntimeValue::Boolean(true)));
    expect_value("false true swap", Ok(&RuntimeValue::Boolean(false)));
    expect_value(
        "10 10 swap +",
        Ok(&RuntimeValue::Number(Number::Natural(20))),
    );
    expect_value(
        "10 20 swap -",
        Ok(&RuntimeValue::Number(Number::Natural(10))),
    );
}

#[test]
fn test_drop() {
    expect_value(
        "0 drop drop",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Stack underflow".to_string(),
        )),
    );
    expect_value(
        "1 2 3 drop +",
        Ok(&RuntimeValue::Number(Number::Natural(3))),
    );
}

#[test]
fn test_pick() {
    expect_value(
        r#" "pick me" 0 pick"#,
        Ok(&RuntimeValue::String("pick me".to_owned())),
    );

    expect_value(
        "100 200 300 2 pick",
        Ok(&RuntimeValue::Number(Number::Natural(100))),
    );
    expect_value(
        "100 200 300 1 pick",
        Ok(&RuntimeValue::Number(Number::Natural(200))),
    );
    expect_value(
        "100 200 300 0 pick",
        Ok(&RuntimeValue::Number(Number::Natural(300))),
    );
    expect_value(
        r#" 100 200 300 "index!" pick"#,
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Can't use string 'index!' as stack index".to_string(),
        )),
    );
    expect_value(
        "100 200 300 3 pick",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Invalid index 3 for pick into stack of size 3".to_string(),
        )),
    );

    expect_value(
        "1000000 pick",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Invalid index 1000000 for pick into stack of size 0".to_string(),
        )),
    );
}

#[test]
fn test_stacksize() {
    expect_value(
        "1 2 3 stacksize",
        Ok(&RuntimeValue::Number(Number::Natural(3))),
    );
    expect_value("stacksize", Ok(&RuntimeValue::Number(Number::Natural(0))));
    expect_value(
        r#" 3.42 "hello" -423 0 stacksize"#,
        Ok(&RuntimeValue::Number(Number::Natural(4))),
    );
}

#[test]
fn test_clear() {
    expect_value(
        "100 200 300 clear stacksize",
        Ok(&RuntimeValue::Number(Number::Natural(0))),
    );
    expect_value(
        "clear clear stacksize",
        Ok(&RuntimeValue::Number(Number::Natural(0))),
    );
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
        "9300000000000000000 integer",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Conversion from natural '9300000000000000000' to integer is invalid"
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
fn test_concat() {
    expect_value(
        "\"hello\" \"_world\" concat",
        Ok(&RuntimeValue::String("hello_world".to_owned())),
    );
    expect_value(
        "\"a\" \"b\" \"c\" concat concat",
        Ok(&RuntimeValue::String("abc".to_owned())),
    );
    expect_value("\"\" \"\" concat", Ok(&RuntimeValue::String("".to_owned())));
}

#[test]
fn test_length() {
    expect_value(
        "\"some_string\" length",
        Ok(&RuntimeValue::Number(Number::Natural(11))),
    );
    expect_value("\"\" length", Ok(&RuntimeValue::Number(Number::Natural(0))));
    expect_value(
        "\"not_consumed\" length drop",
        Ok(&RuntimeValue::String("not_consumed".to_owned())),
    );
    expect_value(
        "\"ünicöde\" length",
        Ok(&RuntimeValue::Number(Number::Natural(7))),
    );
}

#[test]
fn test_contains() {
    expect_value(
        "\"string\" \"s\" contains",
        Ok(&RuntimeValue::Boolean(true)),
    );

    expect_value("\"\" \"str\" contains", Ok(&RuntimeValue::Boolean(false)));

    expect_value("\"str\" \"\" contains", Ok(&RuntimeValue::Boolean(true)));

    expect_value("\"pile\" \"pi\" contains", Ok(&RuntimeValue::Boolean(true)));
}

#[test]
fn test_upcase() {
    expect_value(
        "\"pile\" upcase",
        Ok(&RuntimeValue::String("PILE".to_owned())),
    );
    expect_value(
        "\"ünicöde ß\" upcase",
        Ok(&RuntimeValue::String("ÜNICÖDE SS".to_owned())),
    );
    expect_value(
        "\"mIxEd_CaSe\" upcase",
        Ok(&RuntimeValue::String("MIXED_CASE".to_owned())),
    );
}

#[test]
fn test_downcase() {
    expect_value(
        "\"PILE\" downcase",
        Ok(&RuntimeValue::String("pile".to_owned())),
    );
    expect_value(
        "\"ÜNICÖDE SS\" downcase",
        Ok(&RuntimeValue::String("ünicöde ss".to_owned())),
    );
    expect_value(
        "\"mIxEd_CaSe\" downcase",
        Ok(&RuntimeValue::String("mixed_case".to_owned())),
    );
}

#[test]
fn test_trim() {
    expect_value("\" \" trim", Ok(&RuntimeValue::String("".to_owned())));
    expect_value(
        "\" xyz \" trim",
        Ok(&RuntimeValue::String("xyz".to_owned())),
    );
    expect_value("\"a \" trim", Ok(&RuntimeValue::String("a".to_owned())));
    expect_value("\" b\" trim", Ok(&RuntimeValue::String("b".to_owned())));
    expect_value("\"\tx\t\" trim", Ok(&RuntimeValue::String("x".to_owned())));
    expect_value(
        "\"  \ty\t  \n\t\" trim",
        Ok(&RuntimeValue::String("y".to_owned())),
    );
}

#[test]
fn test_format() {
    expect_value(
        r#"100 "{}" format"#,
        Ok(&RuntimeValue::String("100".to_owned())),
    );
    expect_value(
        r#"1 2 3 "{} {}_{}" format"#,
        Ok(&RuntimeValue::String("1 2_3".to_owned())),
    );
    expect_value(
        r#""no format" format"#,
        Ok(&RuntimeValue::String("no format".to_owned())),
    );

    expect_value(r#""" format"#, Ok(&RuntimeValue::String("".to_owned())));
    expect_value(
        r#"1.23 "x" "{}-{}" format"#,
        Ok(&RuntimeValue::String("1.23-x".to_owned())),
    );
    expect_value(
        r#" "{}" "{}" format"#,
        Ok(&RuntimeValue::String("{}".to_owned())),
    );
    expect_value(
        r#" 2020 12 24 "{}/{}/{}" format"#,
        Ok(&RuntimeValue::String("2020/12/24".to_owned())),
    );
    expect_value(
        r#" -200 -10 - "~~[{}]~~" format"#,
        Ok(&RuntimeValue::String("~~[-190]~~".to_owned())),
    );
    expect_value(
        r#" 42 "{} -> {}" format"#,
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Stack underflow".to_owned(),
        )),
    );
}

#[test]
fn test_index() {
    expect_value(
        r#" "hello" 0 index"#,
        Ok(&RuntimeValue::String("h".to_owned())),
    );
    expect_value(
        r#" "hello" 1 index"#,
        Ok(&RuntimeValue::String("e".to_owned())),
    );
    expect_value(
        r#" "ünicöde" 4 index"#,
        Ok(&RuntimeValue::String("ö".to_owned())),
    );
    expect_value(
        r#" "test" 0 index drop"#,
        Ok(&RuntimeValue::String("test".to_owned())),
    );
    expect_value(
        r#" "123ö" length 1 - index"#,
        Ok(&RuntimeValue::String("ö".to_owned())),
    );
    expect_value(
        r#" "shorty" 10 index"#,
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            r#"Invalid index 10 for string "shorty""#.to_owned(),
        )),
    );
    expect_value(
        r#" "str" 42.1 index"#,
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Can't use float '42.1' as string index".to_owned(),
        )),
    );
}

#[test]
fn test_let1() {
    expect_value(
        "
100 -> a
let [a]
    200 -> a # change local variable
end -> let1

let1
a",
        Ok(&RuntimeValue::Number(Number::Natural(100))),
    )
}

#[test]
fn test_let2() {
    expect_value(
        "
21 -> a
let [b]
    42 -> a # change global variable
end -> let1

let1
a",
        Ok(&RuntimeValue::Number(Number::Natural(42))),
    )
}

#[test]
fn test_let3() {
    expect_value(
        "
let [n]
    dup -> n
    begin n end
    begin n 1 + recur end
    n 10 =
    if
    n # return 0 anyway
end -> recur

0 recur
",
        Ok(&RuntimeValue::Number(Number::Natural(0))),
    )
}

#[test]
fn test_dyn_scope() {
    expect_value(
        "
begin
    n n *
end -> f1

let [n]
    23 -> n
    f1
end -> f2

f2
",
        Ok(&RuntimeValue::Number(Number::Natural(529))),
    )
}

#[test]
fn test_numeric_overflow() {
    expect_value(
        "18446744073709551615 1 +",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Numeric overflow while adding '18446744073709551615' and '1'"
                .to_string(),
        )),
    );
    expect_value(
        "0 1 -",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Numeric overflow while subtracting '0' and '1'".to_string(),
        )),
    );
    expect_value(
        "100000000000 1000000000 *",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Numeric overflow while multiplying '100000000000' and '1000000000'"
                .to_string(),
        )),
    );
    expect_value(
        "-9000000000000000005 -9000000000000000005 +",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Numeric overflow while adding '-9000000000000000005' \
             and '-9000000000000000005'"
                .to_string(),
        )),
    );
    expect_value(
        "-200000000000005 -200000000000005 *",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Numeric overflow while multiplying '-200000000000005' \
             and '-200000000000005'"
                .to_string(),
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
            "Division by zero while dividing '0' and '0'".to_string(),
        )),
    );
    expect_value(
        "-0 -0 /",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Division by zero while dividing '0' and '0'".to_string(),
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

    expect_value(
        "10 \"test\" concat",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Expected string found natural '10'".to_string(),
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

fn read(input: &str) -> Vec<Expr> {
    let ast = Parser::new(Lexer::new(input, Rc::new(ProgramSource::Stdin)))
        .parse()
        .unwrap();
    let ast = locals::translate(ast);
    using::resolve(ast).unwrap().as_ast().expressions
}

#[test]
fn test_eval() -> Result<(), PileError> {
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

#[test]
fn test_eval_cleanup_vars() -> Result<(), PileError> {
    let mut interpreter = Interpreter::empty();
    interpreter.eval(read("let [x] 10 -> x 1 0 / end -> fun"))?;
    assert_eq!(
        interpreter.eval(read("fun")),
        Err(PileError::new(
            Rc::new(ProgramSource::Repl),
            (1, 1),
            "Division by zero while dividing '1' and '0'".to_string(),
        )),
    );
    assert_eq!(
        interpreter.eval(read("x")),
        Err(PileError::new(
            Rc::new(ProgramSource::Repl),
            (1, 1),
            "Unknown variable 'x'".to_string(),
        )),
    );

    Ok(())
}
