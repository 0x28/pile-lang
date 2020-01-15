use crate::cli::ProgramSource;
use crate::lex::*;
use crate::parse::Ast;
use crate::parse::Expr;
use crate::parse::Parser;
use crate::pile_error::PileError;
use crate::using;

use std::fs;
use std::path::PathBuf;

fn resolve_file(s: &str) -> Result<Ast, PileError> {
    let input =
        fs::read_to_string(s).expect(&format!("couldn't read test file {}", s));
    let lexer = Lexer::new(&input, ProgramSource::File(PathBuf::from(s)));
    let parser = Parser::new(lexer);

    let ast = parser.parse().expect("couldn't parse test file");
    using::resolve(ast)
}

fn test_directory() -> String {
    env!("CARGO_MANIFEST_DIR").to_owned() + "/src/using/"
}

fn assert_resolve_eq(s: &str, expr: Vec<Expr>) {
    let path = test_directory() + s;
    let actual_ast = resolve_file(&path).expect("resolve error");
    let path = PathBuf::from(path);
    let expected_ast = Ast {
        source: ProgramSource::File(path),
        expressions: expr,
    };

    if actual_ast != expected_ast {
        panic!(format!(
            "ast doesn't match: {:#?} != {:#?}",
            actual_ast, expected_ast
        ));
    }
}

fn assert_resolve_error(s: &str, error: PileError) {
    let path = test_directory() + s;
    let actual_result = resolve_file(&path);

    assert_eq!(actual_result, Err(error));
}

#[test]
fn test_simple_use() {
    assert_resolve_eq(
        "test_simple/simple.pile",
        vec![Expr::Use {
            line: 1,
            subprogram: Ast {
                source: ProgramSource::File(PathBuf::from(
                    test_directory() + "test_simple/other.pile",
                )),
                expressions: vec![
                    Expr::Atom {
                        line: 1,
                        token: Token::Number(Number::Natural(100)),
                    },
                    Expr::Atom {
                        line: 1,
                        token: Token::Operator(Operator::Print),
                    },
                ],
            },
        }],
    );
}

#[test]
fn test_tree() {
    assert_resolve_eq(
        "test_tree/root.pile",
        vec![
            Expr::Use {
                line: 8,
                subprogram: Ast {
                    source: ProgramSource::File(PathBuf::from(
                        test_directory() + "test_tree/child1.pile",
                    )),
                    expressions: vec![
                        Expr::Use {
                            line: 1,
                            subprogram: Ast {
                                source: ProgramSource::File(PathBuf::from(
                                    test_directory()
                                        + "test_tree/child1_1.pile",
                                )),
                                expressions: vec![
                                    Expr::Atom {
                                        line: 1,
                                        token: Token::Number(Number::Natural(
                                            1,
                                        )),
                                    },
                                    Expr::Atom {
                                        line: 1,
                                        token: Token::Number(Number::Natural(
                                            2,
                                        )),
                                    },
                                    Expr::Atom {
                                        line: 1,
                                        token: Token::Number(Number::Natural(
                                            3,
                                        )),
                                    },
                                ],
                            },
                        },
                        Expr::Use {
                            line: 2,
                            subprogram: Ast {
                                source: ProgramSource::File(PathBuf::from(
                                    test_directory()
                                        + "test_tree/child1_2.pile",
                                )),
                                expressions: vec![],
                            },
                        },
                        Expr::Atom {
                            line: 4,
                            token: Token::String("child1".to_owned()),
                        },
                        Expr::Atom {
                            line: 4,
                            token: Token::Operator(Operator::Print),
                        },
                    ],
                },
            },
            Expr::Use {
                line: 9,
                subprogram: Ast {
                    source: ProgramSource::File(PathBuf::from(
                        test_directory() + "test_tree/child2.pile",
                    )),
                    expressions: vec![],
                },
            },
        ],
    );
}

#[test]
fn test_cycle1() {
    let relative_path = "test_cycle1/cycle1.pile";
    let absolute_path = PathBuf::from(test_directory() + relative_path);

    assert_resolve_error(
        relative_path,
        PileError::new(
            ProgramSource::File(absolute_path.clone()),
            (1, 1),
            format!(
                "Found cyclic use of '{}'.",
                absolute_path.to_string_lossy()
            ),
        ),
    )
}

#[test]
fn test_cycle2() {
    let relative_path = "test_cycle2/cycle2.pile";
    let absolute_path = PathBuf::from(test_directory() + relative_path);

    assert_resolve_error(
        relative_path,
        PileError::new(
            ProgramSource::File(PathBuf::from(
                test_directory() + "test_cycle2/c.pile",
            )),
            (1, 1),
            format!(
                "Found cyclic use of '{}'.",
                absolute_path.to_string_lossy()
            ),
        ),
    )
}

#[test]
fn test_file_not_found() {
    let relative_path = "test_not_found/root.pile";

    assert_resolve_error(
        relative_path,
        PileError::new(
            ProgramSource::File(PathBuf::from(
                test_directory() + "test_not_found/root.pile",
            )),
            (1, 1),
            format!(
                "{}: No such file or directory (os error 2)",
                test_directory() + "test_not_found/unknown.pile"
            ),
        ),
    )
}
