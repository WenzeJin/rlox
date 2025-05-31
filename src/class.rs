//! things about oop in lox

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::value::LoxValue;
use crate::value::LoxFunction;
use crate::error::RloxError;

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    pub super_class: Option<Rc<RefCell<LoxClass>>>,
    pub methods: HashMap<String, LoxFunction>,
}

#[derive(Debug, Clone)]
pub struct LoxInstance {
    pub class: Rc<RefCell<LoxClass>>,
    fields: HashMap<String, LoxValue>,
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        LoxClass {
            name,
            super_class: None,
            methods: HashMap::new(),
        }
    }

    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }

        if let Some(super_class) = &self.super_class {
            return super_class.borrow().find_method(name);
        }

        None
    }
}

impl ToString for LoxClass {
    fn to_string(&self) -> String {
        format!("class {}", self.name)
    }
}

impl LoxInstance {
    pub fn new(class: &Rc<RefCell<LoxClass>>) -> Self {
        LoxInstance {
            class: Rc::clone(class),
            fields: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, value: LoxValue) {
        self.fields.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str, instance: &Rc<RefCell<LoxInstance>>) -> Result<LoxValue, RloxError> {
        if let Some(value) = self.fields.get(name) {
            return Ok(value.clone())
        } 

        if let Some(method) = self.class.borrow().find_method(name) {
            return Ok(LoxValue::Callable(method.bind(Rc::clone(instance))));
        }

        Err(RloxError::RuntimeError( format!("Undefined property '{}'.", name) ))

    }
}

impl<'a> ToString for LoxInstance {
    fn to_string(&self) -> String {
        format!("{} instance", self.class.borrow().name)
    }
}