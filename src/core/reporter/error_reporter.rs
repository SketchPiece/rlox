pub trait ErrorReporter {
    fn report(&self, line: usize, where_occurred: &str, message: &str);
    fn report_runtime(&self, line: usize, message: &str);
}
