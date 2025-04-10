//! Runtime environment

use std::collections::HashMap;
use crate::value::LoxValue;
use crate::error::RloxError;
use crate::ast::token::Token;
use crate::ast::token::TokenType;

pub struct Environment {
    values: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: LoxValue) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<LoxValue, RloxError> {
        if name.t_type != TokenType::Identifier {
            return Err(RloxError::RuntimeError(name.line, "Invalid token type".to_string(), name.lexeme.clone()));
        }
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(RloxError::RuntimeError(name.line, "Undefined variable".to_string(), name.lexeme.to_string())),
        }
    }
    
    pub fn assign(&mut self, name: &Token, value: LoxValue) -> Result<(), RloxError> {
        if name.t_type != TokenType::Identifier {
            return Err(RloxError::RuntimeError(name.line, "Invalid token type".to_string(), name.lexeme.clone()));
        }
        if let Some(v) = self.values.get_mut(&name.lexeme) {
            *v = value;
            Ok(())
        } else {
            Err(RloxError::RuntimeError(name.line, "Undefined variable".to_string(), name.lexeme.to_string()))
        }
    }
}