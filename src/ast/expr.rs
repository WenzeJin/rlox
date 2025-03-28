//! Describes the expression AST nodes.
use crate::ast::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(LiteralValue),
    Unary(Token, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
    fn visit_literal_expr(&mut self, value: &LiteralValue) -> T;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary(left, operator, right) 
                => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping(expression) 
                => visitor.visit_grouping_expr(expression),
            Expr::Literal(value) 
                => visitor.visit_literal_expr(value),
            Expr::Unary(operator, right) 
                => visitor.visit_unary_expr(operator, right),
        }
    }
}