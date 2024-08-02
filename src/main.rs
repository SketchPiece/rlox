use std::cmp::Ordering;
use std::env;

use rlox::run_file;
use rlox::run_prompt;

fn main() {
    let args_amount = env::args().count() - 1;

    match args_amount.cmp(&1) {
        Ordering::Greater | Ordering::Equal => {
            let script_path = env::args().nth(1).expect("First argument must exist");
            run_file(&script_path);
            // eprintln!("Usage: rlox [script]");
            // process::exit(64)
        }
        Ordering::Less => run_prompt(),
    }
}
