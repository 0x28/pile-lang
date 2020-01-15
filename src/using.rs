use crate::cli::ProgramSource;
use crate::lex::Lexer;
use crate::parse::Ast;
use crate::parse::Expr;
use crate::parse::Parser;
use crate::pile_error::PileError;

use std::fs;
use std::path::Path;
use std::path::PathBuf;

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

pub fn resolve(mut ast: Ast) -> Result<Ast, PileError> {
    let path = match &mut ast.source {
        ProgramSource::File(file) => normalize_path(file)
            .map_err(|err| PileError::new(ast.source.clone(), (0, 0), err))?,
        _ => PathBuf::new(),
    };

    let dir = match &ast.source {
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
    mut ast: Ast,
) -> Result<Ast, PileError> {
    for expr in &mut ast.expressions {
        if let Expr::Use { subprogram, line } = expr {
            let component_path = match &subprogram.source {
                ProgramSource::Repl | ProgramSource::Stdin => {
                    panic!("applying 'use' to stdin or repl is impossible!")
                }
                ProgramSource::File(file) => file,
            };
            let component_path =
                match normalize_path(&current_dir.join(&component_path)) {
                    Ok(path) => path,
                    Err(msg) => {
                        return Err(PileError::new(
                            ast.source,
                            (*line, *line),
                            msg,
                        ))
                    }
                };

            if tree.contains(&component_path) {
                return Err(PileError::new(
                    ast.source,
                    (*line, *line),
                    format!(
                        "Found cyclic use of '{}'.",
                        component_path.to_string_lossy()
                    ),
                ));
            }
            let program_text = match read_file(&component_path) {
                Ok(ast) => ast,
                Err(msg) => {
                    return Err(PileError::new(ast.source, (*line, *line), msg))
                }
            };

            let lexer = Lexer::new(
                &program_text,
                ProgramSource::File(PathBuf::from(&component_path)),
            );
            let sub_ast = Parser::new(lexer).parse()?;

            *subprogram = resolve_use(
                &component_path
                    .parent()
                    .map(&Path::to_owned)
                    .unwrap_or_else(|| PathBuf::from(".")),
                &tree.add_node(&component_path),
                sub_ast,
            )?;
            subprogram.source = ProgramSource::File(component_path);
        }
    }

    Ok(ast)
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

fn read_file(file: &PathBuf) -> Result<String, String> {
    fs::read_to_string(&file)
        .map_err(|err| format!("{}: {}", file.to_string_lossy(), err))
}

#[cfg(test)]
mod test;
