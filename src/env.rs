//! Runtime environment

use std::collections::HashMap;
use crate::value::LoxValue;
use crate::error::RloxError;
use crate::ast::token::Token;
use crate::ast::token::TokenType;

struct TableList {
    table: HashMap<String, LoxValue>,
    next: Option<Box<TableList>>,
}

pub struct Environment {
    values: TableList,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: TableList { table: HashMap::new(), next: None },
        }
    }

    pub fn enter_scope(&mut self) {
        let old_values = std::mem::replace(&mut self.values, TableList {
            table: HashMap::new(),
            next: None,
        });
        self.values.next = Some(Box::new(old_values));
    }

    pub fn exit_scope(&mut self) {
        if let Some(next) = self.values.next.take() {
            self.values = *next;
        } else {
            panic!("No scope to exit");
        }
    }

    pub fn define(&mut self, name: &str, value: LoxValue) {
        self.values.table.insert(name.to_string(), value);
    }

    fn get_helper(values: &TableList, name: &str) -> Option<LoxValue> {
        if let Some(value) = values.table.get(name) {
            return Some(value.clone());
        }
        if let Some(next) = &values.next {
            return Self::get_helper(next, name);
        }
        None
    }

    fn get_mut_helper<'a>(values: &'a mut TableList, name: &str) -> Option<&'a mut LoxValue> {
        if let Some(value) = values.table.get_mut(name) {
            return Some(value);
        }
        if let Some(next) = &mut values.next {
            return Self::get_mut_helper(next, name);
        }
        None
    }

    pub fn get(&self, name: &Token) -> Result<LoxValue, RloxError> {
        if name.t_type != TokenType::Identifier {
            return Err(RloxError::RuntimeError(name.line, "Invalid token type".to_string(), name.lexeme.clone()));
        }
        match Self::get_helper(&self.values, &name.lexeme) {
            Some(value) => Ok(value),
            None => Err(RloxError::RuntimeError(name.line, "Undefined variable".to_string(), name.lexeme.clone())),
        }
    }

    fn get_mut(&mut self, name: &Token) -> Result<&mut LoxValue, RloxError> {
        if name.t_type != TokenType::Identifier {
            return Err(RloxError::RuntimeError(name.line, "Invalid token type".to_string(), name.lexeme.clone()));
        }
        match Self::get_mut_helper(&mut self.values, &name.lexeme) {
            Some(value) => Ok(value),
            None => Err(RloxError::RuntimeError(name.line, "Undefined variable".to_string(), name.lexeme.clone())),
        }
    }
    
    pub fn assign(&mut self, name: &Token, value: LoxValue) -> Result<(), RloxError> {
        if name.t_type != TokenType::Identifier {
            return Err(RloxError::RuntimeError(name.line, "Invalid token type".to_string(), name.lexeme.clone()));
        }
        match self.get_mut(name) {
            Ok(old_value) => {
                *old_value = value;
                Ok(())
            }
            Err(e) => Err(e), 
        }
    }
}