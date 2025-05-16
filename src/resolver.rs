//! Semantic Analysis

use std::collections::HashMap;
use std::rc::Rc;
use crate::error::RloxError;
use crate::interpreter::Interpreter;
use crate::ast::*;

#[derive(Debug, Clone, Eq, PartialEq)]
enum FunctionType {
    None,
    Function,
    Method,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ClassType {
    None,
    Class,
}

pub struct Resolver<'a> {
    pub interpreter: &'a mut Interpreter,
    scope: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
    pub had_error: bool,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scope: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
            had_error: false,
        }
    }
}

impl<'a> Resolver<'a> {
    fn error(&mut self, err: RloxError) {
        // Handle the error (e.g., log it, print it, etc.)
        println!("{}", err);
        self.had_error = true;
    }

    pub fn resolve_program(&mut self, program: &stmt::Stmt) {
        if let Err(err) = program.accept(self) {
            self.had_error = true;
            self.error(err);
        }
    }

    fn resolve_stmt(&mut self, stmt: &stmt::Stmt) -> Result<(), RloxError> {
        stmt.accept(self)
    }

    fn resolve_stmts(&mut self, stmts: &[stmt::Stmt]) -> Result<(), RloxError>{
        for stmt in stmts {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &expr::Expr) -> Result<(), RloxError>{
        expr.accept(self)
    }

    fn resolve_exprs(&mut self, exprs: &[expr::Expr]) -> Result<(), RloxError>{
        for expr in exprs {
            self.resolve_expr(expr)?;
        }
        Ok(())
    }

    fn begin_scope(&mut self) {
        self.scope.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scope.pop();
    }

    fn declare(&mut self, name: &token::Token) -> Result<(), RloxError> {
        let mut error = false;
        if let Some(scope) = self.scope.last_mut() {
            if scope.contains_key(&name.lexeme) {
                error = true;
            } else {
                scope.insert(name.lexeme.clone(), false);
            }
        }
        if error {
            Err(RloxError::RuntimeError(
                format!("Variable with this name already declared in this scope: {}", name.lexeme),
                String::new()
            ))
        } else {
            Ok(())
        }
    }

    fn define(&mut self, name: &token::Token) {
        if let Some(scope) = self.scope.last_mut() {
            if let Some(defined) = scope.get_mut(&name.lexeme) {
                *defined = true;
            }
        }
    }

    fn resolve_local(&mut self, name: &token::Token) -> Result<(), RloxError> {
        for (i, scope) in self.scope.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                // Variable is defined in this scope
                self.interpreter.resolve(name, i);
                return Ok(());
            }
        }
        // Not found in any scope
        Err(RloxError::RuntimeError(
            format!("Undefined variable: {}", name.lexeme),
            String::new()
        ))
    }

    fn resolve_function(&mut self, params: &Vec<token::Token>, body: &Rc<Vec<stmt::Stmt>>, 
        decl: FunctionType) -> Result<(), RloxError> {
        self.begin_scope();
        let old_function = self.current_function.clone();
        self.current_function = decl;
        for param in params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve_stmts(body)?;
        self.end_scope();
        self.current_function = old_function;
        Ok(())
    }
}

impl<'a> stmt::Visitor<Result<(), RloxError>> for Resolver<'a> {
    fn visit_program_stmt(&mut self, declarations: &Vec<stmt::Stmt>) -> Result<(), RloxError> {
        self.begin_scope();
        self.current_function = FunctionType::None;
        self.resolve_stmts(declarations)?;
        self.end_scope();
        Ok(())
    }

    fn visit_block_stmt(&mut self, declarations: &Vec<stmt::Stmt>) -> Result<(), RloxError> {
        self.begin_scope();
        self.resolve_stmts(declarations)?;
        self.end_scope();
        Ok(())
    }

    fn visit_expression_stmt(&mut self, expression: &expr::Expr) -> Result<(), RloxError> {
        self.resolve_expr(expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, expression: &expr::Expr) -> Result<(), RloxError> {
        self.resolve_expr(expression)?;
        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &token::Token, initializer: &Option<expr::Expr>) -> Result<(), RloxError> {
        self.declare(name)?;
        if let Some(initializer) = initializer {
            self.resolve_expr(initializer)?;
        }
        self.define(name);
        Ok(())
    }

    fn visit_if_stmt(&mut self, condition: &expr::Expr, then_branch: &Box<stmt::Stmt>, else_branch: &Option<Box<stmt::Stmt>>) -> Result<(), RloxError> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(then_branch)?;
        if let Some(else_branch) = else_branch {
            self.resolve_stmt(else_branch)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &expr::Expr, body: &Box<stmt::Stmt>) -> Result<(), RloxError> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(body)?;
        Ok(())
    }

