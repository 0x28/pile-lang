use super::*;
use crate::lex::Lexer;
use crate::locals;
use crate::parse::Parser;
use crate::program_source::ProgramSource;
use crate::using::{self, ResolvedAst};

use std::rc::Rc;

fn parse_prog(text: &str) -> ResolvedAst {
    let ast =
        Parser::new(Lexer::new(text, Rc::new(ProgramSource::Stdin))).parse();
    let ast = using::resolve(locals::translate(ast.unwrap()));
    ast.unwrap()
}

#[test]
fn test_comp_global() {
    let mut comps = vec![];
    let ast = parse_prog(
        "
100 -> var1
# --- current line ---
let [x y z]
  + x z * y
end
",
    );

    map_identifiers(&ast.as_ast().expressions, (0, u64::MAX), 3, &mut |ident| {
        comps.push(ident.to_owned())
    });

    assert_eq!(comps, vec!["var1"])
}

#[test]
fn test_comp_local() {
    let mut comps = vec![];
    let ast = parse_prog(
        "
100 -> var1
let [a b c]
  + a b * c
  # --- current line ---
end
",
    );

    map_identifiers(&ast.as_ast().expressions, (0, u64::MAX), 5, &mut |ident| {
        comps.push(ident.to_owned())
    });

    assert_eq!(comps, vec!["var1", "a", "b", "c"])
}

#[test]
fn test_comp_none() {
    let mut comps: Vec<String> = vec![];
    let ast = parse_prog(
        "
# --- current line ---
1 2 + print
",
    );

    map_identifiers(&ast.as_ast().expressions, (0, u64::MAX), 2, &mut |ident| {
        comps.push(ident.to_owned())
    });

    assert!(comps.is_empty())
}

#[test]
fn test_comp_use() {
    let mut comps: Vec<String> = vec![];
    let ast = parse_prog(
        "
use \"src/completion/comp_test\"
# --- current line ---
1 2 + print
",
    );

    map_identifiers(&ast.as_ast().expressions, (0, u64::MAX), 3, &mut |ident| {
        comps.push(ident.to_owned())
    });

    assert_eq!(comps, vec!["inc", "dec"])
}

#[test]
fn test_comp_use_empty() {
    let mut comps: Vec<String> = vec![];
    let ast = parse_prog(
        "
# --- current line ---
use \"src/completion/comp_test\"
1 2 + print
",
    );

    map_identifiers(&ast.as_ast().expressions, (0, u64::MAX), 2, &mut |ident| {
        comps.push(ident.to_owned())
    });

    assert!(comps.is_empty())
}

#[test]
fn test_comp_local_assign_empty() {
    let mut comps: Vec<String> = vec![];
    let ast = parse_prog(
        "
let [a1]
  1 -> a1
end
# --- current line ---
",
    );

    map_identifiers(&ast.as_ast().expressions, (0, u64::MAX), 6, &mut |ident| {
        comps.push(ident.to_owned())
    });

    assert!(comps.is_empty())
}

#[test]
fn test_comp_local_assign() {
    let mut comps: Vec<String> = vec![];
    let ast = parse_prog(
        "
begin
  1 -> a1
  # --- current line ---
end
",
    );

    map_identifiers(&ast.as_ast().expressions, (0, u64::MAX), 4, &mut |ident| {
        comps.push(ident.to_owned())
    });

    assert_eq!(comps, vec!["a1"])
}

#[test]
fn test_comp_prefix1() {
    let mut comps = vec![];
    let ast = parse_prog(
        "
10 -> var1
20 -> var2
# --- current line ---
30 -> var3
",
    );
    map_completions("var", 4, ast, &mut |name| comps.push(name.to_owned()));
    assert_eq!(comps, vec!["var1", "var2"])
}

#[test]
fn test_comp_prefix2() {
    let mut comps = vec![];
    let ast = parse_prog("");

    map_completions("pri", 1, ast, &mut |name| comps.push(name.to_owned()));
    assert_eq!(comps, vec!["print"])
}

#[test]
fn test_comp_prefix3() {
    let mut comps = vec![];
    let ast = parse_prog("");

    map_completions("l", 1, ast, &mut |name| comps.push(name.to_owned()));
    assert_eq!(comps, vec!["let"])
}

#[test]
fn test_comp_prefix4() {
    let mut comps = vec![];
    let ast = parse_prog("");

    map_completions("f", 1, ast, &mut |name| comps.push(name.to_owned()));
    assert_eq!(comps, vec!["float", "false"])
}
