use crate::ast::*;
use std::rc::Rc;

pub struct AstPrinter();

impl AstPrinter {
    fn parenthesize(&mut self, name: &str, exprs: Vec<&expr::Expr>) -> String {
        let mut result = String::new();
        result.push_str("(");
        result.push_str(name);
        for expr in exprs {
            result.push_str(" ");
            result.push_str(&expr.accept(self));
        }
        result.push_str(")");
        result
    }

    fn block(&mut self, statements: &Vec<stmt::Stmt>) -> String {
        let mut result = String::new();
        // [stmt1;stmt2;...]
        result.push_str("[");
        for statement in statements {
            result.push_str(&statement.accept(self));
            result.push_str(";");
        }
        // Remove the last semicolon
        if result.len() > 1 {
            result.pop();
        }
        result.push_str("]");
        result
    }
}

impl expr::Visitor<String> for AstPrinter {

    fn visit_binary_expr(&mut self, left: &expr::Expr, operator: &token::Token, right: &expr::Expr) -> String {
        return self.parenthesize(&operator.lexeme, vec![left, right]);
    }

    fn visit_logical_expr(&mut self, left: &expr::Expr, operator: &token::Token, right: &expr::Expr) -> String {
        return self.parenthesize(&operator.lexeme, vec![left, right]);
    }
    
    fn visit_grouping_expr(&mut self, expression: &expr::Expr) -> String {
        return self.parenthesize("group", vec![expression]);
    }

    fn visit_literal_expr(&mut self, value: &expr::LiteralValue) -> String {
        return match value {
            expr::LiteralValue::Number(n) => n.to_string(),
            expr::LiteralValue::String(s) => s.clone(),
            expr::LiteralValue::Boolean(b) => b.to_string(),
            expr::LiteralValue::Nil => "nil".to_string(),
        }
    }

    fn visit_unary_expr(&mut self, operator: &token::Token, right: &expr::Expr) -> String {
        return self.parenthesize(&operator.lexeme, vec![right]);
    }

    fn visit_variable_expr(&mut self, name: &token::Token) -> String {
        return name.lexeme.clone();
    }

    fn visit_assign_expr(&mut self, left: &token::Token, right: &expr::Expr) -> String {
        let mut result = String::new();
        result.push_str("(= ");
        result.push_str(&left.lexeme);
        result.push_str(" ");
        result.push_str(&right.accept(self));
        result.push_str(")");
        result
    }

    fn visit_call_expr(&mut self, callee: &expr::Expr, arguments: &[expr::Expr]) -> String {
        let mut result = String::new();
        result.push_str("(call ");
        result.push_str(&callee.accept(self));
        for argument in arguments {
            result.push_str(" ");
            result.push_str(&argument.accept(self));
        }
        result.push_str(")");
        result
    }
    
}

impl stmt::Visitor<String> for AstPrinter {
    fn visit_block_stmt(&mut self, declarations: &Vec<stmt::Stmt>) -> String {
        self.block(declarations)
    }

    fn visit_program_stmt(&mut self, declarations: &Vec<stmt::Stmt>) -> String {
        self.block(declarations)
    }

    fn visit_expression_stmt(&mut self, expression: &expr::Expr) -> String {
        expression.accept(self)
    }

    fn visit_print_stmt(&mut self, expression: &expr::Expr) -> String {
        self.parenthesize("print", vec![expression])
    }

    fn visit_var_stmt(&mut self, name: &token::Token, initializer: &Option<expr::Expr>) -> String {
        let mut result = String::new();
        result.push_str("(var ");
        result.push_str(&name.lexeme);
        if let Some(expr) = initializer {
            result.push_str(" = ");
            result.push_str(&expr.accept(self));
        }
        result.push_str(")");
        result
    }

    fn visit_if_stmt(&mut self, condition: &expr::Expr, then_branch: &Box<stmt::Stmt>, else_branch: &Option<Box<stmt::Stmt>>) -> String {
        let mut result = String::new();
        result.push_str("(if ");
        result.push_str(&condition.accept(self));
        result.push_str(" ");
        result.push_str(&then_branch.accept(self));
        if let Some(else_branch) = else_branch {
            result.push_str(" ");
            result.push_str(&else_branch.accept(self));
        }
        result.push_str(")");
        result
    }

    fn visit_while_stmt(&mut self, condition: &expr::Expr, body: &Box<stmt::Stmt>) -> String {
        let mut result = String::new();
        result.push_str("(while ");
        result.push_str(&condition.accept(self));
        result.push_str(" ");
        result.push_str(&body.accept(self));
        result.push_str(")");
        result
    }

    fn visit_function_decl_stmt(&mut self, name: &token::Token, params: &Vec<token::Token>, body: &Rc<Vec<stmt::Stmt>>) -> String {
        let mut result = String::new();
        result.push_str("(function ");
        result.push_str(&name.lexeme);
        result.push_str(" (");
        for param in params {
            result.push_str(&param.lexeme);
            result.push_str(" ");
        }
        result.push_str(") ");
        result.push_str(&self.block(body));
        result.push_str(")");
        result
    }

    fn visit_return_stmt(&mut self, value: &Option<expr::Expr>) -> String {
        let mut result = String::new();
        result.push_str("(return");
        if let Some(expr) = value {
            result.push_str(" ");
            result.push_str(&expr.accept(self));
        }
        result.push_str(")");
        result
    }
}

