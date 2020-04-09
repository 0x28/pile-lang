use super::*;
use crate::program_source::ProgramSource;

fn compare_token_lists(
    lexer: Lexer,
    expected: Vec<(u64, Result<Token, PileError>)>,
) {
    let result: Vec<_> = lexer.into_iter().collect();

    assert_eq!(result.len(), expected.len());

    for (actual, expected) in result.iter().zip(expected.iter()) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_empty_program() {
    let lexer = Lexer::new("", Rc::new(ProgramSource::Stdin));

    assert_eq!(lexer.into_iter().count(), 0);
}

#[test]
fn test_comment_simple() {
    let lexer =
        Lexer::new("2 3 *# hello world\n1 1 +", Rc::new(ProgramSource::Stdin));
    let expected = vec![
        (1, Ok(Token::Number(Number::Natural(2)))),
        (1, Ok(Token::Number(Number::Natural(3)))),
        (1, Ok(Token::Operator(Operator::Mul))),
        (2, Ok(Token::Number(Number::Natural(1)))),
        (2, Ok(Token::Number(Number::Natural(1)))),
        (2, Ok(Token::Operator(Operator::Plus))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_comment_only() {
    let lexer = Lexer::new("# empty program", Rc::new(ProgramSource::Stdin));

    assert_eq!(lexer.into_iter().count(), 0);
}

#[test]
fn test_string_simple() {
    let lexer = Lexer::new(
        "\"yay programming languages :)\"# comment",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![(
        1,
        Ok(Token::String(String::from("yay programming languages :)"))),
    )];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_string_escaped() {
    let lexer = Lexer::new(
        "\"\\n\\n\\n\\t\\r\0#test#\"",
        Rc::new(ProgramSource::Stdin),
    );
    let expected =
        vec![(1, Ok(Token::String(String::from("\n\n\n\t\r\0#test#"))))];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_unknown_char() {
    let lexer = Lexer::new("\\hello world\\", Rc::new(ProgramSource::Stdin));
    let expected = vec![
        (
            1,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (1, 1),
                "Unknown char '\\'".to_owned(),
            )),
        ),
        (1, Ok(Token::Identifier(String::from("hello")))),
        (1, Ok(Token::Identifier(String::from("world")))),
        (
            1,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (1, 1),
                "Unknown char '\\'".to_owned(),
            )),
        ),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_numbers_natural() {
    let lexer = Lexer::new(
        "100 2000 3000 123 4543 123 21393#123#123
         203 040 05060 70 80 002 1203004 003",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::Number(Number::Natural(100)))),
        (1, Ok(Token::Number(Number::Natural(2000)))),
        (1, Ok(Token::Number(Number::Natural(3000)))),
        (1, Ok(Token::Number(Number::Natural(123)))),
        (1, Ok(Token::Number(Number::Natural(4543)))),
        (1, Ok(Token::Number(Number::Natural(123)))),
        (1, Ok(Token::Number(Number::Natural(21393)))),
        (2, Ok(Token::Number(Number::Natural(203)))),
        (2, Ok(Token::Number(Number::Natural(40)))),
        (2, Ok(Token::Number(Number::Natural(5060)))),
        (2, Ok(Token::Number(Number::Natural(70)))),
        (2, Ok(Token::Number(Number::Natural(80)))),
        (2, Ok(Token::Number(Number::Natural(2)))),
        (2, Ok(Token::Number(Number::Natural(1203004)))),
        (2, Ok(Token::Number(Number::Natural(3)))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_numbers_integer() {
    let lexer = Lexer::new(
        "-1\n-2\n-3\n-4000\n-0044
        -1000 -200 -42 -42",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::Number(Number::Integer(-1)))),
        (2, Ok(Token::Number(Number::Integer(-2)))),
        (3, Ok(Token::Number(Number::Integer(-3)))),
        (4, Ok(Token::Number(Number::Integer(-4000)))),
        (5, Ok(Token::Number(Number::Integer(-44)))),
        (6, Ok(Token::Number(Number::Integer(-1000)))),
        (6, Ok(Token::Number(Number::Integer(-200)))),
        (6, Ok(Token::Number(Number::Integer(-42)))),
        (6, Ok(Token::Number(Number::Integer(-42)))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_numbers_float() {
    let lexer = Lexer::new(
        "1.1\n2.2\n3.3\n-10e20\n 20E3
         3.1415 7777.7777 -3e-10#number :)",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::Number(Number::Float(1.1)))),
        (2, Ok(Token::Number(Number::Float(2.2)))),
        (3, Ok(Token::Number(Number::Float(3.3)))),
        (4, Ok(Token::Number(Number::Float(-10e20)))),
        (5, Ok(Token::Number(Number::Float(20e3)))),
        (6, Ok(Token::Number(Number::Float(3.1415)))),
        (6, Ok(Token::Number(Number::Float(7777.7777)))),
        (6, Ok(Token::Number(Number::Float(-3e-10)))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_numbers_overflow() {
    let lexer = Lexer::new(
        "8589934592 -8589934592 +8589934592",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (
            1,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (1, 1),
                "'8589934592' is too large to be represented as a number"
                    .to_string(),
            )),
        ),
        (
            1,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (1, 1),
                "'-8589934592' is too small to be represented as a number"
                    .to_string(),
            )),
        ),
        (
            1,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (1, 1),
                "'+8589934592' is too large to be represented as a number"
                    .to_string(),
            )),
        ),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_boolean() {
    let lexer = Lexer::new(
        "true false true
         false true false",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::Boolean(true))),
        (1, Ok(Token::Boolean(false))),
        (1, Ok(Token::Boolean(true))),
        (2, Ok(Token::Boolean(false))),
        (2, Ok(Token::Boolean(true))),
        (2, Ok(Token::Boolean(false))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_keywords() {
    let lexer = Lexer::new(
        "
        begin#what
          10 +
          100 *# this is a operator
        end

        while dotimes DOTIMES END BEGIN -> if IF print
        and AND or OR not NOT
        natural NATURAL integer INTEGER float FLOAT",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (2, Ok(Token::Begin)),
        (3, Ok(Token::Number(Number::Natural(10)))),
        (3, Ok(Token::Operator(Operator::Plus))),
        (4, Ok(Token::Number(Number::Natural(100)))),
        (4, Ok(Token::Operator(Operator::Mul))),
        (5, Ok(Token::End)),
        (7, Ok(Token::Operator(Operator::While))),
        (7, Ok(Token::Operator(Operator::Dotimes))),
        (7, Ok(Token::Operator(Operator::Dotimes))),
        (7, Ok(Token::End)),
        (7, Ok(Token::Begin)),
        (7, Ok(Token::Assign)),
        (7, Ok(Token::Operator(Operator::If))),
        (7, Ok(Token::Operator(Operator::If))),
        (7, Ok(Token::Operator(Operator::Print))),
        (8, Ok(Token::Operator(Operator::And))),
        (8, Ok(Token::Operator(Operator::And))),
        (8, Ok(Token::Operator(Operator::Or))),
        (8, Ok(Token::Operator(Operator::Or))),
        (8, Ok(Token::Operator(Operator::Not))),
        (8, Ok(Token::Operator(Operator::Not))),
        (9, Ok(Token::Operator(Operator::Natural))),
        (9, Ok(Token::Operator(Operator::Natural))),
        (9, Ok(Token::Operator(Operator::Integer))),
        (9, Ok(Token::Operator(Operator::Integer))),
        (9, Ok(Token::Operator(Operator::Float))),
        (9, Ok(Token::Operator(Operator::Float))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_identifier() {
    let lexer = Lexer::new(
        "-> var 100 ->
         begin VAR 200 + end begin true end while
         definition_var looped while_not# variable",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::Assign)),
        (1, Ok(Token::Identifier(String::from("var")))),
        (1, Ok(Token::Number(Number::Natural(100)))),
        (1, Ok(Token::Assign)),
        (2, Ok(Token::Begin)),
        (2, Ok(Token::Identifier(String::from("var")))),
        (2, Ok(Token::Number(Number::Natural(200)))),
        (2, Ok(Token::Operator(Operator::Plus))),
        (2, Ok(Token::End)),
        (2, Ok(Token::Begin)),
        (2, Ok(Token::Boolean(true))),
        (2, Ok(Token::End)),
        (2, Ok(Token::Operator(Operator::While))),
        (3, Ok(Token::Identifier(String::from("definition_var")))),
        (3, Ok(Token::Identifier(String::from("looped")))),
        (3, Ok(Token::Identifier(String::from("while_not")))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_whitespace() {
    let lexer = Lexer::new(
        "\r\t100 200\t\r\n + \n\n 200 100 +\n\n\n\"hallo\"\t\t\t",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::Number(Number::Natural(100)))),
        (1, Ok(Token::Number(Number::Natural(200)))),
        (2, Ok(Token::Operator(Operator::Plus))),
        (4, Ok(Token::Number(Number::Natural(200)))),
        (4, Ok(Token::Number(Number::Natural(100)))),
        (4, Ok(Token::Operator(Operator::Plus))),
        (7, Ok(Token::String(String::from("hallo")))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_operators() {
    let lexer = Lexer::new(
        "\r\t+ -\t\r\n * /\n\n > >=\n\n\n< <=\t\t\t=",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::Operator(Operator::Plus))),
        (1, Ok(Token::Operator(Operator::Minus))),
        (2, Ok(Token::Operator(Operator::Mul))),
        (2, Ok(Token::Operator(Operator::Div))),
        (4, Ok(Token::Operator(Operator::Greater))),
        (4, Ok(Token::Operator(Operator::GreaterEqual))),
        (7, Ok(Token::Operator(Operator::Less))),
        (7, Ok(Token::Operator(Operator::LessEqual))),
        (7, Ok(Token::Operator(Operator::Equal))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_use() {
    let lexer = Lexer::new(
        "use \"test.pile\"\n use \"file\"",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::Use)),
        (1, Ok(Token::String("test.pile".to_owned()))),
        (2, Ok(Token::Use)),
        (2, Ok(Token::String("file".to_owned()))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_error_missing_backslash() {
    let lexer =
        Lexer::new("1 2 3 * + \"cool string\\", Rc::new(ProgramSource::Stdin));
    let expected = vec![
        (1, Ok(Token::Number(Number::Natural(1)))),
        (1, Ok(Token::Number(Number::Natural(2)))),
        (1, Ok(Token::Number(Number::Natural(3)))),
        (1, Ok(Token::Operator(Operator::Mul))),
        (1, Ok(Token::Operator(Operator::Plus))),
        (
            1,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (1, 1),
                "Missing character after backslash.".to_owned(),
            )),
        ),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_error_unknown_escape() {
    let lexer = Lexer::new(
        "\"cool string\\t\\z\" 3.14 \"some string\\a\\b\\c \\\"test\" 100",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (
            1,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (1, 1),
                "Unknown escape chars: \'\\z\'".to_owned(),
            )),
        ),
        (1, Ok(Token::Number(Number::Float(3.14)))),
        (
            1,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (1, 1),
                "Unknown escape chars: \'\\a\' \'\\b\' \'\\c\'".to_owned(),
            )),
        ),
        (1, Ok(Token::Number(Number::Natural(100)))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_error_unknown_char() {
    let lexer = Lexer::new(
        "\"var\" BEGIN 0 1 + 2 * \n{ END append }# comment",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::String(String::from("var")))),
        (1, Ok(Token::Begin)),
        (1, Ok(Token::Number(Number::Natural(0)))),
        (1, Ok(Token::Number(Number::Natural(1)))),
        (1, Ok(Token::Operator(Operator::Plus))),
        (1, Ok(Token::Number(Number::Natural(2)))),
        (1, Ok(Token::Operator(Operator::Mul))),
        (
            2,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (2, 2),
                "Unknown char '{'".to_owned(),
            )),
        ),
        (2, Ok(Token::End)),
        (2, Ok(Token::Identifier(String::from("append")))),
        (
            2,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (2, 2),
                "Unknown char '}'".to_owned(),
            )),
        ),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_error_invalid_number() {
    let lexer = Lexer::new(
        "BEGIN 002 122 + 2f \n 3d 3y * \n END append",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::Begin)),
        (1, Ok(Token::Number(Number::Natural(2)))),
        (1, Ok(Token::Number(Number::Natural(122)))),
        (1, Ok(Token::Operator(Operator::Plus))),
        (
            1,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (1, 1),
                "'2f' isn't a number".to_owned(),
            )),
        ),
        (
            2,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (2, 2),
                "'3d' isn't a number".to_owned(),
            )),
        ),
        (
            2,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (2, 2),
                "'3y' isn't a number".to_owned(),
            )),
        ),
        (2, Ok(Token::Operator(Operator::Mul))),
        (3, Ok(Token::End)),
        (3, Ok(Token::Identifier(String::from("append")))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_error_unknown_operator() {
    let lexer = Lexer::new(
        "BEGIN x ++ y \n -- /= * \n END append",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::Begin)),
        (1, Ok(Token::Identifier(String::from("x")))),
        (
            1,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (1, 1),
                "Unknown operator '++'".to_owned(),
            )),
        ),
        (1, Ok(Token::Identifier(String::from("y")))),
        (
            2,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (2, 2),
                "Unknown operator '--'".to_owned(),
            )),
        ),
        (
            2,
            Err(PileError::new(
                Rc::new(ProgramSource::Stdin),
                (2, 2),
                "Unknown operator '/='".to_owned(),
            )),
        ),
        (2, Ok(Token::Operator(Operator::Mul))),
        (3, Ok(Token::End)),
        (3, Ok(Token::Identifier(String::from("append")))),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_token_fmt() {
    assert_eq!(
        format!("{}", Token::Number(Number::Natural(10))),
        "natural '10'"
    );
    assert_eq!(
        format!("{}", Token::Number(Number::Integer(-10))),
        "integer '-10'"
    );
    assert_eq!(
        format!("{}", Token::Number(Number::Float(42.42))),
        "float '42.42'"
    );
    assert_eq!(
        format!("{}", Token::Identifier("var".to_owned())),
        "identifier 'var'"
    );
    assert_eq!(
        format!("{}", Token::String("hello".to_owned())),
        "string \"hello\""
    );
    assert_eq!(format!("{}", Token::Boolean(true)), "boolean 'true'");
    assert_eq!(format!("{}", Token::Boolean(false)), "boolean 'false'");
    assert_eq!(format!("{}", Token::Begin), "token 'begin'");
    assert_eq!(format!("{}", Token::End), "token 'end'");
    assert_eq!(
        format!("{}", Token::Operator(Operator::If)),
        "operator 'if'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Dotimes)),
        "operator 'dotimes'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::While)),
        "operator 'while'"
    );
    assert_eq!(format!("{}", Token::Assign), "token '->'");
    assert_eq!(
        format!("{}", Token::Operator(Operator::Plus)),
        "operator '+'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Minus)),
        "operator '-'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Div)),
        "operator '/'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Mul)),
        "operator '*'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Greater)),
        "operator '>'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::GreaterEqual)),
        "operator '>='"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Equal)),
        "operator '='"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::LessEqual)),
        "operator '<='"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Less)),
        "operator '<'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::And)),
        "operator 'and'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Or)),
        "operator 'or'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Not)),
        "operator 'not'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Print)),
        "operator 'print'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Natural)),
        "operator 'natural'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Integer)),
        "operator 'integer'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Float)),
        "operator 'float'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Assert)),
        "operator 'assert'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Dup)),
        "operator 'dup'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Drop)),
        "operator 'drop'"
    );
    assert_eq!(
        format!("{}", Token::Operator(Operator::Swap)),
        "operator 'swap'"
    );
    assert_eq!(format!("{}", Token::Use), "token 'use'");
}
