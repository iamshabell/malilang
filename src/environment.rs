use std::collections::HashMap;

use crate::expr::ExpLiteralValue;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, ExpLiteralValue>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn enclosing(enclosing: Environment) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: &str, value: ExpLiteralValue) {
        self.values.insert(name.to_string(), value);
    }
    pub fn get(&self, name: &str) -> Option<&ExpLiteralValue> {
        match self.values.get(name) {
            Some(value) => {
                return Some(value);
            }
            None => match &self.enclosing {
                Some(enclosing) => enclosing.get(name),
                None => None,
            },
        }
    }

    pub fn assign(&mut self, name: &str, value: ExpLiteralValue) {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
        } else {
            match &mut self.enclosing {
                Some(enclosing) => enclosing.assign(name, value),
                None => (),
            }
        }
    }
}
