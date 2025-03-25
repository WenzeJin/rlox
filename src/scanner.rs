
use crate::ast::token::{Token, TokenType, Literal};
use crate::error::{RloxError, report};

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(&mut tokens);
        }

        // push EOF token
        tokens.push(Token::new(TokenType::EOF, "".to_string(), None, self.line));
        tokens
    }
}

impl Scanner {
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self, tokens: &mut Vec<Token>) {
        let c: u8 = self.advance();
        match c {
            // Single-character tokens
            b'(' => self.add_token(tokens, TokenType::LeftParen),
            b')' => self.add_token(tokens, TokenType::RightParen),
            b'{' => self.add_token(tokens, TokenType::LeftBrace),
            b'}' => self.add_token(tokens, TokenType::RightBrace),
            b',' => self.add_token(tokens, TokenType::Comma),
            b'.' => self.add_token(tokens, TokenType::Dot),
            b'-' => self.add_token(tokens, TokenType::Minus),
            b'+' => self.add_token(tokens, TokenType::Plus),
            b';' => self.add_token(tokens, TokenType::Semicolon),
            b'*' => self.add_token(tokens, TokenType::Star),

            // One or two character tokens
            b'!' => {
                let is_equal = self.match_ch(b'=');
                self.add_token(tokens, if is_equal { TokenType::BangEqual } else { TokenType::Bang });
            },
            b'=' => {
                let is_equal = self.match_ch(b'=');
                self.add_token(tokens, if is_equal { TokenType::EqualEqual } else { TokenType::Equal });
            },
            b'<' => {
                let is_equal = self.match_ch(b'=');
                self.add_token(tokens, if is_equal { TokenType::LessEqual } else { TokenType::Less });
            },
            b'>' => {
                let is_equal = self.match_ch(b'=');
                self.add_token(tokens, if is_equal { TokenType::GreaterEqual } else { TokenType::Greater });
            },

            // Slash or comment
            b'/' => {
                if self.match_ch(b'/') {    // this is a comment
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(tokens, TokenType::Slash);
                }
            },
            // String literals handling
            b'"' => self.string(tokens),

            // Ignore whitespaces
            b' ' | b'\r' | b'\t' => (),
            b'\n' => self.line += 1,

            // Default handling
            _ => {
                if is_digit(c) {
                    self.number(tokens);
                } else {
                    report(RloxError::LexicalError(self.line, "Unexpected character".to_string(), (c as char).to_string()))
                }
            },
        };
    }

    fn add_token(&mut self, tokens: &mut Vec<Token>, t_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        tokens.push(Token::new(t_type, text, None, self.line));
    }

    fn add_token_with_lit(&mut self, tokens: &mut Vec<Token>, t_type: TokenType, literal: Literal) {
        let text = self.source[self.start..self.current].to_string();
        tokens.push(Token::new(t_type, text, Some(literal), self.line));
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
    fn string(&mut self, tokens: &mut Vec<Token>) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
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
        self.add_token_with_lit(tokens, TokenType::String, Literal::String(value));
    }

    fn number(&mut self, tokens: &mut Vec<Token>) {
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
            Ok(value) => self.add_token_with_lit(tokens, TokenType::Number, Literal::Number(value)),
            Err(_) => report(RloxError::LexicalError(self.line, "Invalid number".to_string(), self.source[self.start..self.current].to_string())),
        }
    }
}

/* Helper funtions */

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

