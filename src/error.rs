use core::fmt;
use std::io;
use crate::value::LoxValue;


#[derive(Debug)]
pub enum RloxError {
    IOError(io::Error),
    LexicalError(usize, String, String),  // line, message, near
    SyntaxError(usize, String, String),   // line, message, near
    RuntimeError(String),  // message
    SemanticError(String),  // message
    ReturnValue(LoxValue),  // return value, which is not an error actually
}

impl From<io::Error> for RloxError {
    fn from(error: io::Error) -> Self {
        RloxError::IOError(error)
    }
}

impl fmt::Display for RloxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RloxError::IOError(e) => write!(f, "IO Error: {}", e),
            RloxError::LexicalError(_line, message, near) => write!(f, "Error at '{}': {}.", near, message),
            RloxError::SyntaxError(_line, message, near) => write!(f, "Error: at '{}': {}.", near, message),
            RloxError::RuntimeError(message) => write!(f, "RuntimeError: {}", message),
            RloxError::SemanticError(message) => write!(f, "Error: {}", message),
            RloxError::ReturnValue(_) => write!(f, "Uncaught return value."),
        }
    }
}

pub fn report(e: &RloxError) {
    println!("{}", e);
}