use core::fmt;
use std::io;
use crate::value::LoxValue;


#[derive(Debug)]
pub enum RloxError {
    IOError(io::Error),
    LexicalError(usize, String, String),  // line, message, near
    SyntaxError(usize, String, String),   // line, message, near
    RuntimeError(String, String),  // line, message, near
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
            RloxError::LexicalError(line, message, near) => write!(f, "[line {}] Error: {} at '{}'", line, message, near),
            RloxError::SyntaxError(line, message, near) => write!(f, "[line {}] Error: {} at '{}'", line, message, near),
            RloxError::RuntimeError(message, near) => write!(f, "RuntimeError: {} near '{}'", message, near),
            RloxError::ReturnValue(_) => write!(f, "Uncaught return value."),
        }
    }
}

pub fn report(e: &RloxError) {
    eprintln!("{}", e);
}