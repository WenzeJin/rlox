//! This module contains the parser for the language.

use std::vec;
use std::rc::Rc;

use crate::ast::{token::*, expr::*, stmt::*};
use crate::error::{RloxError, report};
use unescape::unescape;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub had_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
            had_error: false,
        }
    }
}

impl Parser {
    pub fn parse_expr(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_e) => None,
        }
    }

    pub fn parse(&mut self) -> Option<Stmt> {
        self.program()
    }
}

impl Parser {
    
    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        };

        false
    }

    fn error(&mut self, message: &str) -> RloxError {
        self.had_error = true;
        let error = RloxError::SyntaxError(
            self.peek().line,
            message.to_string(),
            self.peek().lexeme.clone(),
        );
        report(&error);
        error
    }

    fn consume(&mut self, t: TokenType, message: &str) -> Result<&Token, RloxError> {
        if self.check(t) {
            Ok(self.advance())
        } else {
            Err(self.error(message))
        }
    }

    fn check(&self, t: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().t_type == t
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().t_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().t_type == TokenType::Semicolon {
                return;
            }

            match self.peek().t_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For |
                TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => {
                    return;
                },
                _ => {}
            }

            self.advance();
        }
    }

}

/// Parser methods for parsing expressions
impl Parser {

    fn expression(&mut self) -> Result<Expr, RloxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, RloxError> {
        let expr = self.or()?;

        if self.match_token(vec![TokenType::Equal]) {
            let value = self.assignment()?;
            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(name, Box::new(value)));
            } else {
                return Err(self.error("Invalid assignment target, requires variable"));
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, RloxError> {
        let mut expr = self.and()?;

        while self.match_token(vec![TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        };

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, RloxError> {
        let mut expr = self.equality()?;

        while self.match_token(vec![TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        };

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, RloxError> {
        let mut expr = self.comparison()?;

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        };

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, RloxError> {
        let mut expr = self.term()?;

        while self.match_token(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        };

       Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, RloxError> {
        let mut expr = self.factor()?;

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        };

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, RloxError> {
        let mut expr = self.unary()?;

        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        };

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, RloxError> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expr, RloxError> {
        // call -> primary ( '(' arguments? ')' )*
        let mut expr = self.primary()?;

        loop {
            if self.match_token(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, RloxError> {
        let mut arguments = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(self.error("Cannot have more than 255 arguments"));
                }
                arguments.push(self.expression()?);
                if !self.match_token(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after arguments")?;
        Ok(Expr::Call(Box::new(callee), arguments, self.previous().line))
    }

    fn primary(&mut self) -> Result<Expr, RloxError> {
        match self.peek().t_type {
            TokenType::False => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Boolean(false)))
            },
            TokenType::True => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Boolean(true)))
            },
            TokenType::Nil => {
                self.advance();
                Ok(Expr::Literal(LiteralValue::Nil))
            },
            TokenType::Identifier => {
                let name = self.advance().clone();
                Ok(Expr::Variable(Rc::new(name)))
            },
            TokenType::Number => {
                let lexeme = self.peek().lexeme.clone();
                let number = lexeme.parse::<f64>().unwrap();
                self.advance();
                Ok(Expr::Literal(LiteralValue::Number(number)))
            },
            TokenType::String => {
                let lexeme = self.peek().lexeme.clone();
                let string = lexeme[1..lexeme.len()-1].to_string();
                match unescape(&string) {
                    Some(unescaped) => {
                        self.advance();
                        Ok(Expr::Literal(LiteralValue::String(unescaped)))
                    },
                    None => {
                        report(&RloxError::LexicalError(
                            self.peek().line,
                            "Invalid string escape sequence".to_string(),
                            string.clone(),
                        ));
                        let lit = string.clone();
                        self.advance();
                        self.had_error = true;
                        Ok(Expr::Literal(LiteralValue::String(lit)))
                    }
                }
            },
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.advance();
                Ok(Expr::Grouping(Box::new(expr)))
            },
            _ => Err(self.error("Expected expression")),
        }
    }

}

/// Parser methods for parsing statements
impl Parser {

    fn program(&mut self) -> Option<Stmt> {
        let mut statements = vec![];
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        };
        Some(Stmt::Program(statements))
    }

    fn block(&mut self) -> Result<Stmt, RloxError> {
        let mut statements = vec![];
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        };
        self.consume(TokenType::RightBrace, "Expect '}' after block")?;
        Ok(Stmt::Block(statements))
    }

