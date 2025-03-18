//! Chalk Command Line Tool

use std::{
    env,
    io::{Write, stdin},
};

use chalk_core::{ast::Parser, tokenizer::Tokenizable};

fn eval_statement(statement: &str) -> Option<f32> {
    let tokens = statement.tokenize();

    if tokens.is_err() {
        println!(
            "The provided statement is invalid Chalk format, please only use mathematical notation"
        );
        return None;
    }

    let tokens = tokens.unwrap();
    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    if ast.is_err() {
        println!(
            "The provided statement is invalid Chalk format, please only use mathematical notation"
        );
        return None;
    }

    let ast = ast.unwrap();
    Some(ast.eval())
}

fn main() {
    let statement = env::args().skip(1).collect::<String>();

    if !statement.is_empty() {
        if let Some(val) = eval_statement(&statement) {
            println!("{statement} = {val}");
        }

        return;
    }

    println!("Welcome to the Chalk Repl");
    loop {
        print!("--> ");
        let _ = std::io::stdout().flush();
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read StdIn");

        let statement = buf.trim();

        if let Some(val) = eval_statement(statement) {
            println!("{val}");
        }
    }
}
