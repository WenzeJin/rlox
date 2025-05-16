//! Describes the expression AST nodes.

use crate::ast::expr::Expr;
use crate::ast::token::Token;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Stmt {
    Var(Token, Option<Expr>),
    Block(Vec<Stmt>),
    Program(Vec<Stmt>),
    Expression(Expr),
    Print(Expr),
    Return(Option<Expr>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    FunctionDecl(Token, Vec<Token>, Rc<Vec<Stmt>>),   // Decl name, params, body. Body uses Rc, because function instance will link to it.
    ClassDecl(Token, Vec<Stmt>), // Class name, methods (FuntionDecl)
}

pub trait Visitor<T> {
    fn visit_program_stmt(&mut self, declarations: &Vec<Stmt>) -> T;
    fn visit_block_stmt(&mut self, declarations: &Vec<Stmt>) -> T;
    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;
    fn visit_print_stmt(&mut self, expression: &Expr) -> T;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> T;
    fn visit_if_stmt(&mut self, condition: &Expr, then_branch: &Box<Stmt>, else_branch: &Option<Box<Stmt>>) -> T;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Box<Stmt>) -> T;
    fn visit_function_decl_stmt(&mut self, name: &Token, params: &Vec<Token>, body: &Rc<Vec<Stmt>>) -> T;
    fn visit_return_stmt(&mut self, value: &Option<Expr>) -> T;
    fn visit_class_decl_stmt(&mut self, name: &Token, methods: &Vec<Stmt>) -> T;
}

impl Stmt {
    pub fn accept<T, V>(&self, visitor: &mut V) -> T
    where V: Visitor<T>,
    {
        match self {
            Stmt::Program(declarations) 
                => visitor.visit_program_stmt(declarations),
            Stmt::Block(declarations) 
                => visitor.visit_block_stmt(declarations),
            Stmt::Expression(expression) 
                => visitor.visit_expression_stmt(expression),
            Stmt::Print(expression) 
                => visitor.visit_print_stmt(expression),
            Stmt::Var(name, initializer) => 
                visitor.visit_var_stmt(name, initializer),
            Stmt::If(condition, then_branch, else_branch)
                => visitor.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::While(condition, body)
                => visitor.visit_while_stmt(condition, body),
            Stmt::FunctionDecl(name, params, body)
                => visitor.visit_function_decl_stmt(name, params, body),
            Stmt::Return(value)
                => visitor.visit_return_stmt(value),
            Stmt::ClassDecl(name, methods)
                => visitor.visit_class_decl_stmt(name, methods),

        }
    }
}