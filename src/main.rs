use std::env;
use std::fs;

mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

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

    Ok(())
}
