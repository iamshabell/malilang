use std::collections::HashMap;

use crate::expr::ExpLiteralValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, ExpLiteralValue>,
    enclosing: Option<Box<Environment>>,
}

impl PartialOrd for Environment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.values.iter().partial_cmp(&other.values) {
            Some(std::cmp::Ordering::Equal) => match self.enclosing.partial_cmp(&other.enclosing) {
                Some(ordering) => Some(ordering),
                None => None,
            },
            Some(ordering) => Some(ordering),
            None => None,
        }
    }
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
