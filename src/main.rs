use std::env;
use std::fs;

mod ast;
mod lexer;
mod parser;
mod semantic_analyzer;

use lexer::Lexer;
use parser::Parser;
use semantic_analyzer::{SemanticAnalyzer, SemanticError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [--ast] <file>", args[0]);
        return Ok(());
    }

    let print_ast_flag = args.contains(&"--ast".to_string());
    let file_path = args.last().unwrap();

    let content = fs::read_to_string(file_path)?;

    let lexer = Lexer::new(content);
    let tokens: Vec<_> = lexer.collect();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    if print_ast_flag {
        println!("{:?}", &ast);
    }

    match SemanticAnalyzer::analyze(&ast) {
        Ok(()) => {}
        Err(errors) => {
            eprintln!("Semantic analysis failed:");
            for error in errors {
                match error {
                    SemanticError::UndeclaredVariable(name) => {
                        eprintln!("  Error: Use of undeclared variable '{}'", name);
                    }
                }
            }
            std::process::exit(1);
        }
    }

    Ok(())
}
