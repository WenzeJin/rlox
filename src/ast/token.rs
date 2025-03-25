#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Token {
        Token {
            t_type,
            lexeme,
            literal,
            line,
        }
    }
}