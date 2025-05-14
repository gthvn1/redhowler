// Token definitions for the Monkey language.
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum TokenType {
    // Special tokens
    Illegal,
    EOF,

    // Identifiers + literals
    Ident,
    Int,

    // One character operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,

    // Two characters operators
    Equal,    // ==
    NotEqual, // !=

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        Self {
            token_type,
            literal: literal.to_string(),
        }
    }

    pub fn literal(&self) -> String {
        self.literal.clone()
    }
}
