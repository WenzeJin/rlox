//! Implements values in lox language.

use crate::error::RloxError;
use crate::interpreter::Interpreter;
use crate::ast::stmt::Stmt;
use crate::class::{LoxClass, LoxInstance};
use std::cell::RefCell;
use std::rc::Rc;
use crate::env::{EnvItem, Environment};

#[derive(Debug, Clone)]
pub enum LoxValue {
    Number(f64),
    Class(Rc<RefCell<LoxClass>>),
    String(String),
    Boolean(bool),
    Callable(LoxFunction),
    Instance(Rc<RefCell<LoxInstance>>),
    Null,
}

impl PartialEq for LoxValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LoxValue::Number(a), LoxValue::Number(b)) => a == b,
            (LoxValue::String(a), LoxValue::String(b)) => a == b,
            (LoxValue::Boolean(a), LoxValue::Boolean(b)) => a == b,
            (LoxValue::Null, LoxValue::Null) => true,
            (LoxValue::Instance(a), LoxValue::Instance(b)) => Rc::ptr_eq(&a, &b),
            _ => false,
        }
    }
}


#[derive(Debug, Clone)]
pub enum LoxFunction {
    BuiltInFunction(u32, fn(Vec<LoxValue>) -> Result<LoxValue, RloxError>),     // (Arity, Function)
    UserFunction {
        def_name: String,
        params: Vec<String>,
        body: Rc<Vec<Stmt>>,
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
            LoxValue::Callable(f) => f.to_string(),
            LoxValue::Instance(i) => i.borrow().to_string(),
            LoxValue::Class(c) => c.borrow().to_string(),
        }
    }
}



impl ToString for LoxFunction {
    fn to_string(&self) -> String {
        match self {
            LoxFunction::BuiltInFunction(_, _) => "<native fn>".to_string(),
            LoxFunction::UserFunction{def_name, ..} => format!("<fn {}>", def_name),
        }
    }
}

impl LoxFunction {
    pub fn arity (&self) -> u32 {
        match self {
            LoxFunction::UserFunction{params, .. } => params.len() as u32,
            LoxFunction::BuiltInFunction(arity, _) => *arity,
        }
    }

    pub fn invoke(&self, interpreter: &mut Interpreter, arguments: Vec<LoxValue>) -> Result<LoxValue, RloxError> {
        if (self.arity() as usize) != arguments.len() {
            panic!("Arity should be checked before invoking");
        }
        match self {
            LoxFunction::UserFunction{ params, body, closure, .. } => {
                // create a new environment for the function call
                let global = interpreter.env.global.clone();
                let closure = closure.clone();
                let env = Environment::from(global, closure);
                let old_env = interpreter.change_env(env);
                // enter a new scope
                interpreter.env.enter_scope();
                // bind parameters to arguments
                for (param, arg) in params.iter().zip(arguments.iter()) {
                    interpreter.env.define(param, arg.clone());
                }
                // evaluate the function body
                let result = interpreter.execute_block(body);
                // exit the scope
                interpreter.env.exit_scope();
                interpreter.change_env(old_env);
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
            LoxFunction::BuiltInFunction(_, implementation) => {
                // invoke built-in function
                implementation(arguments)
            },
        }
    }

    pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> LoxFunction {
        match self {
            LoxFunction::UserFunction { def_name, params, body, closure } => {
                // eprintln!("old closure: {:?}", closure);
                let mut new_closure = EnvItem::from_parent(Rc::clone(&closure));
                new_closure.table.insert("this".to_string(), LoxValue::Instance(instance));
                LoxFunction::UserFunction {
                    def_name: def_name.clone(),
                    params: params.clone(),
                    body: Rc::clone(body),
                    closure: Rc::new(RefCell::new(new_closure)),
                }
            }
            _ => panic!("Cannot bind a built-in function"),
        }
    }
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

