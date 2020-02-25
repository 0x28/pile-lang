use crate::program_source::ProgramSource;
use crate::lex::Operator;
use crate::lex::Token;
use crate::parse::Expr;

use std::fmt;

struct TracedExpr<'e>(&'e Expr);
struct TracedToken<'t>(&'t Token);

impl<'e> fmt::Display for TracedExpr<'e> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Expr::Atom { token, .. } => write!(f, "{}", TracedToken(token)),
            Expr::Assignment { var, .. } => {
                write!(f, "-> {}", var)
            }
            Expr::Block { expressions, .. } => {
                write!(f, "begin")?;
                for expr in expressions.iter() {
                    write!(f, " {}", TracedExpr(expr))?;
                }
                write!(f, " end")
            }
            Expr::Use { subprogram, .. } => {
                let source_file = match subprogram.source.as_ref() {
                    ProgramSource::File(path) => path.to_string_lossy(),
                    _ => panic!("impossible use statement"),
                };
                write!(f, "use \"{}\"", source_file)
            }
        }
    }
}

impl<'t> fmt::Display for TracedToken<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Token::Begin => write!(f, "begin"),
            Token::End => write!(f, "end"),
            Token::Assign => write!(f, "->"),
            Token::Operator(op) => write!(f, "{}", op),
            Token::Number(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Use => write!(f, "use"),
            Token::Boolean(b) => write!(f, "{}", b),
            Token::Identifier(i) => write!(f, "{}", i),
        }
    }
}

pub fn before_eval(expr: &Expr) {
    println!("→ {}", TracedExpr(expr));

    if is_print(expr) {
        println!("──── stdout ────")
    }
}

pub fn after_eval(expr: &Expr) {
    if is_print(expr) {
        println!("\n────────────────")
    }
}

fn is_print(expr: &Expr) -> bool {
    match expr {
        Expr::Atom {
            token: Token::Operator(Operator::Print),
            ..
        } => true,
        _ => false,
    }
}
