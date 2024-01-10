use super::keywords::{Keyword, Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Ident,
    Keyword(Keyword),
    Type(Type),
    LParen,
    RParen,
    LSquirly,
    RSquirly,
    Colon,
    Comma,
    Slash,
    Assign,
    Plus,
    Minus,
    Asterisk,
    Bang,
    Eq,
    NotEq,
    Greater,
    Less,
    GreaterEq,
    LessEq,
    Number,
    Function,
    Invalid,
    Comment,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub position: (usize, usize), // (line, col)
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str, position: (usize, usize)) -> Self {
        Self {
            token_type,
            literal: literal.to_owned(),
            position,
        }
    }
}
