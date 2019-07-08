use super::*;
use crate::lex::Number;

fn expect_ast(input: &str, ast: Ast) {
    let lex = Lexer::new(input);
    let parser = Parser::new(lex);

    assert_eq!(ast, parser.parse().unwrap());
}

#[test]
fn test_simple1() {
    expect_ast("", Ast::Program(vec![]))
}

#[test]
fn test_simple2() {
    expect_ast(
        "100 200 +",
        Ast::Program(vec![
            Expr::Atom {
                line: 1,
                token: Token::Number(Number::Natural(100)),
            },
            Expr::Atom {
                line: 1,
                token: Token::Number(Number::Natural(200)),
            },
            Expr::Atom {
                line: 1,
                token: Token::Plus,
            },
        ]),
    )
}

#[test]
fn test_simple3() {
    expect_ast(
        "\"hello world\" \" test\" append",
        Ast::Program(vec![
            Expr::Atom {
                line: 1,
                token: Token::String(String::from("hello world")),
            },
            Expr::Atom {
                line: 1,
                token: Token::String(String::from(" test")),
            },
            Expr::Atom {
                line: 1,
                token: Token::Identifier(String::from("append")),
            },
        ]),
    )
}
