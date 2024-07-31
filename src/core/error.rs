pub fn error(line: usize, message: &str) {
    report(line, "", message)
}

pub fn report(line: usize, where_occurred: &str, message: &str) {
    eprintln!("[line {line}] Error{where_occurred}: {message}");
    // process::exit(65)
}
