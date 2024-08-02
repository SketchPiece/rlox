use std::{cell::RefCell, rc::Rc};

use super::ErrorReporter;

// Note: learned how to use RefCell to add internal mutability despite LogReporter is used as immutable ref
#[derive(Debug, Default)]
pub struct LogReporter {
    had_error: RefCell<bool>,
    had_runtime_error: RefCell<bool>,
}

impl LogReporter {
    pub fn new() -> Rc<Self> {
        Rc::new(LogReporter::default())
    }

    pub fn is_had_error(&self) -> bool {
        *self.had_error.borrow()
    }

    pub fn is_had_runtime_error(&self) -> bool {
        *self.had_runtime_error.borrow()
    }
}

impl ErrorReporter for LogReporter {
    fn report(&self, line: usize, where_occurred: &str, message: &str) {
        eprintln!("[line {line}] Error{where_occurred}: {message}");

        // updating had_error to indicate ocurred error somewhere during interpreting
        // it can be either in scanner, parser, interpreter or anywhere else where we attach reporter
        // Note: we can use LogReporter as immutable reference anywhere despite we are mutating its inner state
        // just discovered rust is awesome
        let mut had_error = self.had_error.borrow_mut();
        *had_error = true;
    }

    fn report_runtime(&self, line: usize, message: &str) {
        eprintln!("{}\n[line {}]", message, line);

        let mut had_runtime_error = self.had_runtime_error.borrow_mut();
        *had_runtime_error = true
    }
}
