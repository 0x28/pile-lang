use crate::interpret::Interpreter;
use crate::lex::Lexer;
use crate::locals;
use crate::parse::Parser;
use crate::program_source::ProgramSource;
use crate::using;

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;

fn run_example_file(path: PathBuf) {
    let program = fs::read_to_string(&path).unwrap();
    let lexer =
        Lexer::new(program.as_ref(), Rc::new(ProgramSource::File(path)));
    let parser = Parser::new(lexer);
    let ast = parser.parse().unwrap();
    let ast = locals::translate(ast);
    let ast = using::resolve(ast).unwrap();

    let mut interpreter = Interpreter::new(ast, 100, false);
    interpreter.run().expect("Test program failed!");
}

#[test]
fn run_example_files() {
    let example_dir = env!("CARGO_MANIFEST_DIR").to_owned() + "/src/examples/";
    let files = fs::read_dir(example_dir).unwrap();

    files
        .map(Result::unwrap)
        .map(|file| file.path())
        .filter(|path| path.extension().unwrap_or_default() == "pile")
        .map(run_example_file)
        .for_each(drop);
}

#[test]
#[cfg(not(tarpaulin))] // NOTE: causes a timeout for tarpaulin!
fn test_quine() {
    let quine_file =
        env!("CARGO_MANIFEST_DIR").to_owned() + "/src/examples/quine.pile";

    let output = Command::new("cargo")
        .args(&["run", "--", &quine_file])
        .output()
        .expect("Couldn't run quine test!");

    let output = String::from_utf8_lossy(&output.stdout);

    assert_eq!(output, fs::read_to_string(quine_file).unwrap());
}
