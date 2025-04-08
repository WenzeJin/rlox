//! This module contains the parser for the language.

use std::vec;

use crate::ast::{token::*, expr::*};
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
    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_e) => None,
        }
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