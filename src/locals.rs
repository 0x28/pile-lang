use crate::parse::Expr;
use crate::parse::{Ast, ParsedAst};

use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct ScopedAst(Ast);

impl ScopedAst {
    pub fn ast(self) -> Ast {
        self.0
    }
}

impl AsRef<Ast> for ScopedAst {
    fn as_ref(&self) -> &Ast {
        &self.0
    }
}

pub fn translate(ast: ParsedAst) -> ScopedAst {
    let ast = ast.ast();
    let source = Rc::clone(&ast.source);
    let expressions: Vec<Expr> = ast
        .expressions
        .into_iter()
        .map(|expr| {
            if let Expr::Block {
                begin,
                end,
                locals,
                expressions,
            } = expr
            {
                let expressions =
                    translate_block(locals, expressions, begin, end);

                Expr::Block {
                    begin,
                    end,
                    expressions,
                    locals: vec![],
                }
            } else {
                expr
            }
        })
        .collect();

    ScopedAst(Ast {
        source,
        expressions,
    })
}

fn translate_block(
    locals: Vec<String>,
    expr: Rc<Vec<Expr>>,
    begin: u64,
    end: u64,
) -> Rc<Vec<Expr>> {
    let mut expr = Rc::try_unwrap(expr)
        .expect("references to expressions while translating scope");

    for var in locals.into_iter().rev() {
        expr.insert(
            0,
            Expr::Save {
                line: begin,
                var: var.to_owned(),
            },
        );
        expr.push(Expr::Restore { line: end, var });
    }

    Rc::new(expr)
}
