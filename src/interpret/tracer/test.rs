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
