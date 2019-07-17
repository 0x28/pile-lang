mod lex;
mod parse;

use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} FILE", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let program_text = match fs::read_to_string(filename) {
        Err(error) => {
            eprintln!("{}: {}", filename, error);
            std::process::exit(1);
        }
        Ok(contents) => contents,
    };

    let lexer = lex::Lexer::new(program_text.as_ref());
    let parser = parse::Parser::new(lexer);
    let ast = match parser.parse() {
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1)
        }
        Ok(ast) => ast,
    };
}
