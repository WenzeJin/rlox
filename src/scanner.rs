//! Scanner:
//! This is a lexer module that scans the source code and generates tokens
//! 
//! Author: Wenze Jin

use std::collections::HashMap;
use crate::ast::token::{Token, TokenType, Literal};
use crate::error::{RloxError, report};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        let keywords = generate_keywords();
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // push EOF token
        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
        
        std::mem::take(&mut self.tokens)
    }
}

impl Scanner {
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c: u8 = self.advance();
        match c {
            // Single-character tokens
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b';' => self.add_token(TokenType::Semicolon),
            b'*' => self.add_token(TokenType::Star),

            // One or two character tokens
            b'!' => {
                let is_equal = self.match_ch(b'=');
                self.add_token(if is_equal { TokenType::BangEqual } else { TokenType::Bang });
            },
            b'=' => {
                let is_equal = self.match_ch(b'=');
                self.add_token(if is_equal { TokenType::EqualEqual } else { TokenType::Equal });
            },
            b'<' => {
                let is_equal = self.match_ch(b'=');
                self.add_token(if is_equal { TokenType::LessEqual } else { TokenType::Less });
            },
            b'>' => {
                let is_equal = self.match_ch(b'=');
                self.add_token(if is_equal { TokenType::GreaterEqual } else { TokenType::Greater });
            },

            // Slash or comment
            b'/' => {
                if self.match_ch(b'/') {    // this is a comment
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            // String literals handling
            b'"' => self.string(),

            // Ignore whitespaces
            b' ' | b'\r' | b'\t' => (),
            b'\n' => self.line += 1,

            // Default handling
            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    report(RloxError::LexicalError(self.line, "Unexpected character".to_string(), (c as char).to_string()))
                }
            },
        };
    }

    fn add_token(&mut self, t_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(t_type, text, None, self.line));
    }

    fn add_token_with_lit(&mut self, t_type: TokenType, literal: Literal) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(t_type, text, Some(literal), self.line));
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source.as_bytes()[self.current - 1]
    }

    fn match_ch(&mut self, ch: u8) -> bool {
        if self.is_at_end() {
            false
        } else {
            if ch != self.source.as_bytes()[self.current] {
                false
            } else {
                self.current += 1;
                true
            }
        }
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            b'\0'
        } else {
            self.source.as_bytes()[self.current + 1]
        }
    }
}

impl Scanner {
    fn string(&mut self) {
        let mut escaped = false;

        while (self.peek() != b'"' || escaped) && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            if self.peek() == b'\\' {
                escaped = !escaped;
            } else {
                escaped = false;
            }
            self.advance();
        }

        if self.is_at_end() {
            report(RloxError::LexicalError(self.line, "Unterminated string".to_string(), self.source[self.start..self.current].to_string()));
            return;
        }

        // closing quote
        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_lit(TokenType::String, Literal::String(value));
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == b'.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        match self.source[self.start..self.current].parse::<f64>() {
            Ok(value) => self.add_token_with_lit(TokenType::Number, Literal::Number(value)),
            Err(_) => report(RloxError::LexicalError(self.line, "Invalid number".to_string(), self.source[self.start..self.current].to_string())),
        }
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let t_type = self.keywords.get(&text).unwrap_or(&TokenType::Identifier).clone();
        self.add_token(t_type);
    }
}

/* Helper funtions */

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

fn is_alpha(c: u8) -> bool {
    (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_'
}

fn is_alpha_numeric(c: u8) -> bool {
    is_alpha(c) || is_digit(c)
}

/* Keywords Map */

fn generate_keywords() -> HashMap<String, TokenType> {
    let mut keywords: HashMap<String, TokenType> = HashMap::with_capacity(30);
    keywords.insert("and".to_string(), TokenType::And);
    keywords.insert("class".to_string(), TokenType::Class);
    keywords.insert("else".to_string(), TokenType::Else);
    keywords.insert("false".to_string(), TokenType::False);
    keywords.insert("for".to_string(), TokenType::For);
    keywords.insert("fun".to_string(), TokenType::Fun);
    keywords.insert("if".to_string(), TokenType::If);
    keywords.insert("nil".to_string(), TokenType::Nil);
    keywords.insert("or".to_string(), TokenType::Or);
    keywords.insert("print".to_string(), TokenType::Print);
    keywords.insert("return".to_string(), TokenType::Return);
    keywords.insert("super".to_string(), TokenType::Super);
    keywords.insert("this".to_string(), TokenType::This);
    keywords.insert("true".to_string(), TokenType::True);
    keywords.insert("var".to_string(), TokenType::Var);
    keywords.insert("while".to_string(), TokenType::While);
    keywords
}