    /// Parses a single declaration.
    /// Statement is also a declaration, so this method can be used to parse both.
    /// If the statement is not a declaration, it will return None, not an error.
    /// If any error occurs, it will return None and synchronize the parser.
    fn declaration(&mut self) -> Option<Stmt> {
        match if self.match_token(vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        } {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, RloxError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name")?.clone();
        let mut initializer: Option<Expr> = None;
        if self.match_token(vec![TokenType::Equal]) {
            let expression = self.expression()?;
            initializer = Some(expression);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration")?;
        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, RloxError> {
        if self.match_token(vec![TokenType::Print]) {
            self.print_statement()
        } else if self.match_token(vec![TokenType::LeftBrace]) {
            self.block()
        } else if self.match_token(vec![TokenType::If]) {
            self.if_statement() 
        } else if self.match_token(vec![TokenType::While]) {
            self.while_statement()
        } else if self.match_token(vec![TokenType::For]) {
            self.for_statement()
        } else if self.match_token(vec![TokenType::Fun]) {
            self.function_declaration("function")
        } else if self.match_token(vec![TokenType::Return]) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, RloxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value")?;
        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, RloxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression")?;
        Ok(Stmt::Expression(expr))
    }

    fn if_statement(&mut self) -> Result<Stmt, RloxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition")?;
        let then_branch = self.statement()?;
        let else_branch = if self.match_token(vec![TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If(condition, Box::new(then_branch), else_branch))
    }

    fn while_statement(&mut self) -> Result<Stmt, RloxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after while condition")?;
        let body = self.statement()?;
        Ok(Stmt::While(condition, Box::new(body)))
    }

    fn for_statement(&mut self) -> Result<Stmt, RloxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'")?;
        let initializer = if self.match_token(vec![TokenType::Var]) {
            Some(self.var_declaration()?)
        } else if self.match_token(vec![TokenType::Semicolon]) {
            None
        } else {
            Some(self.expression_statement()?)
        };

        let mut condition: Option<Expr> = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition")?;

        let mut increment: Option<Expr> = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses")?;

        let mut body = self.statement()?;

        // insert increment statement at the end of the body
        if let Some(increment) = increment {
            if let Stmt::Block(ref mut block) = body {
                block.push(Stmt::Expression(increment));
            } else {
                body = Stmt::Block(vec![body, Stmt::Expression(increment)]);
            }
        }

        // first build the inner while
        let while_body = Stmt::While(
            condition.unwrap_or(Expr::Literal(LiteralValue::Boolean(true))), // if no condition, loop forever
            Box::new(body),
        );

        // then build the outer block
        let mut block: Vec<Stmt> = vec![];
        if let Some(inner) = initializer {
            block.push(inner);
        }
        block.push(while_body);
        Ok(Stmt::Block(block))
    }

    fn function_declaration(&mut self, kind: &str) -> Result<Stmt, RloxError> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {} name", kind))?.clone();
        self.consume(TokenType::LeftParen, &format!("Expect '(' after {} name", kind))?;
        let mut params = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(self.error("Can't have more than 255 parameters"));
                }
                params.push(self.consume(TokenType::Identifier, "Expect parameter name")?.clone());
                if !self.match_token(vec![TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters")?;
        self.consume(TokenType::LeftBrace, &format!("Expect '{{' before {} body", kind))?;
        let body_block = self.block()?;
        let body = match body_block {
            Stmt::Block(block) => block,
            _ => panic!("should not happen"),
        };
        Ok(Stmt::FunctionDecl(name, params, Rc::new(body)))
    }

    fn return_statement(&mut self) -> Result<Stmt, RloxError> {
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after return value")?;
        Ok(Stmt::Return(value))
    }
}