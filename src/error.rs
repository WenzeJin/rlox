use core::fmt;
use std::io;
use crate::ast::token::{Token, TokenType};

#[derive(Debug)]
pub enum RloxError {
    IOError(io::Error),
    LexicalError(usize, String, String),  // line, message, near
    SyntaxError(usize, String, String),   // line, message, near
    RuntimeError(usize, String, String),  // line, message, near
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
            RloxError::RuntimeError(line, message, near) => write!(f, "[line {}] Error: {} at '{}'", line, message, near),
        }
    }
}

pub fn report(e: &RloxError) {
    eprintln!("{}", e);
}

pub fn report_line(line: usize, place: &str, msg: &str) {
    eprintln!("[line {}] Error{}: {}", line, place, msg);
}

pub fn report_token(token: &Token, msg: &str) {
    if token.t_type == TokenType::EOF {
        report_line(token.line, " at end", msg);
    } else {
        report_line(token.line, &format!(" at '{}'", token.lexeme), msg);
    }
}