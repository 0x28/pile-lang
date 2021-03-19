use crate::completion;
use crate::interpret::Interpreter;
use crate::lex::Lexer;
use crate::locals;
use crate::parse::Parser;
use crate::program_source::ProgramSource;
use crate::using;

use std::cell::{Ref, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

use rustyline::completion::{extract_word, Completer};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{CompletionType, Config, EditMode, Editor};
use rustyline_derive::Helper;

#[derive(Helper)]
struct ReplHelper<'i> {
    interpreter: Option<Ref<'i, Interpreter>>,
}

impl<'i> Validator for ReplHelper<'i> {}
impl<'i> Highlighter for ReplHelper<'i> {}
impl<'i> Hinter for ReplHelper<'i> {}
impl<'i> Completer for ReplHelper<'i> {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let (start, word) = extract_word(line, pos, None, &[b' ', b'\t']);
        if let Some(interpreter) = &self.interpreter {
            Ok((
                start,
                completion::complete_to_vec(word, u64::MAX, interpreter.ast()),
            ))
        } else {
            Ok((0, Vec::with_capacity(0)))
        }
    }
}

fn pile_history() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

    home.join(".pile_history")
}

fn create_editor<'i>() -> Editor<ReplHelper<'i>> {
    let config = Config::builder()
        .edit_mode(EditMode::Emacs)
        .completion_type(CompletionType::List)
        .build();
    let mut editor = Editor::with_config(config);
    editor.set_helper(Some(ReplHelper { interpreter: None }));
    let _ignored = editor.load_history(&pile_history());

    editor
}

fn get_line<'i>(
    editor: &mut Editor<ReplHelper<'i>>,
    interpreter: Ref<'i, Interpreter>,
) -> Result<String, i32> {
    if let Some(helper) = editor.helper_mut() {
        helper.interpreter = Some(interpreter)
    }
    let line = match editor.readline("pile > ") {
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
    };
    if let Some(helper) = editor.helper_mut() {
        helper.interpreter = None
    }

    line
}

pub fn repl() -> ! {
    let interpreter = RefCell::new(Interpreter::empty());
    let mut editor = create_editor();
    let exit_code;

    loop {
        let line = match get_line(&mut editor, interpreter.borrow()) {
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

        let ast = match using::resolve(locals::translate(ast)) {
            Ok(ast) => ast,
            Err(msg) => {
                eprintln!("{}", msg);
                continue;
            }
        };

        match interpreter.borrow_mut().eval(ast) {
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
