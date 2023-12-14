// Token definitions for the Monkey language.
#[derive(PartialEq, Debug)]
pub enum TokenType {
    // Special tokens
    Illegal,
    EOF,

    // Identifiers + literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

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
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
