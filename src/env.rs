//! Runtime environment

use std::collections::HashMap;
use crate::value::LoxValue;
use crate::error::RloxError;

pub struct Environment {
    values: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<LoxValue, RloxError> {
        match self.values.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(RloxError::RuntimeError(0, "Undefined variable".to_string(), name.to_string())),
        }
    }
        
}