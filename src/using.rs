use crate::lex::Lexer;
use crate::parse::Ast;
use crate::parse::Expr;
use crate::parse::Parser;
use crate::pile_error::PileError;
use crate::program_source::ProgramSource;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

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

pub fn resolve(ast: Ast) -> Result<Ast, PileError> {
    let path = match ast.source.as_ref() {
        ProgramSource::File(file) => normalize_path(file).map_err(|err| {
            PileError::new(Rc::clone(&ast.source), (0, 0), err)
        })?,
        _ => PathBuf::new(),
    };

    let dir = match &ast.source.as_ref() {
        ProgramSource::Repl | ProgramSource::Stdin => PathBuf::from("."),
        ProgramSource::File(file) => file
            .parent()
            .map(&Path::to_owned)
            .unwrap_or_else(|| PathBuf::from(".")),
    };

    resolve_use(&dir, &DependencyTree::new(&path), ast)
}

fn resolve_use(
    current_dir: &PathBuf,
    tree: &DependencyTree,
    ast: Ast,
) -> Result<Ast, PileError> {
    let source = &ast.source;
    let expressions: Result<Vec<Expr>, PileError> = ast
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
                            PileError::new(
                                Rc::clone(&source),
                                (line, line),
                                msg,
                            )
                        })?;

                if tree.contains(&component_path) {
                    return Err(PileError::new(
                        Rc::clone(&source),
                        (line, line),
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

                Ok(Expr::Use { subprogram, line })
            } else {
                Ok(expr)
            }
        })
        .collect();

    Ok(Ast {
        source: Rc::clone(source),
        expressions: expressions?,
    })
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
) -> Result<Ast, PileError> {
    let program_text = fs::read_to_string(&file).map_err(|err| {
        PileError::new(
            Rc::clone(source),
            (line, line),
            format!("{}: {}", file.to_string_lossy(), err),
        )
    })?;

    let sub_source = Rc::new(ProgramSource::File(PathBuf::from(&file)));
    let lexer = Lexer::new(&program_text, Rc::clone(&sub_source));
    Parser::new(lexer).parse()
}

#[cfg(test)]
mod test;
