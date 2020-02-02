use crate::program_source::ProgramSource;
use crate::interpret::Interpreter;
use crate::lex::Lexer;
use crate::parse::Parser;
use crate::using;

use std::io::Write;
use std::rc::Rc;

pub fn repl() -> ! {
    let mut interpreter = Interpreter::empty();
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();

    loop {
        print!("pile > ");
        stdout.flush().expect("stdout flush failed!");
        let mut line = String::new();
        stdin.read_line(&mut line).expect("stdin read failed!");
        if line.is_empty() {
            println!();
            break;
        }
        let lexer = Lexer::new(&line, Rc::new(ProgramSource::Repl));
        let parser = Parser::new(lexer);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(msg) => {
                eprintln!("{}", msg);
                continue;
            }
        };

        let expr = match using::resolve(ast) {
            Ok(ast) => ast.expressions,
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

    std::process::exit(0);
}
