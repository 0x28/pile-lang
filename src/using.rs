use crate::lex::Lexer;
use crate::locals;
use crate::locals::ScopedAst;
use crate::parse::Ast;
use crate::parse::Expr;
use crate::parse::Parser;
use crate::pile_error::PileError;
use crate::program_source::ProgramSource;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct ResolvedAst(Ast);

impl ResolvedAst {
    #[allow(dead_code)] // used in the tests
    pub fn ast(self) -> Ast {
        self.0
    }

    pub fn repl_ast() -> ResolvedAst {
        ResolvedAst(Ast {
            source: Rc::new(ProgramSource::Repl),
            expressions: vec![],
        })
    }

    pub fn append(&mut self, mut other: ResolvedAst) {
        self.0.expressions.append(&mut other.0.expressions)
    }
}

impl AsRef<Ast> for ResolvedAst {
    fn as_ref(&self) -> &Ast {
        &self.0
    }
}

struct DependencyTree<'d> {
    parent: Option<&'d DependencyTree<'d>>,
    path: PathBuf,
}

impl<'d> DependencyTree<'d> {
    fn new(path: &PathBuf) -> Self {
        DependencyTree {
            parent: None,
            path: path.to_owned(),
        }
    }

    fn add_node(&'d self, path: &PathBuf) -> Self {
        DependencyTree {
            parent: Some(self),
            path: path.to_owned(),
        }
    }

    fn contains(&self, next_path: &PathBuf) -> bool {
        if &self.path == next_path {
            true
        } else {
            match self.parent {
                Some(parent) => parent.contains(next_path),
                None => false,
            }
        }
    }
}

pub fn resolve(ast: ScopedAst) -> Result<ResolvedAst, PileError> {
    let path = match ast.as_ref().source.as_ref() {
        ProgramSource::File(file) => normalize_path(file).map_err(|err| {
            PileError::in_file(Rc::clone(&ast.as_ref().source), err)
        })?,
        _ => PathBuf::new(),
    };

    let dir = match ast.as_ref().source.as_ref() {
        ProgramSource::Repl | ProgramSource::Stdin => PathBuf::from("."),
        ProgramSource::File(file) => file
            .parent()
            .map(&Path::to_owned)
            .unwrap_or_else(|| PathBuf::from(".")),
    };

    resolve_use(&dir, &DependencyTree::new(&path), ast)
}

fn resolve_use(
    current_dir: &Path,
    tree: &DependencyTree,
    ast: ScopedAst,
) -> Result<ResolvedAst, PileError> {
    let source = Rc::clone(&ast.as_ref().source);
    let expressions: Result<Vec<Expr>, PileError> = ast
        .ast()
        .expressions
        .into_iter()
        .map(|expr| {
            if let Expr::Use { subprogram, line } = expr {
                let component_path = match &subprogram.source.as_ref() {
                    ProgramSource::Repl | ProgramSource::Stdin => {
                        panic!("applying 'use' to stdin or repl is impossible!")
                    }
                    ProgramSource::File(file) => file,
                };

                let component_path =
                    normalize_path(&current_dir.join(&component_path))
                        .map_err(|msg| {
                            PileError::in_line(Rc::clone(&source), line, msg)
                        })?;

                if tree.contains(&component_path) {
                    return Err(PileError::in_line(
                        Rc::clone(&source),
                        line,
                        format!(
                            "Found cyclic use of '{}'.",
                            component_path.to_string_lossy()
                        ),
                    ));
                }

                let sub_ast = read_program(&component_path, &source, line)?;
                let subprogram = resolve_use(
                    &component_path
                        .parent()
                        .map(&Path::to_owned)
                        .unwrap_or_else(|| PathBuf::from(".")),
                    &tree.add_node(&component_path),
                    sub_ast,
                )?;

                Ok(Expr::Use {
                    subprogram: subprogram.0,
                    line,
                })
            } else {
                Ok(expr)
            }
        })
        .collect();

    Ok(ResolvedAst(Ast {
        source,
        expressions: expressions?,
    }))
}

fn normalize_path(file: &PathBuf) -> Result<PathBuf, String> {
    let mut file = file.to_owned();
    if file.extension() == None {
        file.set_extension("pile");
    }

    let path = file
        .canonicalize()
        .map_err(|err| format!("{}: {}", file.to_string_lossy(), err))?;

    Ok(path)
}

fn read_program(
    file: &PathBuf,
    source: &Rc<ProgramSource>,
    line: u64,
) -> Result<ScopedAst, PileError> {
    let program_text = fs::read_to_string(&file).map_err(|err| {
        PileError::in_line(
            Rc::clone(source),
            line,
            format!("{}: {}", file.to_string_lossy(), err),
        )
    })?;

    let sub_source = Rc::new(ProgramSource::File(PathBuf::from(&file)));
    let lexer = Lexer::new(&program_text, Rc::clone(&sub_source));
    Ok(locals::translate(Parser::new(lexer).parse()?))
}

#[cfg(test)]
mod test;
