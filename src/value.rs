//! Implements values in lox language.


#[derive(Debug, Clone, PartialEq)]
pub enum LoxValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}


impl ToString for LoxValue {
    fn to_string(&self) -> String {
        match self {
            LoxValue::Number(n) => n.to_string(),
            LoxValue::String(s) => s.clone(),
            LoxValue::Boolean(b) => b.to_string(),
            LoxValue::Null => "nil".to_string(),
        }
    }
}