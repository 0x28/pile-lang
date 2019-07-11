use super::*;
use crate::lex::Number;

fn ast_assert_eq(left: &Ast, right: &Ast) {
    let left_iter = left.expressions.iter();
    let right_iter = right.expressions.iter();

    for (expr_left, expr_right) in left_iter.zip(right_iter) {
        assert_eq!(expr_left, expr_right)
    }
}

fn expect_ast(input: &str, ast: Ast) {
    let lex = Lexer::new(input);
    let parser = Parser::new(lex);
    let result_ast = parser.parse().unwrap();

    ast_assert_eq(&ast, &result_ast);
}

#[test]
fn test_simple1() {
    expect_ast(
        "",
        Ast {
            expressions: vec![],
        },
    )
}

#[test]
fn test_simple2() {
    expect_ast(
        "100 200 +",
        Ast {
            expressions: vec![
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
            ],
        },
    )
}

#[test]
fn test_simple3() {
    expect_ast(
        "\"hello world\" \" test\" append",
        Ast {
            expressions: vec![
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
            ],
        },
    )
}

#[test]
fn test_simple4() {
    expect_ast(
        "quote var 100 def",
        Ast {
            expressions: vec![
                Expr::Atom {
                    line: 1,
                    token: Token::Quote,
                },
                Expr::Atom {
                    line: 1,
                    token: Token::Identifier(String::from("var")),
                },
                Expr::Atom {
                    line: 1,
                    token: Token::Number(Number::Natural(100)),
                },
                Expr::Atom {
                    line: 1,
                    token: Token::Def,
                },
            ],
        },
    )
}

#[test]
fn test_block1() {
    expect_ast(
        "begin 100 end 20 dotimes",
        Ast {
            expressions: vec![
                Expr::Block(vec![Expr::Atom {
                    line: 1,
                    token: Token::Number(Number::Natural(100)),
                }]),
                Expr::Atom {
                    line: 1,
                    token: Token::Number(Number::Natural(20)),
                },
                Expr::Atom {
                    line: 1,
                    token: Token::Dotimes,
                },
            ],
        },
    )
}

#[test]
fn test_block2() {
    expect_ast(
        "begin 100 end begin -100 end 1 2 > if",
        Ast {
            expressions: vec![
                Expr::Block(vec![Expr::Atom {
                    line: 1,
                    token: Token::Number(Number::Natural(100)),
                }]),
                Expr::Block(vec![Expr::Atom {
                    line: 1,
                    token: Token::Number(Number::Integer(-100)),
                }]),
                Expr::Atom {
                    line: 1,
                    token: Token::Number(Number::Natural(1)),
                },
                Expr::Atom {
                    line: 1,
                    token: Token::Number(Number::Natural(2)),
                },
                Expr::Atom {
                    line: 1,
                    token: Token::Greater,
                },
                Expr::Atom {
                    line: 1,
                    token: Token::If,
                },
            ],
        },
    )
}

#[test]
fn test_block3() {
    expect_ast(
        "
begin
    begin
        \"a\"
    end

    begin
        \"b\" 3.14 +
    end
end",
        Ast {
            expressions: vec![Expr::Block(vec![
                Expr::Block(vec![Expr::Atom {
                    line: 4,
                    token: Token::String(String::from("a")),
                }]),
                Expr::Block(vec![
                    Expr::Atom {
                        line: 8,
                        token: Token::String(String::from("b")),
                    },
                    Expr::Atom {
                        line: 8,
                        token: Token::Number(Number::Float(3.14)),
                    },
                    Expr::Atom {
                        line: 8,
                        token: Token::Plus,
                    },
                ]),
            ])],
        },
    );
}
