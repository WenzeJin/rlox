//! Semantic Analysis

use std::collections::HashMap;
use std::rc::Rc;
use crate::error::RloxError;
use crate::interpreter::Interpreter;
use crate::ast::*;

enum FunctionType {
    None,
    Function,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scope: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    had_error: bool,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scope: Vec::new(),
            current_function: FunctionType::None,
            had_error: false,
        }
    }
}

impl<'a> Resolver<'a> {
    fn error(&mut self, err: RloxError) {
        // Handle the error (e.g., log it, print it, etc.)
        eprintln!("{}", err);
        self.had_error = true;
    }

    fn resolve_stmt(&mut self, stmt: &stmt::Stmt) {
        if let Err(err) = stmt.accept(self) {
            self.had_error = true;
            self.error(err);
        }
    }

    fn resolve_stmts(&mut self, stmts: &[stmt::Stmt]) {
        for stmt in stmts {
            self.resolve_stmt(stmt);
            if self.had_error {
                break;
            }
        }
    }

    fn resolve_expr(&mut self, expr: &expr::Expr) {
        if let Err(err) = expr.accept(self) {
            self.had_error = true;
            self.error(err);
        }
    }

    fn resolve_exprs(&mut self, exprs: &[expr::Expr]) {
        for expr in exprs {
            self.resolve_expr(expr);
            if self.had_error {
                break;
            }
        }
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

    fn resolve_local(&mut self, name: &Rc<token::Token>) -> Result<(), RloxError> {
        for (i, scope) in self.scope.iter().enumerate().rev() {
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
}

impl<'a> stmt::Visitor<Result<(), RloxError>> for Resolver<'a> {
    fn visit_block_stmt(&mut self, declarations: &Vec<stmt::Stmt>) -> Result<(), RloxError> {
        self.begin_scope();
        self.resolve_stmts(declarations);
        self.end_scope();
        Ok(())
    }


}

impl<'a> expr::Visitor<Result<(), RloxError>> for Resolver<'a> {
    fn visit_variable_expr(&mut self, name: &Rc<token::Token>) -> Result<(), RloxError> {
        if !self.scope.is_empty() {
            if let Some(scope) = self.scope.last() {
                if let Some(defined) = scope.get(&name.lexeme) {
                    if !defined {
                        return Err(RloxError::RuntimeError(
                            format!("Variable used before declaration: {}", name.lexeme),
                            String::new()
                        ));
                    }
                }
            }
        }
        self.resolve_local(name)?;
        Ok(())
    }

    fn visit_assign_expr(&mut self, left: &Rc<token::Token>, right: &expr::Expr) -> Result<(), RloxError> {
        right.accept(self)?;
        self.resolve_local(left)?;
        Ok(())
    }

    

}