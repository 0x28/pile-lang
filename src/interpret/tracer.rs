use super::scoping::ScopeStack;
use crate::lex::Operator;
use crate::lex::Token;
use crate::parse::Expr;
use crate::program_source::ProgramSource;

use std::fmt;

struct TracedExpr<'e>(&'e Expr);

impl<'e> fmt::Display for TracedExpr<'e> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Expr::Atom { token, .. } => write!(f, "{}", token),
            Expr::Assignment { var, .. } => write!(f, "-> {}", var),
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
            Expr::Save { var, .. } => write!(f, "save(\"{}\")", var),
            Expr::Restore { var, .. } => write!(f, "restore(\"{}\")", var),
        }
    }
}

pub fn before_eval(expr: &Expr, lookup: &ScopeStack) {
    if let Expr::Atom {
        token: Token::Identifier(ident),
        ..
    } = expr
    {
        if let Some(value) = lookup.resolve(ident) {
            println!("→ {:20} (= {})", ident, value)
        }
    } else {
        println!("→ {}", TracedExpr(expr));

        if is_print(expr) {
            println!("──── stdout ────")
        }
    }
}

pub fn after_eval(expr: &Expr) {
    if is_print(expr) {
        println!("\n────────────────")
    }
}

fn is_print(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Atom {
            token: Token::Operator(Operator::Print),
            ..
        }
    )
}

#[cfg(test)]
mod test;
