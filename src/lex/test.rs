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
    let mut lexer = Lexer::new("".chars());
    let result = token_list(&mut lexer);

    assert_eq!(result.len(), 0);
}

#[test]
fn test_comment_simple() {
    let mut lexer = Lexer::new("# hello world\n1 1 +".chars());
    let expected = vec![
        (2, Ok(Token::Number(Number::Natural(1)))),
        (2, Ok(Token::Number(Number::Natural(1)))),
        (2, Ok(Token::Plus)),
    ];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_comment_only() {
    let mut lexer = Lexer::new("# empty program".chars());
    let result = token_list(&mut lexer);

    assert_eq!(result.len(), 0);
}

#[test]
fn test_string_simple() {
    let mut lexer = Lexer::new("\"yay programming languages :)\"".chars());
    let expected = vec![(
        1,
        Ok(Token::String(String::from("yay programming languages :)"))),
    )];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_string_escaped() {
    let mut lexer = Lexer::new("\"\\n\\n\\n\\t\\r\0\"".chars());
    let expected = vec![(1, Ok(Token::String(String::from("\n\n\n\t\r\0"))))];

    compare_token_lists(&mut lexer, expected);
}

#[test]
fn test_unknown_char() {
    let mut lexer = Lexer::new("\\hello world\\".chars());
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
        "100 2000 3000 123 4543 123 21393
         203 040 05060 70 80 002 1203004 003"
            .chars(),
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
        -1000 -200 -42 -42"
        .chars(),
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
