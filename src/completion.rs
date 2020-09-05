use crate::lex::{Operator, Token};
use crate::parse::Expr;
use crate::using::ResolvedAst;

use std::collections::HashSet;

macro_rules! ensure_token_completion {
    ($($token: pat => $tok_string: literal),+ $(,)? ) => {
        // NOTE: This unused function ensures that new tokens result in an error
        // if the arguments to ensure_token_completion aren't updated.
        #[allow(dead_code)]
        fn ensure_all_covered(tok: Token) {
            match tok {
                $($token             => {})+,
                // the following tokens can't be known at compile time
                Token::Number(_)     => {},
                Token::Identifier(_) => {},
                Token::String(_)     => {},
            }
        }
        &[$($tok_string,)+]
    }
}

fn token_completions() -> &'static [&'static str] {
    ensure_token_completion! {
        Token::Operator(Operator::If)           => "if",
        Token::Operator(Operator::Dotimes)      => "dotimes",
        Token::Operator(Operator::While)        => "while",
        Token::Operator(Operator::Plus)         => "+",
        Token::Operator(Operator::Minus)        => "-",
        Token::Operator(Operator::Div)          => "/",
        Token::Operator(Operator::Mul)          => "*",
        Token::Operator(Operator::Greater)      => ">",
        Token::Operator(Operator::GreaterEqual) => ">=",
        Token::Operator(Operator::Equal)        => "=",
        Token::Operator(Operator::LessEqual)    => "<=",
        Token::Operator(Operator::And)          => "and",
        Token::Operator(Operator::Or)           => "or",
        Token::Operator(Operator::Not)          => "not",
        Token::Operator(Operator::Less)         => "<",
        Token::Operator(Operator::Print)        => "print",
        Token::Operator(Operator::Assert)       => "assert",
        Token::Operator(Operator::Dup)          => "dup",
        Token::Operator(Operator::Drop)         => "drop",
        Token::Operator(Operator::Swap)         => "swap",
        Token::Operator(Operator::Natural)      => "natural",
        Token::Operator(Operator::Integer)      => "integer",
        Token::Operator(Operator::Float)        => "float",
        Token::Assign                           => "->",
        Token::Begin                            => "begin",
        Token::End                              => "end",
        Token::Let                              => "let",
        Token::BracketLeft                      => "[",
        Token::BracketRight                     => "]",
        Token::Boolean(true)                    => "true",
        Token::Boolean(false)                   => "false",
        Token::Use                              => "use",
    }
}

fn map_identifiers<O>(
    expressions: &[Expr],
    range: (u64, u64),
    line: u64,
    operation: &mut O,
) where
    O: FnMut(&str),
{
    for expr in expressions {
        match expr {
            Expr::Assignment {
                var,
                line: assign_line,
            } if *assign_line <= line && line <= range.1 => {
                // NOTE: from the line of the assignment until the end of the
                // current block
                operation(&var)
            }
            Expr::Save { var, .. } if range.0 <= line && line <= range.1 => {
                operation(&var)
            }
            Expr::Block {
                expressions,
                begin,
                end,
                ..
            } => map_identifiers(expressions, (*begin, *end), line, operation),
            Expr::Use {
                subprogram,
                line: use_line,
            } if *use_line <= line => {
                map_sub_identifiers(&subprogram.expressions, operation)
            }
            _ => (),
        }
    }
}

fn map_sub_identifiers<O>(expressions: &[Expr], operation: &mut O)
where
    O: FnMut(&str),
{
    for expr in expressions {
        match expr {
            Expr::Assignment { var, .. } => operation(&var),
            Expr::Block { .. } => {
                // NOTE: only top level assignments matter in used modules
            }
            Expr::Use { subprogram, .. } => {
                map_sub_identifiers(&subprogram.expressions, operation)
            }
            _ => (),
        }
    }
}

pub fn map_completions<O>(
    prefix: &str,
    line: u64,
    ast: ResolvedAst,
    operation: &mut O,
) where
    O: FnMut(&str),
{
    let mut filter = |name: &str| {
        if name.starts_with(prefix) {
            operation(name)
        }
    };

    for token in token_completions() {
        filter(token)
    }

    map_identifiers(&ast.as_ref().expressions, (0, u64::MAX), line, &mut filter)
}

pub fn complete_to_stdout(prefix: &str, line: u64, ast: ResolvedAst) {
    let mut completions = HashSet::new();

    map_completions(prefix, line, ast, &mut |name| {
        completions.insert(name.to_owned());
    });

    for name in completions {
        println!("{}", name);
    }
}

#[cfg(test)]
mod test;
