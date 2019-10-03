mod cli;
mod interpret;
mod lex;
mod parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program_text = match cli::read_program(&args) {
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
        Ok(program) => program,
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

    let mut interpreter = interpret::Interpreter::new(ast);
    let value = interpreter.run();

    if let Err(runtime_error) = value {
        eprintln!("{}", runtime_error);
        std::process::exit(1);
    }
}
