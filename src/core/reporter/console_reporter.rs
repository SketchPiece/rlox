use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use super::ErrorReporter;

// Note: learned how to use RefCell to add internal mutability despite LogReporter is used as immutable ref
#[derive(Debug, Default)]
pub struct ConsoleReporter {
    had_error: Cell<bool>,
    had_runtime_error: Cell<bool>,
}

impl ConsoleReporter {
    pub fn new() -> Rc<Self> {
        Rc::new(ConsoleReporter::default())
    }

    pub fn is_had_error(&self) -> bool {
        self.had_error.get()
    }

    pub fn is_had_runtime_error(&self) -> bool {
        self.had_runtime_error.get()
    }
}

impl ErrorReporter for ConsoleReporter {
    fn report(&self, line: usize, where_occurred: &str, message: &str) {
        eprintln!("[line {line}] Error{where_occurred}: {message}");

        // updating had_error to indicate ocurred error somewhere during interpreting
        // it can be either in scanner, parser, interpreter or anywhere else where we attach reporter
        // Note: we can use LogReporter as immutable reference anywhere despite we are mutating its inner state
        // just discovered rust is awesome
        // Update: changed RefCell to Cell because bool implements Copy trait and its more performant
        // to use Cell here, I suppose
        self.had_error.set(true);
    }

    fn report_runtime(&self, line: usize, message: &str) {
        eprintln!("{}\n[line {}]", message, line);

        self.had_runtime_error.set(true);
    }
}
