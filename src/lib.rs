pub mod core;
pub mod helpers;
pub use core::*;

use interpreter::Interpreter;
use parser::Parser;
use reporter::log_reporter::LogReporter;
use scanner::Scanner;
use std::{
    env::args,
    fs,
    io::{self, Write},
    process,
    rc::Rc,
};

pub fn run_file(path: &str) {
    if is_debug_run() {
        println!("Running file: {path}");
    }
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
    let debug_run = is_debug_run();
    let log_reporter = LogReporter::new();

    let mut scanner = Scanner::new(source).attach_reporter(Rc::clone(&log_reporter));
    let tokens = scanner.scan_tokens();

    if debug_run {
        println!();
        helpers::print_tokens(&tokens);
    }

    let mut parser = Parser::new(tokens).attach_reporter(Rc::clone(&log_reporter));
    let statements = parser.parse();
    if debug_run {
        println!();
        helpers::print_statements(&statements, 0);

        println!();
        println!("Execution result:");
    }

    if log_reporter.is_had_error() {
        process::exit(65);
    }

    let mut interpreter = Interpreter::new().attach_reporter(Rc::clone(&log_reporter));
    interpreter.interpret(&statements);

    if log_reporter.is_had_runtime_error() {
        process::exit(70)
    }
}

fn is_debug_run() -> bool {
    args().any(|arg| arg == "--debug" || arg == "-d")
}
