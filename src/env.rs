//! Runtime environment


use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::value::LoxValue;
use crate::error::RloxError;
use crate::ast::token::Token;
use crate::ast::token::TokenType;

#[derive(Debug)]
pub struct EnvItem {
    table: HashMap<String, LoxValue>,
    parent: Option<Rc<RefCell<EnvItem>>>,
}

#[derive(Debug)]
pub struct Environment {
    pub values: Rc<RefCell<EnvItem>>,
    pub global: Rc<RefCell<EnvItem>>,
}

impl Environment {
    pub fn new() -> Self {
        let global = Rc::new(RefCell::new(
            EnvItem {
                table: HashMap::new(),
                parent: None,
            }
        ));
        Environment {
            global: Rc::clone(&global),
            values: Rc::clone(&global),
        }
    }

    pub fn from(global: Rc<RefCell<EnvItem>>, closure: Rc<RefCell<EnvItem>>) -> Self {
        Environment {
            global: Rc::clone(&global),
            values: Rc::clone(&closure),
        }
    }

    /// Enter a new scope, which will push a new table onto the stack. 
    pub fn enter_scope(&mut self) {
        // let curr_stack = std::mem::take(&mut self.values);
        // self.values = StackItem::Table(HashMap::new(), Box::new(curr_stack));
        self.values = Rc::new(RefCell::new(
            EnvItem {
                table: HashMap::new(),
                parent: Some(Rc::clone(&self.values)),
            }
        ));
    }

    /// Exit the current scope, which will pop the top table from the stack. <br>
    pub fn exit_scope(&mut self) {
        let parent = self.values.borrow_mut().parent.take();
        if let Some(parent) = parent {
            self.values = parent;
        } else {
            panic!("No parent scope to exit to");
        }
    }

    pub fn assign(&mut self, name: &Token, value: LoxValue) -> Result<(), RloxError> {
        if name.t_type != TokenType::Identifier {
            return Err(RloxError::RuntimeError("Invalid token type".to_string(), name.lexeme.clone()));
        }
        let name = &name.lexeme;
        let mut current = Rc::clone(&self.values);
        loop {
            if let Some(v) = current.borrow_mut().table.get_mut(name) {
                *v = value;
                return Ok(());
            }
            {
                let mut _next: Option<Rc<RefCell<EnvItem>>> = None;
                if let Some(parent) = &current.borrow_mut().parent {
                    _next = Some(Rc::clone(parent));
                } else {
                    break;
                }
                current = _next.unwrap();
            }
        }
        Err(RloxError::RuntimeError("Undefined variable".to_string(), name.clone()))
    }

    pub fn define_globally(&mut self, name: &str, value: LoxValue) {
        self.global.borrow_mut().table.insert(name.to_string(), value);
    }

    pub fn define(&mut self, name: &str, value: LoxValue) {
        self.values.borrow_mut().table.insert(name.to_string(), value);
    }

    fn get_helper(values: &Rc<RefCell<EnvItem>>, name: &str) -> Option<LoxValue> {
        let values = values.borrow();
        if let Some(value) = values.table.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &values.parent {
            Self::get_helper(parent, name)
        } else {
            None
        }
    }

    pub fn get(&self, name: &Token) -> Result<LoxValue, RloxError> {
        if name.t_type != TokenType::Identifier {
            return Err(RloxError::RuntimeError("Invalid token type".to_string(), name.lexeme.clone()));
        }
        match Self::get_helper(&self.values, &name.lexeme) {
            Some(value) => Ok(value),
            None => Err(RloxError::RuntimeError("Undefined variable".to_string(), name.lexeme.clone())),
        }
    }
    
    
}