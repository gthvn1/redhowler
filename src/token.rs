// Token definitions for the Monkey language.
#[derive(PartialEq, Debug)]
pub enum Token {
    // Special tokens
    Illegal(String),
    EOF(String),

    // Identifiers + literals
    Ident(String),
    Int(String),

    // Operators
    Assign(String),
    Plus(String),

    // Delimiters
    Comma(String),
    Semicolon(String),
    LParen(String),
    RParen(String),
    LBrace(String),
    RBrace(String),

    // Keywords
    Function(String),
    Let(String),
}

impl Token {
    pub fn lookup_ident(ident: &str) -> Token {
        match ident {
            "fn" => Token::Function(String::from(ident)),
            "let" => Token::Let(String::from(ident)),
            _ => Token::Ident(String::from(ident)),
        }
    }
}
