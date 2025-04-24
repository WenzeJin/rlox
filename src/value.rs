//! Implements values in lox language.

use crate::error::RloxError;
use crate::interpreter::Interpreter;
use crate::ast::stmt::Stmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LoxValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(LoxCallable),
    Null,
}

#[derive(Debug, Clone)]
pub enum LoxCallable {
    UserFunction(Vec<String>, Box<Stmt>),      // (Parameters, Body)
    BuiltInFunction(u32, fn(Vec<LoxValue>) -> Result<LoxValue, RloxError>),     // (Arity, Function)
}

impl ToString for LoxValue {
    fn to_string(&self) -> String {
        match self {
            LoxValue::Number(n) => n.to_string(),
            LoxValue::String(s) => s.clone(),
            LoxValue::Boolean(b) => b.to_string(),
            LoxValue::Null => "nil".to_string(),
            LoxValue::Callable(_) => "function".to_string(),
        }
    }
}

impl LoxCallable {
    pub fn arity (&self) -> u32 {
        match self {
            LoxCallable::UserFunction(params, _) => params.len() as u32,
            LoxCallable::BuiltInFunction(arity, _) => *arity,
        }
    }

    pub fn invoke(&self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, RloxError> {
        if (self.arity() as usize) != arguments.len() {
            panic!("Arity should be checked before invoking");
        }
        match self {
            LoxCallable::UserFunction(params, body) => {
                // invoke user defined function
                Ok(LoxValue::Null)
            },
            LoxCallable::BuiltInFunction(_, implementation) => {
                // invoke built-in function
                implementation(arguments)
            },
        }
    }
}

impl PartialEq for LoxCallable {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}