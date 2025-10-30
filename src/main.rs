use std::env;
use std::fs;

mod ast;
mod code_generator;
mod lexer;
mod parser;
mod semantic_analyzer;

use code_generator::generate_c_code;
use lexer::Lexer;
use parser::Parser;
use semantic_analyzer::{SemanticAnalyzer, SemanticError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [--ast] [--stdout] <file>", args[0]);
        return Ok(());
    }
    let print_ast_flag = args.contains(&"--ast".to_string());
    let stdout_flag = args.contains(&"--stdout".to_string());
    let file_path = args.last().unwrap();
    let content = fs::read_to_string(file_path)?;

    // Tokenize
    let lexer = Lexer::new(content);
    let tokens: Vec<_> = lexer.collect();

    // Syntax analysis
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    if print_ast_flag {
        println!("{:?}", &ast);
    }

    // Semantic analysis
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

    // Code generation
    let generated_code = generate_c_code(&ast);
    if stdout_flag {
        println!("{}", generated_code);
    } else {
        let output_path = if file_path.ends_with(".hand") {
            file_path.replace(".hand", ".c")
        } else {
            format!("{}.c", file_path)
        };

        fs::write(&output_path, generated_code)?;
        eprintln!("Generated C code written to: {}", output_path);
    }
    Ok(())
}
