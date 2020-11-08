use crate::interpret::Interpreter;
use crate::lex::Lexer;
use crate::locals;
use crate::parse::Parser;
use crate::program_source::ProgramSource;
use crate::using;

use std::path::PathBuf;
use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::{Config, EditMode, Editor};

fn pile_history() -> PathBuf {
    let home = dirs::home_dir().unwrap_or(PathBuf::from("."));

    home.join(".pile_history")
}

fn create_editor() -> Editor<()> {
    let config = Config::builder().edit_mode(EditMode::Emacs).build();
    let mut editor = Editor::with_config(config);
    let _ignored = editor.load_history(&pile_history());

    editor
}

fn get_line(editor: &mut Editor<()>) -> Result<String, i32> {
    match editor.readline("pile > ") {
        Ok(line) => {
            editor.add_history_entry(line.as_str());
            Ok(line)
        }
        Err(ReadlineError::Eof) => Err(0),
        Err(ReadlineError::Interrupted) => Err(1),
        Err(err) => {
            eprintln!("readline failed: {:?}", err);
            Err(2)
        }
    }
}

pub fn repl() -> ! {
    let mut interpreter = Interpreter::empty();
    let mut editor = create_editor();
    let exit_code;

    loop {
        let line = match get_line(&mut editor) {
            Ok(line) => line,
            Err(code) => {
                exit_code = code;
                break;
            }
        };

        let lexer = Lexer::new(&line, Rc::new(ProgramSource::Repl));
        let parser = Parser::new(lexer);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(msg) => {
                eprintln!("{}", msg);
                continue;
            }
        };

        let expr = match using::resolve(locals::translate(ast)) {
            Ok(ast) => ast.ast().expressions,
            Err(msg) => {
                eprintln!("{}", msg);
                continue;
            }
        };

        match interpreter.eval(expr) {
            Ok(Some(value)) => println!("{}", value),
            Ok(None) => println!(),
            Err(msg) => eprintln!("{}", msg),
        };
    }

    if let Err(err) = editor.save_history(&pile_history()) {
        eprintln!(
            "Couldn't save pile history to {}: {:?}",
            pile_history().to_string_lossy(),
            err
        );
    }

    std::process::exit(exit_code);
}
