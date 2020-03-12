use super::*;
use crate::lex::{Number, Operator, Token};
use crate::parse::Expr;

#[test]
fn test_is_print() {
    assert_eq!(
        true,
        is_print(&Expr::Atom {
            token: Token::Operator(Operator::Print),
            line: 42
        })
    );

    assert_eq!(
        false,
        is_print(&Expr::Atom {
            token: Token::Operator(Operator::And),
            line: 1
        })
    );

    assert_eq!(
        false,
        is_print(&Expr::Atom {
            token: Token::Assign,
            line: 2
        })
    );

    assert_eq!(
        false,
        is_print(&Expr::Atom {
            token: Token::Number(Number::Natural(987)),
            line: 3
        })
    );
}

#[test]
fn test_fmt_traced_token() {
    assert_eq!(format!("{}", TracedToken(&Token::Begin)), "begin");
    assert_eq!(format!("{}", TracedToken(&Token::End)), "end");
    assert_eq!(format!("{}", TracedToken(&Token::Assign)), "->");
    assert_eq!(
        format!("{}", TracedToken(&Token::Operator(Operator::Plus))),
        "+"
    );
    assert_eq!(
        format!("{}", TracedToken(&Token::Number(Number::Float(12.34)))),
        "12.34"
    );
    assert_eq!(
        format!("{}", TracedToken(&Token::String("hello".to_owned()))),
        "\"hello\""
    );
    assert_eq!(format!("{}", TracedToken(&Token::Use)), "use");
    assert_eq!(format!("{}", TracedToken(&Token::Boolean(true))), "true");
    assert_eq!(format!("{}", TracedToken(&Token::Boolean(false))), "false");
    assert_eq!(
        format!("{}", TracedToken(&Token::Identifier("var".to_owned()))),
        "var"
    );
}
