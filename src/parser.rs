//! This module contains the parser for the language.

use std::vec;

use crate::ast::{token::*, expr::*};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }
}

impl Parser {
    pub fn parse(&mut self) -> Expr {
        self.expression()
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

}

impl Parser{

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        };

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        };

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        };

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        };

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(operator, Box::new(right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        match self.peek().t_type {
            TokenType::False => {
                self.advance();
                Expr::Literal(LiteralValue::Boolean(false))
            },
            TokenType::True => {
                self.advance();
                Expr::Literal(LiteralValue::Boolean(true))
            },
            TokenType::Nil => {
                self.advance();
                Expr::Literal(LiteralValue::Nil)
            },
            TokenType::Number => {
                if let Some(Literal::Number(number)) = self.peek().literal {
                    self.advance();
                    Expr::Literal(LiteralValue::Number(number))
                } else {
                    panic!("Expected number literal, got {:?}", self.peek().literal);
                }
            },
            TokenType::String => {
                if let Some(Literal::String(string)) = self.peek().literal.as_ref() {
                    let string = string.clone();
                    self.advance();
                    Expr::Literal(LiteralValue::String(string))
                } else {
                    panic!("Expected string literal, got {:?}", self.peek().literal);
                }
            },
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                self.advance();
                Expr::Grouping(Box::new(expr))
            },
            _ => panic!("Unexpected token: {:?}", self.peek().t_type),
        }
    }

}