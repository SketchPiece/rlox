pub mod core;
pub mod helpers;
pub use core::*;

use interpreter::Interpreter;
use parser::Parser;
use reporter::log_reporter::LogReporter;
use scanner::Scanner;
use std::{
    fs,
    io::{self, Write},
    rc::Rc,
};

pub fn run_file(path: &str) {
    println!("Running file: {path}");
    let contents = fs::read_to_string(path).expect("Error reading a file");
    run(&contents);
}

pub fn run_prompt() {
    let mut line = String::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        line.clear();

        if io::stdin().read_line(&mut line).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        if line.trim().is_empty() {
            break;
        }

        run(&line);
    }
}

pub fn run(source: &str) {
    let debug_run = true;
    let log_reporter = LogReporter::new();

    let mut scanner = Scanner::new(source).attach_reporter(Rc::clone(&log_reporter));
    let tokens = scanner.scan_tokens();
    if debug_run {
        helpers::print_tokens(&tokens);
    }

    let mut parser = Parser::new(tokens).attach_reporter(Rc::clone(&log_reporter));
    if let Ok(expr) = parser.parse() {
        if debug_run {
            println!("Expression AST: {:?}", expr.stringify());
        }
        let interpreter = Interpreter::new(expr);
        interpreter.interpret();
    }
}
