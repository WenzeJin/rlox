//! Implements values in lox language.

use crate::error::RloxError;
use crate::interpreter::Interpreter;
use crate::ast::stmt::Stmt;
use std::cell::RefCell;
use std::rc::Rc;
use crate::env::{EnvItem, Environment};

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
    BuiltInFunction(u32, fn(Vec<LoxValue>) -> Result<LoxValue, RloxError>),     // (Arity, Function)
    UserFunction {
        def_name: String,
        params: Vec<String>,
        body: Rc<Stmt>,
        closure: Rc<RefCell<EnvItem>>,  // Environment of this function
    },
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
            LoxCallable::UserFunction { params, .. } => params.len() as u32,
            LoxCallable::BuiltInFunction(arity, _) => *arity,
        }
    }

    pub fn invoke(&self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, RloxError> {
        if (self.arity() as usize) != arguments.len() {
            panic!("Arity should be checked before invoking");
        }
        match self {
            LoxCallable::UserFunction { def_name, params, body, closure } => {
                // create a new environment for the function call
                let global = interpreter.env.global.clone();
                let closure = closure.clone();
                let env = Environment::from(global, closure);
                let mut interpreter = Interpreter::from_env(env);
                // enter a new scope
                interpreter.env.enter_scope();
                // bind parameters to arguments
                for (param, arg) in params.iter().zip(arguments.iter()) {
                    interpreter.env.define(param, arg.clone());
                }
                // evaluate the function body
                let result = body.accept(&mut interpreter);
                // exit the scope
                interpreter.env.exit_scope();
                // return the result
                // if the result is a return statement, return the value
                match result {
                    Ok(_) => {
                        Ok(LoxValue::Null)
                    },
                    Err(RloxError::ReturnValue(value)) => {
                        // return the value
                        Ok(value)
                    },
                    Err(e) => {
                        // return the error
                        Err(e)
                    }
                }
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