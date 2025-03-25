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

    fn visit_literal_expr(&mut self, value: &token::Token) -> String {
        return value.lexeme.clone();
    }

    fn visit_unary_expr(&mut self, operator: &token::Token, right: &expr::Expr) -> String {
        return self.parenthesize(&operator.lexeme, vec![right]);
    }
    
}

