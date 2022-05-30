mod cli;
mod completion;
mod formatting;
mod interpret;
mod lex;
mod locals;
mod parse;
mod pile_error;
mod program_source;
mod repl;
mod using;

fn main() {
    if let Err(msg) = pile() {
        eprintln!("{}", msg);
        std::process::exit(1);
    }
}

fn pile() -> Result<(), String> {
    let options = cli::read_options(std::env::args_os())?;
    let program_text = options.read_program()?;

    let lexer = lex::Lexer::new(program_text.as_ref(), options.source());

    if options.format() {
        formatting::format(lexer).map_err(|e| e.to_string())?;
        return Ok(());
    }

    let parser = parse::Parser::new(lexer);
    let ast = parser.parse().map_err(|e| e.to_string())?;

    let ast = locals::translate(ast);
    let ast = using::resolve(ast).map_err(|e| e.to_string())?;

    match options.completion() {
        None => {
            let mut interpreter = interpret::Interpreter::new(
                ast,
                options.stack_size(),
                options.trace(),
            );
            interpreter.run().map_err(|e| e.to_string())?;
        }
        Some(cli::CompletionOptions { prefix, line }) => {
            completion::complete_to_stdout(prefix, *line, &ast)
        }
    }

    Ok(())
}

#[cfg(test)]
mod examples;
