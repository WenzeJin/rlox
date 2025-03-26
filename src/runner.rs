use std::fs::File;
use std::io::{self, Read, Write};
use crate::error::RloxError;
use crate::scanner::Scanner;
use crate::parser::Parser;
use crate::ast::*;

pub fn run_file(filename: &str) -> Result<(), RloxError> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    run_tree_walk(contents)?;
    Ok(())
}

pub fn run_prompt() -> Result<(), RloxError> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();
    loop {
        print!("> ");
        stdout.flush()?;
        buffer.clear();
        stdin.read_line(&mut buffer)?;
        run_tree_walk(buffer.clone())?;
    }
}

fn run_tree_walk(source: String) -> Result<(), RloxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens);
    let expression = parser.parse();
    let mut printer = pretty_printer::AstPrinter();
    println!("{}", expression.accept(&mut printer));
    Ok(())
}