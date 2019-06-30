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

    for (actual, expected) in results.iter().zip(expected.iter())
    {
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
