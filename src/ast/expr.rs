//! Describes the expression AST nodes.
use crate::ast::token::Token;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(LiteralValue),
    Unary(Token, Box<Expr>),
    Assign(Rc<Token>, Box<Expr>),
    Variable(Rc<Token>),
    Call(Box<Expr>, Vec<Expr>, usize), // (callee, arguments, line)
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
    fn visit_logical_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&mut self, expression: &Expr) -> T;
    fn visit_literal_expr(&mut self, value: &LiteralValue) -> T;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_variable_expr(&mut self, name: &Rc<Token>) -> T;
    fn visit_assign_expr(&mut self, left: &Rc<Token>, right: &Expr) -> T;
    fn visit_call_expr(&mut self, callee: &Expr, arguments: &[Expr]) -> T;
}

impl Expr {
    pub fn accept<T, V>(&self, visitor: &mut V) -> T 
    where V: Visitor<T>,
    {
        match self {
            Expr::Binary(left, operator, right) 
                => visitor.visit_binary_expr(left, operator, right),
            Expr::Logical(left, operator, right)
                => visitor.visit_logical_expr(left, operator, right),
            Expr::Grouping(expression) 
                => visitor.visit_grouping_expr(expression),
            Expr::Literal(value) 
                => visitor.visit_literal_expr(value),
            Expr::Unary(operator, right) 
                => visitor.visit_unary_expr(operator, right),
            Expr::Variable(name)
                => visitor.visit_variable_expr(name),
            Expr::Assign(left, right)
                => visitor.visit_assign_expr(left, right),
            Expr::Call(callee, arguments, _)
                => visitor.visit_call_expr(callee, arguments),
        }
    }
}