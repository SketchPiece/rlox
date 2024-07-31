pub mod helpers;
pub mod interpreter;
pub use interpreter::*;

use parser::Parser;
use scanner::Scanner;
use std::fs;

pub fn run_file(path: &str) {
    println!("run file: {path}");
    let contents = fs::read_to_string(path).expect("Error reading a file");
    run(&contents);
}

pub fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    helpers::print_tokens(&tokens);
    let mut parser = Parser::new(tokens);
    if let Ok(expr) = parser.parse() {
        println!("{:?}", expr.accept())
    }
}
