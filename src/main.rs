mod interpret;
mod lex;
mod parse;

use std::fs;
use std::io::{self, Read};
use std::process;

fn read_program(args: &[String]) -> String {
    if args.len() == 1 {
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Err(error) => {
                eprintln!("stdin: {}", error);
                std::process::exit(1);
            }
            Ok(contents) => contents,
        };
        buffer
    } else if args.len() == 2 {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Err(error) => {
                eprintln!("{}: {}", filename, error);
                std::process::exit(1);
            }
            Ok(contents) => contents,
        }
    } else {
        eprintln!("Usage: {} [FILE]", args[0]);
        process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program_text = read_program(&args);

    let lexer = lex::Lexer::new(program_text.as_ref());
    let parser = parse::Parser::new(lexer);
    let ast = match parser.parse() {
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1)
        }
        Ok(ast) => ast,
    };

    let mut interpreter = interpret::Interpreter::new(ast);
    let value = interpreter.run();
}
