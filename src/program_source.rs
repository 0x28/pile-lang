use std::path::PathBuf;

#[derive(PartialEq, Debug, Clone)]
pub enum ProgramSource {
    Repl,
    Stdin,
    File(PathBuf),
}
