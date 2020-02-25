use super::*;
use crate::lex::Number;
use crate::lex::Operator;
use crate::pile_error::PileError;
use crate::program_source::ProgramSource;

fn ast_assert_eq(left: &Ast, right: &Ast) {
    let left_iter = left.expressions.iter();
    let right_iter = right.expressions.iter();

    assert_eq!(left.expressions.len(), right.expressions.len());

    for (expr_left, expr_right) in left_iter.zip(right_iter) {
        assert_eq!(expr_left, expr_right)
    }
}

fn expect_ast(input: &str, ast: Ast) {
    let lex = Lexer::new(input, Rc::new(ProgramSource::Stdin));
    let parser = Parser::new(lex);
    let result_ast = parser.parse().unwrap();

    ast_assert_eq(&ast, &result_ast);
}

fn expect_error(input: &str, err: Result<Ast, PileError>) {
    let lex = Lexer::new(input, Rc::new(ProgramSource::Stdin));
    let parser = Parser::new(lex);
    let result_err = parser.parse();

    assert_eq!(err, result_err);
}

#[test]
fn test_simple1() {
    expect_ast(
        "",
        Ast {
            source: Rc::new(ProgramSource::Stdin),
            expressions: vec![],
        },
    )
}

#[test]
fn test_simple2() {
    expect_ast(
        "100 200 +",
        Ast {
            source: Rc::new(ProgramSource::Stdin),
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
            source: Rc::new(ProgramSource::Stdin),
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
        "100 -> var",
        Ast {
            source: Rc::new(ProgramSource::Stdin),
            expressions: vec![
                Expr::Atom {
                    line: 1,
                    token: Token::Number(Number::Natural(100)),
                },
                Expr::Assignment {
                    line: 1,
                    var: String::from("var"),
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
            source: Rc::new(ProgramSource::Stdin),
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
            source: Rc::new(ProgramSource::Stdin),
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
            source: Rc::new(ProgramSource::Stdin),
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
fn test_use() {
    expect_ast(
        "
use \"file_a\"
1 2 3 +
use \"file_b\"
use \"file_c\"
100 200 +
",
        Ast {
            source: Rc::new(ProgramSource::Stdin),
            expressions: vec![
                Expr::Use {
                    line: 2,
                    subprogram: Ast {
                        source: Rc::new(ProgramSource::File(PathBuf::from(
                            "file_a",
                        ))),
                        expressions: vec![],
                    },
                },
                Expr::Atom {
                    line: 3,
                    token: Token::Number(Number::Natural(1)),
                },
                Expr::Atom {
                    line: 3,
                    token: Token::Number(Number::Natural(2)),
                },
                Expr::Atom {
                    line: 3,
                    token: Token::Number(Number::Natural(3)),
                },
                Expr::Atom {
                    line: 3,
                    token: Token::Operator(Operator::Plus),
                },
                Expr::Use {
                    line: 4,
                    subprogram: Ast {
                        source: Rc::new(ProgramSource::File(PathBuf::from(
                            "file_b",
                        ))),
                        expressions: vec![],
                    },
                },
                Expr::Use {
                    line: 5,
                    subprogram: Ast {
                        source: Rc::new(ProgramSource::File(PathBuf::from(
                            "file_c",
                        ))),
                        expressions: vec![],
                    },
                },
                Expr::Atom {
                    line: 6,
                    token: Token::Number(Number::Natural(100)),
                },
                Expr::Atom {
                    line: 6,
                    token: Token::Number(Number::Natural(200)),
                },
                Expr::Atom {
                    line: 6,
                    token: Token::Operator(Operator::Plus),
                },
            ],
        },
    );
}

#[test]
fn test_error_use_in_block() {
    expect_error(
        "
begin
    use \"file1\"
end
",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (3, 3),
            "'use' isn't allowed inside blocks.".to_owned(),
        )),
    )
}

#[test]
fn test_error_use_in_assign() {
    expect_error(
        "
1 -> use
",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (2, 2),
            "Expected identifier found token 'use'.".to_owned(),
        )),
    )
}

#[test]
fn test_error_use_wrong_arg() {
    expect_error(
        "
use 42
",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (2, 2),
            "Expected string found natural '42'.".to_owned(),
        )),
    )
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
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (5, 5),
            "Unmatched 'end'.".to_owned(),
        )),
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
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (9, 9),
            "Expected 'end' found end of file.".to_owned(),
        )),
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
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (14, 14),
            "Expected 'end' found end of file.".to_owned(),
        )),
    )
}

#[test]
fn test_error_bad_assign1() {
    expect_error(
        "->",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Expected identifier found end of file.".to_string(),
        )),
    )
}

#[test]
fn test_error_bad_assign2() {
    expect_error(
        "-> end",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Expected identifier found token 'end'.".to_string(),
        )),
    )
}

#[test]
fn test_error_bad_assign3() {
    expect_error(
        "-> -> x",
        Err(PileError::new(
            Rc::new(ProgramSource::Stdin),
            (1, 1),
            "Expected identifier found token '->'.".to_string(),
        )),
    )
}
