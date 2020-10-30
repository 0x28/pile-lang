use super::*;
use crate::program_source::ProgramSource;

fn compare_token_lists(
    lexer: Lexer,
    expected: Vec<(u64, Result<Token, PileError>, &str)>,
) {
    let result: Vec<_> = lexer.collect();

    assert_eq!(result.len(), expected.len());

    for (actual, expected) in result.into_iter().zip(expected.into_iter()) {
        let (line, token, lexeme) = expected;
        let lexeme = lexeme.to_owned();
        assert_eq!(
            actual,
            LexerItem {
                line,
                token,
                lexeme
            }
        );
    }
}

#[test]
fn test_empty_program() {
    let lexer = Lexer::new("", Rc::new(ProgramSource::Stdin));

    assert_eq!(lexer.count(), 0);
}

#[test]
fn test_comment_simple() {
    let lexer =
        Lexer::new("2 3 *# hello world\n1 1 +", Rc::new(ProgramSource::Stdin));
    let expected = vec![
        (1, Ok(Token::Number(Number::Natural(2))), "2"),
        (1, Ok(Token::Number(Number::Natural(3))), "3"),
        (1, Ok(Token::Operator(Operator::Mul)), "*"),
        (
            1,
            Ok(Token::Comment(" hello world".to_owned())),
            "# hello world",
        ),
        (2, Ok(Token::Number(Number::Natural(1))), "1"),
        (2, Ok(Token::Number(Number::Natural(1))), "1"),
        (2, Ok(Token::Operator(Operator::Plus)), "+"),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_comment_only() {
    let lexer = Lexer::new("# empty #program", Rc::new(ProgramSource::Stdin));

    let expected = vec![(
        1,
        Ok(Token::Comment(" empty #program".to_owned())),
        "# empty #program",
    )];
    compare_token_lists(lexer, expected);
}

#[test]
fn test_string_simple() {
    let lexer = Lexer::new(
        "\"yay programming languages :)\"# comment",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (
            1,
            Ok(Token::String(String::from("yay programming languages :)"))),
            "\"yay programming languages :)\"",
        ),
        (1, Ok(Token::Comment(" comment".to_owned())), "# comment"),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_string_escaped() {
    let lexer = Lexer::new(
        "\"\\n\\n\\n\\t\\r\0#test#\"",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![(
        1,
        Ok(Token::String(String::from("\n\n\n\t\r\0#test#"))),
        "\"\\n\\n\\n\\t\\r\0#test#\"",
    )];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_unknown_char() {
    let lexer = Lexer::new("\\hello world\\", Rc::new(ProgramSource::Stdin));
    let expected = vec![
        (
            1,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                1,
                "Unknown char '\\'".to_owned(),
            )),
            "\\",
        ),
        (1, Ok(Token::Identifier(String::from("hello"))), "hello"),
        (1, Ok(Token::Identifier(String::from("world"))), "world"),
        (
            1,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                1,
                "Unknown char '\\'".to_owned(),
            )),
            "\\",
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
        (1, Ok(Token::Number(Number::Natural(100))), "100"),
        (1, Ok(Token::Number(Number::Natural(2000))), "2000"),
        (1, Ok(Token::Number(Number::Natural(3000))), "3000"),
        (1, Ok(Token::Number(Number::Natural(123))), "123"),
        (1, Ok(Token::Number(Number::Natural(4543))), "4543"),
        (1, Ok(Token::Number(Number::Natural(123))), "123"),
        (1, Ok(Token::Number(Number::Natural(21393))), "21393"),
        (1, Ok(Token::Comment("123#123".to_owned())), "#123#123"),
        (2, Ok(Token::Number(Number::Natural(203))), "203"),
        (2, Ok(Token::Number(Number::Natural(40))), "040"),
        (2, Ok(Token::Number(Number::Natural(5060))), "05060"),
        (2, Ok(Token::Number(Number::Natural(70))), "70"),
        (2, Ok(Token::Number(Number::Natural(80))), "80"),
        (2, Ok(Token::Number(Number::Natural(2))), "002"),
        (2, Ok(Token::Number(Number::Natural(1203004))), "1203004"),
        (2, Ok(Token::Number(Number::Natural(3))), "003"),
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
        (1, Ok(Token::Number(Number::Integer(-1))), "-1"),
        (2, Ok(Token::Number(Number::Integer(-2))), "-2"),
        (3, Ok(Token::Number(Number::Integer(-3))), "-3"),
        (4, Ok(Token::Number(Number::Integer(-4000))), "-4000"),
        (5, Ok(Token::Number(Number::Integer(-44))), "-0044"),
        (6, Ok(Token::Number(Number::Integer(-1000))), "-1000"),
        (6, Ok(Token::Number(Number::Integer(-200))), "-200"),
        (6, Ok(Token::Number(Number::Integer(-42))), "-42"),
        (6, Ok(Token::Number(Number::Integer(-42))), "-42"),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_numbers_float() {
    let lexer = Lexer::new(
        "1.1\n2.2\n3.3\n-10000000.0\n 20000.0 30E5
         3.2211 7777.7777 -0.00000003#number :)",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::Number(Number::Float(1.1))), "1.1"),
        (2, Ok(Token::Number(Number::Float(2.2))), "2.2"),
        (3, Ok(Token::Number(Number::Float(3.3))), "3.3"),
        (
            4,
            Ok(Token::Number(Number::Float(-10000000.0))),
            "-10000000.0",
        ),
        (5, Ok(Token::Number(Number::Float(20000.0))), "20000.0"),
        (5, Ok(Token::Number(Number::Float(3000000.0))), "30E5"),
        (6, Ok(Token::Number(Number::Float(3.2211))), "3.2211"),
        (6, Ok(Token::Number(Number::Float(7777.7777))), "7777.7777"),
        (
            6,
            Ok(Token::Number(Number::Float(-0.00000003))),
            "-0.00000003",
        ),
        (6, Ok(Token::Comment("number :)".to_owned())), "#number :)"),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_numbers_overflow() {
    let lexer = Lexer::new(
        "85892349393234324592 -858243349923432034592 +85648993234023044592",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (
            1,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                1,
                "'85892349393234324592' is too large to be represented as a \
                 number"
                    .to_string(),
            )),
            "85892349393234324592",
        ),
        (
            1,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                1,
                "'-858243349923432034592' is too small to be represented as a \
                 number"
                    .to_string(),
            )),
            "-858243349923432034592",
        ),
        (
            1,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                1,
                "'+85648993234023044592' is too large to be represented as a \
                 number"
                    .to_string(),
            )),
            "+85648993234023044592",
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
        (1, Ok(Token::Boolean(true)), "true"),
        (1, Ok(Token::Boolean(false)), "false"),
        (1, Ok(Token::Boolean(true)), "true"),
        (2, Ok(Token::Boolean(false)), "false"),
        (2, Ok(Token::Boolean(true)), "true"),
        (2, Ok(Token::Boolean(false)), "false"),
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
        natural NATURAL integer INTEGER float FLOAT let LET [[[]]]",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (2, Ok(Token::Begin), "begin"),
        (2, Ok(Token::Comment("what".to_owned())), "#what"),
        (3, Ok(Token::Number(Number::Natural(10))), "10"),
        (3, Ok(Token::Operator(Operator::Plus)), "+"),
        (4, Ok(Token::Number(Number::Natural(100))), "100"),
        (4, Ok(Token::Operator(Operator::Mul)), "*"),
        (
            4,
            Ok(Token::Comment(" this is a operator".to_owned())),
            "# this is a operator",
        ),
        (5, Ok(Token::End), "end"),
        (7, Ok(Token::Operator(Operator::While)), "while"),
        (7, Ok(Token::Operator(Operator::Dotimes)), "dotimes"),
        (7, Ok(Token::Operator(Operator::Dotimes)), "DOTIMES"),
        (7, Ok(Token::End), "END"),
        (7, Ok(Token::Begin), "BEGIN"),
        (7, Ok(Token::Assign), "->"),
        (7, Ok(Token::Operator(Operator::If)), "if"),
        (7, Ok(Token::Operator(Operator::If)), "IF"),
        (7, Ok(Token::Operator(Operator::Print)), "print"),
        (8, Ok(Token::Operator(Operator::And)), "and"),
        (8, Ok(Token::Operator(Operator::And)), "AND"),
        (8, Ok(Token::Operator(Operator::Or)), "or"),
        (8, Ok(Token::Operator(Operator::Or)), "OR"),
        (8, Ok(Token::Operator(Operator::Not)), "not"),
        (8, Ok(Token::Operator(Operator::Not)), "NOT"),
        (9, Ok(Token::Operator(Operator::Natural)), "natural"),
        (9, Ok(Token::Operator(Operator::Natural)), "NATURAL"),
        (9, Ok(Token::Operator(Operator::Integer)), "integer"),
        (9, Ok(Token::Operator(Operator::Integer)), "INTEGER"),
        (9, Ok(Token::Operator(Operator::Float)), "float"),
        (9, Ok(Token::Operator(Operator::Float)), "FLOAT"),
        (9, Ok(Token::Let), "let"),
        (9, Ok(Token::Let), "LET"),
        (9, Ok(Token::BracketLeft), "["),
        (9, Ok(Token::BracketLeft), "["),
        (9, Ok(Token::BracketLeft), "["),
        (9, Ok(Token::BracketRight), "]"),
        (9, Ok(Token::BracketRight), "]"),
        (9, Ok(Token::BracketRight), "]"),
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
        (1, Ok(Token::Assign), "->"),
        (1, Ok(Token::Identifier(String::from("var"))), "var"),
        (1, Ok(Token::Number(Number::Natural(100))), "100"),
        (1, Ok(Token::Assign), "->"),
        (2, Ok(Token::Begin), "begin"),
        (2, Ok(Token::Identifier(String::from("var"))), "VAR"),
        (2, Ok(Token::Number(Number::Natural(200))), "200"),
        (2, Ok(Token::Operator(Operator::Plus)), "+"),
        (2, Ok(Token::End), "end"),
        (2, Ok(Token::Begin), "begin"),
        (2, Ok(Token::Boolean(true)), "true"),
        (2, Ok(Token::End), "end"),
        (2, Ok(Token::Operator(Operator::While)), "while"),
        (
            3,
            Ok(Token::Identifier(String::from("definition_var"))),
            "definition_var",
        ),
        (3, Ok(Token::Identifier(String::from("looped"))), "looped"),
        (
            3,
            Ok(Token::Identifier(String::from("while_not"))),
            "while_not",
        ),
        (3, Ok(Token::Comment(" variable".to_owned())), "# variable"),
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
        (1, Ok(Token::Number(Number::Natural(100))), "100"),
        (1, Ok(Token::Number(Number::Natural(200))), "200"),
        (2, Ok(Token::Operator(Operator::Plus)), "+"),
        (4, Ok(Token::Number(Number::Natural(200))), "200"),
        (4, Ok(Token::Number(Number::Natural(100))), "100"),
        (4, Ok(Token::Operator(Operator::Plus)), "+"),
        (7, Ok(Token::String(String::from("hallo"))), "\"hallo\""),
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
        (1, Ok(Token::Operator(Operator::Plus)), "+"),
        (1, Ok(Token::Operator(Operator::Minus)), "-"),
        (2, Ok(Token::Operator(Operator::Mul)), "*"),
        (2, Ok(Token::Operator(Operator::Div)), "/"),
        (4, Ok(Token::Operator(Operator::Greater)), ">"),
        (4, Ok(Token::Operator(Operator::GreaterEqual)), ">="),
        (7, Ok(Token::Operator(Operator::Less)), "<"),
        (7, Ok(Token::Operator(Operator::LessEqual)), "<="),
        (7, Ok(Token::Operator(Operator::Equal)), "="),
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
        (1, Ok(Token::Use), "use"),
        (
            1,
            Ok(Token::String("test.pile".to_owned())),
            "\"test.pile\"",
        ),
        (2, Ok(Token::Use), "use"),
        (2, Ok(Token::String("file".to_owned())), "\"file\""),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_error_missing_backslash() {
    let lexer =
        Lexer::new("1 2 3 * + \"cool string\\", Rc::new(ProgramSource::Stdin));
    let expected = vec![
        (1, Ok(Token::Number(Number::Natural(1))), "1"),
        (1, Ok(Token::Number(Number::Natural(2))), "2"),
        (1, Ok(Token::Number(Number::Natural(3))), "3"),
        (1, Ok(Token::Operator(Operator::Mul)), "*"),
        (1, Ok(Token::Operator(Operator::Plus)), "+"),
        (
            1,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                1,
                "Missing character after backslash.".to_owned(),
            )),
            "\"cool string\\",
        ),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_error_unknown_escape() {
    let lexer = Lexer::new(
        "\"cool string\\t\\z\" 3.33 \"some string\\a\\b\\c \\\"test\" 100",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (
            1,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                1,
                r"Unknown escape chars: '\z'".to_owned(),
            )),
            "\"cool string\\t\\z\"",
        ),
        (1, Ok(Token::Number(Number::Float(3.33))), "3.33"),
        (
            1,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                1,
                r"Unknown escape chars: '\a' '\b' '\c'".to_owned(),
            )),
            "\"some string\\a\\b\\c \\\"test\"",
        ),
        (1, Ok(Token::Number(Number::Natural(100))), "100"),
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
        (1, Ok(Token::String(String::from("var"))), "\"var\""),
        (1, Ok(Token::Begin), "BEGIN"),
        (1, Ok(Token::Number(Number::Natural(0))), "0"),
        (1, Ok(Token::Number(Number::Natural(1))), "1"),
        (1, Ok(Token::Operator(Operator::Plus)), "+"),
        (1, Ok(Token::Number(Number::Natural(2))), "2"),
        (1, Ok(Token::Operator(Operator::Mul)), "*"),
        (
            2,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                2,
                "Unknown char '{'".to_owned(),
            )),
            "{",
        ),
        (2, Ok(Token::End), "END"),
        (2, Ok(Token::Identifier(String::from("append"))), "append"),
        (
            2,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                2,
                "Unknown char '}'".to_owned(),
            )),
            "}",
        ),
        (2, Ok(Token::Comment(" comment".to_owned())), "# comment"),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_error_invalid_number() {
    let lexer = Lexer::new(
        "BEGIN 002 122 + 2f \n -3d 3y * \n END append",
        Rc::new(ProgramSource::Stdin),
    );
    let expected = vec![
        (1, Ok(Token::Begin), "BEGIN"),
        (1, Ok(Token::Number(Number::Natural(2))), "002"),
        (1, Ok(Token::Number(Number::Natural(122))), "122"),
        (1, Ok(Token::Operator(Operator::Plus)), "+"),
        (
            1,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                1,
                "'2f' isn't a number".to_owned(),
            )),
            "2f",
        ),
        (
            2,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                2,
                "'-3d' isn't a number".to_owned(),
            )),
            "-3d",
        ),
        (
            2,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                2,
                "'3y' isn't a number".to_owned(),
            )),
            "3y",
        ),
        (2, Ok(Token::Operator(Operator::Mul)), "*"),
        (3, Ok(Token::End), "END"),
        (3, Ok(Token::Identifier(String::from("append"))), "append"),
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
        (1, Ok(Token::Begin), "BEGIN"),
        (1, Ok(Token::Identifier(String::from("x"))), "x"),
        (
            1,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                1,
                "Unknown operator '++'".to_owned(),
            )),
            "++",
        ),
        (1, Ok(Token::Identifier(String::from("y"))), "y"),
        (
            2,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                2,
                "Unknown operator '--'".to_owned(),
            )),
            "--",
        ),
        (
            2,
            Err(PileError::in_line(
                Rc::new(ProgramSource::Stdin),
                2,
                "Unknown operator '/='".to_owned(),
            )),
            "/=",
        ),
        (2, Ok(Token::Operator(Operator::Mul)), "*"),
        (3, Ok(Token::End), "END"),
        (3, Ok(Token::Identifier(String::from("append"))), "append"),
    ];

    compare_token_lists(lexer, expected);
}

