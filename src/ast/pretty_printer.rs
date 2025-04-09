use crate::ast::*;

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
    
}

impl stmt::Visitor<String> for AstPrinter {
    fn visit_block_stmt(&mut self, statements: &Vec<stmt::Stmt>) -> String {
        self.block(statements)
    }

    fn visit_expression_stmt(&mut self, expression: &expr::Expr) -> String {
        expression.accept(self)
    }

    fn visit_print_stmt(&mut self, expression: &expr::Expr) -> String {
        self.parenthesize("print", vec![expression])
    }

    fn visit_var_stmt(&mut self, name: &token::Token, initializer: &Option<Box<expr::Expr>>) -> String {
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
}

