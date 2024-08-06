use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

use super::{ast::Value, tokens::Token};

pub enum EnvError {
    AssignUndefinedVariable,
}

#[derive(Debug, Default)]
pub struct Environment {
    enclosing: Option<Rc<Environment>>,
    values: RefCell<HashMap<String, Value>>,
}

impl Environment {
    pub fn with_enclosing(enclosing: Rc<Environment>) -> Self {
        Self {
            enclosing: Some(enclosing),
            ..Default::default()
        }
    }

    pub fn get(&self, name: &Token) -> Option<Value> {
        let values = self.values.borrow_mut();
        if let Some(value) = values.get(&name.lexeme) {
            Some(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        } else {
            None
        }
    }

    pub fn define(&self, name: String, value: Value) {
        let mut values = self.values.borrow_mut();
        values.insert(name, value);
    }

    pub fn assign(&self, name: String, value: Value) -> Result<(), EnvError> {
        let mut values = self.values.borrow_mut();
        if let Entry::Occupied(mut entry) = values.entry(name) {
            entry.insert(value);
            Ok(())
        } else {
            Err(EnvError::AssignUndefinedVariable)
        }
    }
}
