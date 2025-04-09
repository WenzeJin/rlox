
use crate::ast::expr;
use crate::value::{self, LoxValue};
use crate::ast::token::{Token, TokenType};
use crate::error::RloxError;

pub struct Interpreter {

}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }
}

impl Interpreter {

    fn is_truthy(value: &LoxValue) -> bool {
        match value {
            LoxValue::Boolean(b) => *b,
            LoxValue::Null => false,
            _ => true,
        }
    }
}

impl expr::Visitor<Result<LoxValue, RloxError>> for Interpreter {
    fn visit_literal_expr(&mut self, value: &expr::LiteralValue) -> Result<LoxValue, RloxError> {
        match value {
            expr::LiteralValue::Number(n) => Ok(LoxValue::Number(*n)),
            expr::LiteralValue::String(s) => Ok(LoxValue::String(s.clone())),
            expr::LiteralValue::Boolean(b) => Ok(LoxValue::Boolean(*b)),
            expr::LiteralValue::Nil => Ok(LoxValue::Null),
        }
    }

    fn visit_grouping_expr(&mut self, expression: &expr::Expr) -> Result<LoxValue, RloxError> {
        expression.accept(self)
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &expr::Expr) -> Result<LoxValue, RloxError> {
        let rv = right.accept(self)?;
        match operator.t_type {
            TokenType::Minus => {
                if let LoxValue::Number(n) = rv {
                    Ok(LoxValue::Number(-n))
                } else {
                    Err(RloxError::RuntimeError(operator.line, "Operand must be a number".to_string(), operator.lexeme.clone()))
                }
            }
            TokenType::Bang => {
                Ok(LoxValue::Boolean(!Interpreter::is_truthy(&rv)))
            }
            _ => Err(RloxError::RuntimeError(operator.line, "Unknown unary operator".to_string(), operator.lexeme.clone())),
        }
        
    }

    fn visit_binary_expr(&mut self, left: &expr::Expr, operator: &Token, right: &expr::Expr) -> Result<LoxValue, RloxError> {
        let lv = left.accept(self)?;
        let rv = right.accept(self)?;

        match operator.t_type {
            TokenType::Plus => {
                match (lv, rv) {
                    (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l + r)),
                    (LoxValue::String(l), LoxValue::String(r)) => Ok(LoxValue::String(format!("{l}{r}"))),
                    _ => Err(RloxError::RuntimeError(operator.line, "Operands must be two numbers or two strings".to_string(), operator.lexeme.clone()))
                }
            }
            TokenType::Minus => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    Ok(LoxValue::Number(l - r))
                } else {
                    Err(RloxError::RuntimeError(operator.line, "Operands must be numbers".to_string(), operator.lexeme.clone()))
                }
            }
            TokenType::Star => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    Ok(LoxValue::Number(l * r))
                } else {
                    Err(RloxError::RuntimeError(operator.line, "Operands must be numbers".to_string(), operator.lexeme.clone()))
                }
            }
            TokenType::Slash => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    if r == 0.0 {
                        Err(RloxError::RuntimeError(operator.line, "Division by zero".to_string(), operator.lexeme.clone()))
                    } else {
                        Ok(LoxValue::Number(l / r))
                    }
                } else {
                    Err(RloxError::RuntimeError(operator.line, "Operands must be numbers".to_string(), operator.lexeme.clone()))
                }
            }
            TokenType::Greater => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    Ok(LoxValue::Boolean(l > r))
                } else {
                    Err(RloxError::RuntimeError(operator.line, "Operands must be numbers".to_string(), operator.lexeme.clone()))
                }
            }
            TokenType::GreaterEqual => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    Ok(LoxValue::Boolean(l >= r))
                } else {
                    Err(RloxError::RuntimeError(operator.line, "Operands must be numbers".to_string(), operator.lexeme.clone()))
                }
            }
            TokenType::Less => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    Ok(LoxValue::Boolean(l < r))
                } else {
                    Err(RloxError::RuntimeError(operator.line, "Operands must be numbers".to_string(), operator.lexeme.clone()))
                }
            }
            TokenType::LessEqual => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (lv, rv) {
                    Ok(LoxValue::Boolean(l <= r))
                } else {
                    Err(RloxError::RuntimeError(operator.line, "Operands must be numbers".to_string(), operator.lexeme.clone()))
                }
            }
            TokenType::EqualEqual => 
                Ok(LoxValue::Boolean(lv == rv)),
            TokenType::BangEqual => 
                Ok(LoxValue::Boolean(lv != rv)),
            _ => Err(RloxError::RuntimeError(operator.line, "Unknown binary operator".to_string(), operator.lexeme.clone()))
        }
    }
}
