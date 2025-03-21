//! Chalk Command Line Tool

use std::{
    env,
    io::{Write, stdin},
};

use chalk_core::{
    ast::{Expr, Parser},
    exec::Evaluator,
    tokenizer::Tokenizable,
};

/// Evaluates a statement as a Chalk AST
fn eval_statement(statement: &str) -> Option<Expr> {
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
    Some(ast)
}

fn main() {
    let statement = env::args().skip(1).collect::<String>();
    let mut executor = Evaluator::default();

    if !statement.is_empty() {
        if let Some(val) = eval_statement(&statement) {
            if let Ok(eval) = executor.exec(&val) {
                println!("`{val}` = {eval}");
            } else {
                panic!("Runtime error has occurred on expression `{val}`")
            }
        }

        return;
    }

    println!("Welcome to the Chalk Repl\n");
    loop {
        print!("->> ");
        let _ = std::io::stdout().flush();
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read StdIn");

        let statement = buf.trim();

        if let Some(val) = eval_statement(statement) {
            if let Ok(eval) = executor.exec(&val) {
                println!("`{val}` = {eval}\n");
            } else {
                println!("Runtime error has occurred on expression `{val}`")
            }
        }
    }
}
