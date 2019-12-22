use super::*;
use crate::lex::Number;
use crate::lex::Operator;

fn ast_assert_eq(left: &Ast, right: &Ast) {
    let left_iter = left.expressions.iter();
    let right_iter = right.expressions.iter();

    assert_eq!(left.expressions.len(), right.expressions.len());

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

fn expect_error(input: &str, err: Result<Ast, String>) {
    let lex = Lexer::new(input);
    let parser = Parser::new(lex);
    let result_err = parser.parse();

    assert_eq!(err, result_err);
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
                    token: Token::Operator(Operator::Plus),
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
                Expr::Quoted {
                    line: 1,
                    token: Token::Identifier(String::from("var")),
                },
                Expr::Atom {
                    line: 1,
                    token: Token::Number(Number::Natural(100)),
                },
                Expr::Atom {
                    line: 1,
                    token: Token::Operator(Operator::Def),
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
                Expr::Block {
                    begin: 1,
                    end: 1,
                    expressions: Rc::new(vec![Expr::Atom {
                        line: 1,
                        token: Token::Number(Number::Natural(100)),
                    }]),
                },
                Expr::Atom {
                    line: 1,
                    token: Token::Number(Number::Natural(20)),
                },
                Expr::Atom {
                    line: 1,
                    token: Token::Operator(Operator::Dotimes),
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
                Expr::Block {
                    begin: 1,
                    end: 1,
                    expressions: Rc::new(vec![Expr::Atom {
                        line: 1,
                        token: Token::Number(Number::Natural(100)),
                    }]),
                },
                Expr::Block {
                    begin: 1,
                    end: 1,
                    expressions: Rc::new(vec![Expr::Atom {
                        line: 1,
                        token: Token::Number(Number::Integer(-100)),
                    }]),
                },
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
                    token: Token::Operator(Operator::Greater),
                },
                Expr::Atom {
                    line: 1,
                    token: Token::Operator(Operator::If),
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
            expressions: vec![Expr::Block {
                begin: 2,
                end: 10,
                expressions: Rc::new(vec![
                    Expr::Block {
                        begin: 3,
                        end: 5,
                        expressions: Rc::new(vec![Expr::Atom {
                            line: 4,
                            token: Token::String(String::from("a")),
                        }]),
                    },
                    Expr::Block {
                        begin: 7,
                        end: 9,
                        expressions: Rc::new(vec![
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
                                token: Token::Operator(Operator::Plus),
                            },
                        ]),
                    },
                ]),
            }],
        },
    );
}

#[test]
fn test_error_unmatched_end() {
    expect_error(
        "
begin
    +
end
end
",
        Err(String::from("Line 5: Unmatched 'end'.")),
    )
}

#[test]
fn test_error_no_end1() {
    expect_error(
        "
begin
    1
    begin
        2
    end
    +
    *
",
        Err(String::from("Line 9: Expected 'end' found 'EOF'.")),
    )
}

#[test]
fn test_error_no_end2() {
    expect_error(
        "
begin 1
  begin 2
    begin 1
      begin 2
        begin 1
          begin 2
            begin 1
              begin 2
                begin 1
                  begin 2
                    begin 1
                      begin 2
",
        Err(String::from("Line 14: Expected 'end' found 'EOF'.")),
    )
}

#[test]
fn test_error_bad_quote1() {
    expect_error("quote", Err("Line 1: Unexpected 'EOF'".to_string()))
}

#[test]
fn test_error_bad_quote2() {
    expect_error(
        "quote end",
        Err("Line 1: Unexpected token 'end'".to_string()),
    )
}
