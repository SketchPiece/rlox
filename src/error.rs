pub fn error(line: usize, message: &str) {
    report(line, "", message)
}

pub fn report(line: usize, where_occured: &str, message: &str) {
    eprintln!("[line {line}] Error{where_occured}: {message}");
    // process::exit(65)
}
