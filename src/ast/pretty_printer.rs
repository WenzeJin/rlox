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
    
}

