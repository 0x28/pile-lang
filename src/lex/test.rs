use super::*;

fn token_list(lexer: &mut Lexer) -> Vec<(u64, Result<Token, String>)> {
    let mut result = vec![];
    loop {
        match lexer.next() {
            (_, Ok(Token::Fin)) => break,
            token => result.push(token),
        };
    }

    result
}

fn compare_token_lists(
    lexer: &mut Lexer,
    expected: Vec<(u64, Result<Token, String>)>,
) {
    let results = token_list(lexer);

    assert_eq!(results.len(), expected.len());

    for (actual, expected) in results.iter().zip(expected.iter()) {
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_empty_program() {
    let mut lexer = Lexer::new("");
    let result = token_list(&mut lexer);

    assert_eq!(result.len(), 0);
}

#[test]
fn test_comment_simple() {
    let mut lexer = Lexer::new("2 3 *# hello world\n1 1 +");
    let expected = vec![
        (1, Ok(Token::Number(Number::Natural(2)))),
        (1, Ok(Token::Number(Number::Natural(3)))),
        (1, Ok(Token::Mul)),
        (2, Ok(Token::Number(Number::Natural(1)))),
        (2, Ok(Token::Number(Number::Natural(1)))),
        (2, Ok(Token::Plus)),
    ];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_comment_only() {
    let mut lexer = Lexer::new("# empty program");
    let result = token_list(&mut lexer);

    assert_eq!(result.len(), 0);
}

#[test]
fn test_string_simple() {
    let mut lexer = Lexer::new("\"yay programming languages :)\"# comment");
    let expected = vec![(
        1,
        Ok(Token::String(String::from("yay programming languages :)"))),
    )];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_string_escaped() {
    let mut lexer = Lexer::new("\"\\n\\n\\n\\t\\r\0#test#\"");
    let expected =
        vec![(1, Ok(Token::String(String::from("\n\n\n\t\r\0#test#"))))];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_unknown_char() {
    let mut lexer = Lexer::new("\\hello world\\");
    let expected = vec![
        (1, Err(String::from("Unknown char '\\'"))),
        (1, Ok(Token::Identifier(String::from("hello")))),
        (1, Ok(Token::Identifier(String::from("world")))),
        (1, Err(String::from("Unknown char '\\'"))),
    ];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_numbers_natural() {
    let mut lexer = Lexer::new(
        "100 2000 3000 123 4543 123 21393#123#123
         203 040 05060 70 80 002 1203004 003",
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

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_numbers_integer() {
    let mut lexer = Lexer::new(
        "-1\n-2\n-3\n-4000\n-0044
        -1000 -200 -42 -42",
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

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_numbers_float() {
    let mut lexer = Lexer::new(
        "1.1\n2.2\n3.3\n-10e20\n-inf +inf
         3.1415 7777.7777 -3e-10#number :)",
    );
    let expected = vec![
        (1, Ok(Token::Number(Number::Float(1.1)))),
        (2, Ok(Token::Number(Number::Float(2.2)))),
        (3, Ok(Token::Number(Number::Float(3.3)))),
        (4, Ok(Token::Number(Number::Float(-10e20)))),
        (5, Ok(Token::Number(Number::Float(std::f32::NEG_INFINITY)))),
        (5, Ok(Token::Number(Number::Float(std::f32::INFINITY)))),
        (6, Ok(Token::Number(Number::Float(3.1415)))),
        (6, Ok(Token::Number(Number::Float(7777.7777)))),
        (6, Ok(Token::Number(Number::Float(-3e-10)))),
    ];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_keywords() {
    let mut lexer = Lexer::new(
        "
        begin#what
          10 +
          100 *# this is a operator
        end

        while def dotimes LOOP DEF DOTIMES END BEGIN QUOTE quote if IF",
    );
    let expected = vec![
        (2, Ok(Token::Begin)),
        (3, Ok(Token::Number(Number::Natural(10)))),
        (3, Ok(Token::Plus)),
        (4, Ok(Token::Number(Number::Natural(100)))),
        (4, Ok(Token::Mul)),
        (5, Ok(Token::End)),
        (7, Ok(Token::While)),
        (7, Ok(Token::Def)),
        (7, Ok(Token::Dotimes)),
        (7, Ok(Token::Loop)),
        (7, Ok(Token::Def)),
        (7, Ok(Token::Dotimes)),
        (7, Ok(Token::End)),
        (7, Ok(Token::Begin)),
        (7, Ok(Token::Quote)),
        (7, Ok(Token::Quote)),
        (7, Ok(Token::If)),
        (7, Ok(Token::If)),
    ];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_identifier() {
    let mut lexer = Lexer::new(
        "quote var 100 def
         begin VAR 200 + end loop
         definition_var looped while_not# variable",
    );
    let expected = vec![
        (1, Ok(Token::Quote)),
        (1, Ok(Token::Identifier(String::from("var")))),
        (1, Ok(Token::Number(Number::Natural(100)))),
        (1, Ok(Token::Def)),
        (2, Ok(Token::Begin)),
        (2, Ok(Token::Identifier(String::from("var")))),
        (2, Ok(Token::Number(Number::Natural(200)))),
        (2, Ok(Token::Plus)),
        (2, Ok(Token::End)),
        (2, Ok(Token::Loop)),
        (3, Ok(Token::Identifier(String::from("definition_var")))),
        (3, Ok(Token::Identifier(String::from("looped")))),
        (3, Ok(Token::Identifier(String::from("while_not")))),
    ];

    compare_token_lists(&mut lexer, expected);
}


#[test]
fn test_whitespace() {
    let mut lexer =
        Lexer::new("\r\t100 200\t\r\n + \n\n 200 100 +\n\n\n\"hallo\"\t\t\t");
    let expected = vec![
        (1, Ok(Token::Number(Number::Natural(100)))),
        (1, Ok(Token::Number(Number::Natural(200)))),
        (2, Ok(Token::Plus)),
        (4, Ok(Token::Number(Number::Natural(200)))),
        (4, Ok(Token::Number(Number::Natural(100)))),
        (4, Ok(Token::Plus)),
        (7, Ok(Token::String(String::from("hallo")))),
    ];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_operators() {
    let mut lexer =
        Lexer::new("\r\t+ -\t\r\n * /\n\n > >=\n\n\n< <=\t\t\t=");
    let expected = vec![
        (1, Ok(Token::Plus)),
        (1, Ok(Token::Minus)),
        (2, Ok(Token::Mul)),
        (2, Ok(Token::Div)),
        (4, Ok(Token::Greater)),
        (4, Ok(Token::GreaterEqual)),
        (7, Ok(Token::Less)),
        (7, Ok(Token::LessEqual)),
        (7, Ok(Token::Equal)),
    ];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_error_missing_backspace() {
    let mut lexer = Lexer::new("1 2 3 * + \"cool string\\");
    let expected = vec![
        (1, Ok(Token::Number(Number::Natural(1)))),
        (1, Ok(Token::Number(Number::Natural(2)))),
        (1, Ok(Token::Number(Number::Natural(3)))),
        (1, Ok(Token::Mul)),
        (1, Ok(Token::Plus)),
        (1, Err(String::from("Missing character after backspace."))),
    ];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_error_unknown_escape() {
    let mut lexer = Lexer::new(
        "\"cool string\\t\\z\" 3.14 \"some string\\a\\b\\c \\\"test\" 100",
    );
    let expected = vec![
        (1, Err(String::from("Unknown escape chars: \'\\z\'"))),
        (1, Ok(Token::Number(Number::Float(3.14)))),
        (
            1,
            Err(String::from(
                "Unknown escape chars: \'\\a\' \'\\b\' \'\\c\'",
            )),
        ),
        (1, Ok(Token::Number(Number::Natural(100)))),
    ];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_error_unknown_char() {
    let mut lexer =
        Lexer::new("\"var\" BEGIN 0 1 + 2 * \n{ END append }# comment");
    let expected = vec![
        (1, Ok(Token::String(String::from("var")))),
        (1, Ok(Token::Begin)),
        (1, Ok(Token::Number(Number::Natural(0)))),
        (1, Ok(Token::Number(Number::Natural(1)))),
        (1, Ok(Token::Plus)),
        (1, Ok(Token::Number(Number::Natural(2)))),
        (1, Ok(Token::Mul)),
        (2, Err(String::from("Unknown char '{'"))),
        (2, Ok(Token::End)),
        (2, Ok(Token::Identifier(String::from("append")))),
        (2, Err(String::from("Unknown char '}'"))),
    ];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_error_invalid_number() {
    let mut lexer = Lexer::new("BEGIN 002 122 + 2f \n 3d 3y * \n END append");
    let expected = vec![
        (1, Ok(Token::Begin)),
        (1, Ok(Token::Number(Number::Natural(2)))),
        (1, Ok(Token::Number(Number::Natural(122)))),
        (1, Ok(Token::Plus)),
        (1, Err(String::from("'2f' isn't a number"))),
        (2, Err(String::from("'3d' isn't a number"))),
        (2, Err(String::from("'3y' isn't a number"))),
        (2, Ok(Token::Mul)),
        (3, Ok(Token::End)),
        (3, Ok(Token::Identifier(String::from("append")))),
    ];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_error_unknown_operator() {
    let mut lexer = Lexer::new("BEGIN x ++ y \n -- /= * \n END append");
    let expected = vec![
        (1, Ok(Token::Begin)),
        (1, Ok(Token::Identifier(String::from("x")))),
        (1, Err(String::from("Unknown operator '++'"))),
        (1, Ok(Token::Identifier(String::from("y")))),
        (2, Err(String::from("Unknown operator '--'"))),
        (2, Err(String::from("Unknown operator '/='"))),
        (2, Ok(Token::Mul)),
        (3, Ok(Token::End)),
        (3, Ok(Token::Identifier(String::from("append")))),
    ];

    compare_token_lists(&mut lexer, expected);
}
