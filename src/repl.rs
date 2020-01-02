use crate::interpret::Interpreter;
use crate::lex::Lexer;
use crate::parse::Parser;
use crate::cli::ProgramSource;

use std::io::Write;

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
        let lexer = Lexer::new(&line, ProgramSource::Repl);
        let parser = Parser::new(lexer);

        let expr = match parser.parse() {
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