use super::*;
use crate::lex::{Number, Operator, Token};
use crate::parse::{Ast, Expr};
use crate::program_source::ProgramSource;

use std::path::PathBuf;
use std::rc::Rc;

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
fn test_fmt_traced_expr() {
    assert_eq!(
        format!(
            "{}",
            TracedExpr(&Expr::Atom {
                token: Token::Use,
                line: 123
            })
        ),
        "use"
    );

    assert_eq!(
        format!(
            "{}",
            TracedExpr(&Expr::Assignment {
                var: "n".to_owned(),
                line: 123
            })
        ),
        "-> n"
    );

    assert_eq!(
        format!(
            "{}",
            TracedExpr(&Expr::Block {
                expressions: Rc::new(vec!(Expr::Assignment {
                    var: "x".to_owned(),
                    line: 123
                })),
                locals: vec![],
                begin: 1,
                end: 2
            })
        ),
        "begin -> x end"
    );

    assert_eq!(
        format!(
            "{}",
            TracedExpr(&Expr::Block {
                expressions: Rc::new(vec!(Expr::Atom {
                    token: Token::Number(Number::Natural(111)),
                    line: 123
                })),
                locals: vec![],
                begin: 1,
                end: 2
            })
        ),
        "begin 111 end"
    );

    assert_eq!(
        format!(
            "{}",
            TracedExpr(&Expr::Block {
                expressions: Rc::new(vec!(
                    Expr::Save {
                        line: 1,
                        var: "a".to_owned()
                    },
                    Expr::Atom {
                        token: Token::Number(Number::Natural(111)),
                        line: 123
                    },
                    Expr::Restore {
                        line: 1,
                        var: "a".to_owned()
                    },
                )),
                locals: vec!["a".to_owned()],
                begin: 1,
                end: 2
            })
        ),
        r#"begin save("a") 111 restore("a") end"#
    );

    assert_eq!(
        format!(
            "{}",
            TracedExpr(&Expr::Use {
                subprogram: Ast {
                    source: Rc::new(ProgramSource::File(PathBuf::from(
                        "other_file.pile"
                    ))),
                    expressions: vec![],
                },
                line: 1,
            })
        ),
        "use \"other_file.pile\""
    );
}
