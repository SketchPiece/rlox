use std::env;
use std::process;

use rlox::run_file;

fn main() {
    let script_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: rlox [script]");
        process::exit(64)
    });
    run_file(&script_path);
}
