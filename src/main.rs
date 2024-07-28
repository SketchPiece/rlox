use std::env;
use std::fs;
use std::process;

use asp::print_asp_main;
use scanner::Scanner;

mod asp;
mod error;
mod expr;
mod helpers;
mod scanner;
mod tokens;

fn main() {
    let script_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: rlox [script]");
        process::exit(64)
    });
    run_file(&script_path);
    print_asp_main();
}

fn run_file(path: &str) {
    println!("run file: {path}");
    let contents = fs::read_to_string(path).expect("Error reading a file");
    run(&contents);
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens.iter() {
        println!("{:?} ", token)
    }
}
