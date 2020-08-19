mod cli;
mod interpret;
mod lex;
mod locals;
mod parse;
mod pile_error;
mod program_source;
mod repl;
mod using;

fn main() {
    match pile() {
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
        Ok(()) => (),
    }
}

fn pile() -> Result<(), String> {
    let options = cli::read_options(std::env::args_os())?;
    let program_text = options.read_program()?;

    let lexer = lex::Lexer::new(program_text.as_ref(), options.source());
    let parser = parse::Parser::new(lexer);
    let ast = parser.parse().map_err(|e| e.to_string())?;

    let ast = locals::translate(ast);
    let ast = using::resolve(ast).map_err(|e| e.to_string())?;

    let mut interpreter =
        interpret::Interpreter::new(ast, options.stack_size(), options.trace());
    interpreter.run().map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod examples;
