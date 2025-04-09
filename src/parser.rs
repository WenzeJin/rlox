//! This module contains the parser for the language.

use std::vec;

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
        self.statements()
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
impl Parser{

    fn expression(&mut self) -> Result<Expr, RloxError> {
        self.equality()
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

        self.primary()
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
                Ok(Expr::Variable(name))
            },
            TokenType::Number => {
                if let Some(Literal::Number(number)) = self.peek().literal {
                    self.advance();
                    Ok(Expr::Literal(LiteralValue::Number(number)))
                } else {
                    panic!("Expected number literal, got {:?}", self.peek().literal);
                }
            },
            TokenType::String => {
                if let Some(Literal::String(string)) = self.peek().literal.as_ref() {
                    match unescape(string) {
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
                } else {
                    panic!("Expected string literal, got {:?}", self.peek().literal);
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
    fn statements(&mut self) -> Option<Stmt> {
        let mut statements = vec![];
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        };
        Some(Stmt::Block(statements))
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
        let mut initializer: Option<Box<Expr>> = None;
        if self.match_token(vec![TokenType::Equal]) {
            initializer = Some(Box::new(self.expression()?));
        }
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration")?;
        Ok(Stmt::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, RloxError> {
        if self.match_token(vec![TokenType::Print]) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, RloxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value")?;
        Ok(Stmt::Print(Box::new(value)))
    }

    fn expression_statement(&mut self) -> Result<Stmt, RloxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression")?;
        Ok(Stmt::Expression(Box::new(expr)))
    }
}