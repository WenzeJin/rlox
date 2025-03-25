use std::io;

#[derive(Debug)]
pub enum RloxError {
    IOError(io::Error),
    LexicalError(i32, String, String),  // line, message, near
    SyntaxError(i32, String, String),   // line, message, near
    RuntimeError(i32, String, String),  // line, message, near
}

impl From<io::Error> for RloxError {
    fn from(error: io::Error) -> Self {
        RloxError::IOError(error)
    }
}