#[test]
fn test_token_error_fmt() {
    assert_eq!(
        Token::Number(Number::Natural(10)).error_fmt(),
        "natural '10'"
    );
    assert_eq!(
        Token::Number(Number::Integer(-10)).error_fmt(),
        "integer '-10'"
    );
    assert_eq!(
        Token::Number(Number::Float(42.42)).error_fmt(),
        "float '42.42'"
    );
    assert_eq!(
        Token::Identifier("var".to_owned()).error_fmt(),
        "identifier 'var'"
    );
    assert_eq!(
        Token::String("hello".to_owned()).error_fmt(),
        "string \"hello\""
    );
    assert_eq!(Token::Boolean(true).error_fmt(), "boolean 'true'");
    assert_eq!(Token::Boolean(false).error_fmt(), "boolean 'false'");
    assert_eq!(Token::Begin.error_fmt(), "token 'begin'");
    assert_eq!(Token::End.error_fmt(), "token 'end'");
    assert_eq!(Token::Operator(Operator::If).error_fmt(), "operator 'if'");
    assert_eq!(
        Token::Operator(Operator::Dotimes).error_fmt(),
        "operator 'dotimes'"
    );
    assert_eq!(
        Token::Operator(Operator::While).error_fmt(),
        "operator 'while'"
    );
    assert_eq!(Token::Assign.error_fmt(), "token '->'");
    assert_eq!(Token::Operator(Operator::Plus).error_fmt(), "operator '+'");
    assert_eq!(Token::Operator(Operator::Minus).error_fmt(), "operator '-'");
    assert_eq!(Token::Operator(Operator::Div).error_fmt(), "operator '/'");
    assert_eq!(Token::Operator(Operator::Mul).error_fmt(), "operator '*'");
    assert_eq!(
        Token::Operator(Operator::Greater).error_fmt(),
        "operator '>'"
    );
    assert_eq!(
        Token::Operator(Operator::GreaterEqual).error_fmt(),
        "operator '>='"
    );
    assert_eq!(Token::Operator(Operator::Equal).error_fmt(), "operator '='");
    assert_eq!(
        Token::Operator(Operator::LessEqual).error_fmt(),
        "operator '<='"
    );
    assert_eq!(Token::Operator(Operator::Less).error_fmt(), "operator '<'");
    assert_eq!(Token::Operator(Operator::And).error_fmt(), "operator 'and'");
    assert_eq!(Token::Operator(Operator::Or).error_fmt(), "operator 'or'");
    assert_eq!(Token::Operator(Operator::Not).error_fmt(), "operator 'not'");
    assert_eq!(
        Token::Operator(Operator::Print).error_fmt(),
        "operator 'print'"
    );
    assert_eq!(
        Token::Operator(Operator::Natural).error_fmt(),
        "operator 'natural'"
    );
    assert_eq!(
        Token::Operator(Operator::Integer).error_fmt(),
        "operator 'integer'"
    );
    assert_eq!(
        Token::Operator(Operator::Float).error_fmt(),
        "operator 'float'"
    );
    assert_eq!(
        Token::Operator(Operator::Assert).error_fmt(),
        "operator 'assert'"
    );
    assert_eq!(Token::Operator(Operator::Dup).error_fmt(), "operator 'dup'");
    assert_eq!(
        Token::Operator(Operator::Drop).error_fmt(),
        "operator 'drop'"
    );
    assert_eq!(
        Token::Operator(Operator::Swap).error_fmt(),
        "operator 'swap'"
    );
    assert_eq!(Token::Use.error_fmt(), "token 'use'");
    assert_eq!(Token::Let.error_fmt(), "token 'let'");
    assert_eq!(Token::BracketLeft.error_fmt(), "token '['");
    assert_eq!(Token::BracketRight.error_fmt(), "token ']'");
}

#[test]
fn test_token_fmt() {
    assert_eq!(format!("{}", Token::Begin), "begin");
    assert_eq!(format!("{}", Token::End), "end");
    assert_eq!(format!("{}", Token::Let), "let");
    assert_eq!(format!("{}", Token::BracketLeft), "[");
    assert_eq!(format!("{}", Token::BracketRight), "]");
    assert_eq!(format!("{}", Token::Assign), "->");
    assert_eq!(format!("{}", Token::Operator(Operator::Plus)), "+");
    assert_eq!(format!("{}", Token::Number(Number::Float(12.34))), "12.34");
    assert_eq!(
        format!("{}", Token::String("hello".to_owned())),
        "\"hello\""
    );
    assert_eq!(format!("{}", Token::Use), "use");
    assert_eq!(format!("{}", Token::Boolean(true)), "true");
    assert_eq!(format!("{}", Token::Boolean(false)), "false");
    assert_eq!(format!("{}", Token::Identifier("var".to_owned())), "var");
}
