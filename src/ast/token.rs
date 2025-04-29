#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub t_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            t_type,
            lexeme,
            line,
        }
    }
}