    fn visit_function_decl_stmt(&mut self, name: &token::Token, params: &Vec<token::Token>, body: &Rc<Vec<stmt::Stmt>>) -> Result<(), RloxError> {
        self.declare(name)?;
        self.define(name);
        self.resolve_function(params, body, FunctionType::Function)?;
        Ok(())
    }

    fn visit_return_stmt(&mut self, value: &Option<expr::Expr>) -> Result<(), RloxError> {
        if self.current_function == FunctionType::None {
            return Err(RloxError::RuntimeError(
                format!("Can't return from non-function."),
                String::new()
            ));
        }
        if let Some(value) = value {
            self.resolve_expr(value)?;
        }
        Ok(())
    }

    fn visit_class_decl_stmt(&mut self, name: &token::Token, methods: &Vec<stmt::Stmt>) -> Result<(), RloxError> {
        self.declare(name)?;
        self.define(name);

        self.begin_scope();
        self.scope.last_mut().unwrap().insert("this".to_string(), true);
        let enclosing_class = self.current_class.clone();
        self.current_class = ClassType::Class;

        for method in methods {
            if let stmt::Stmt::FunctionDecl(_, params, body) = method {
                self.resolve_function(params, body, FunctionType::Method)?;
            } else {
                unreachable!("Class methods should be function declarations");
            }
        }

        self.current_class = enclosing_class;
        self.end_scope();

        Ok(())
    }
}

impl<'a> expr::Visitor<Result<(), RloxError>> for Resolver<'a> {
    fn visit_binary_expr(&mut self, left: &expr::Expr, _operator: &token::Token, right: &expr::Expr) -> Result<(), RloxError> {
        left.accept(self)?;
        right.accept(self)?;
        Ok(())
    }

    fn visit_logical_expr(&mut self, left: &expr::Expr, _: &token::Token, right: &expr::Expr) -> Result<(), RloxError> {
        left.accept(self)?;
        right.accept(self)?;
        Ok(())
    }

    fn visit_grouping_expr(&mut self, expression: &expr::Expr) -> Result<(), RloxError> {
        expression.accept(self)?;
        Ok(())
    }

    fn visit_literal_expr(&mut self, _: &expr::LiteralValue) -> Result<(), RloxError> {
        Ok(())
    }

    fn visit_unary_expr(&mut self, _operator: &token::Token, right: &expr::Expr) -> Result<(), RloxError> {
        right.accept(self)?;
        Ok(())
    }

    fn visit_variable_expr(&mut self, name: &token::Token) -> Result<(), RloxError> {
        if !self.scope.is_empty() {
            if let Some(scope) = self.scope.last() {
                if let Some(defined) = scope.get(&name.lexeme) {
                    if !defined {
                        return Err(RloxError::RuntimeError(
                            format!("Can't read local variable in its own initializer."),
                            String::new()
                        ));
                    }
                }
            }
        }
        self.resolve_local(name)?;
        Ok(())
    }

    fn visit_assign_expr(&mut self, left: &token::Token, right: &expr::Expr) -> Result<(), RloxError> {
        right.accept(self)?;
        self.resolve_local(left)?;
        Ok(())
    }

    fn visit_call_expr(&mut self, callee: &expr::Expr, arguments: &[expr::Expr]) -> Result<(), RloxError> {
        callee.accept(self)?;
        for argument in arguments {
            argument.accept(self)?;
        }
        Ok(())
    }

    fn visit_get_expr(&mut self, object: &expr::Expr, _name: &token::Token) -> Result<(), RloxError> {
        object.accept(self)?;
        Ok(())
    }

    fn visit_set_expr(&mut self, object: &expr::Expr, _name: &token::Token, value: &expr::Expr) -> Result<(), RloxError> {
        object.accept(self)?;
        value.accept(self)?;
        Ok(())
    }

    fn visit_this_expr(&mut self, name: &token::Token) -> Result<(), RloxError> {
        if self.current_class == ClassType::None {
            return Err(RloxError::RuntimeError(
                format!("Can't use 'this' outside of a class."),
                String::new()
            ));
        }
        self.resolve_local(name)?;
        Ok(())
    }

}