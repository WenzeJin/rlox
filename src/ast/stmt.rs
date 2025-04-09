//! Describes the expression AST nodes.

use crate::ast::expr::Expr;
use crate::ast::token::Token;
use crate::ast::token::TokenType;

#[derive(Debug, Clone)]
pub enum Stmt {
    Var(Token, Option<Box<Expr>>),
    Block(Vec<Stmt>),
    Expression(Box<Expr>),
    Print(Box<Expr>),
}

pub trait Visitor<T> {
    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> T;
    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;
    fn visit_print_stmt(&mut self, expression: &Expr) -> T;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Box<Expr>>) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Block(statements) 
                => visitor.visit_block_stmt(statements),
            Stmt::Expression(expression) 
                => visitor.visit_expression_stmt(expression),
            Stmt::Print(expression) 
                => visitor.visit_print_stmt(expression),
            Stmt::Var(name, initializer) => 
                visitor.visit_var_stmt(name, initializer),
        }
    }
}