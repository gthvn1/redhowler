// Token definitions for the Monkey language.
#[derive(PartialEq, Debug)]
pub enum Token {
    // Special tokens
    Illegal(String),
    EOF(String),

    // Identifiers + literals
    Ident(String),
    Int(String),

    // One character operators
    Assign(String),
    Plus(String),
    Minus(String),
    Bang(String),
    Asterisk(String),
    Slash(String),
    LT(String),
    GT(String),

    // Two characters operators
    Equal(String),    // ==
    NotEqual(String), // !=

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
    True(String),
    False(String),
    If(String),
    Else(String),
    Return(String),
}

impl Token {
    pub fn lookup_ident(ident: &str) -> Token {
        match ident {
            "fn" => Token::Function(String::from(ident)),
            "let" => Token::Let(String::from(ident)),
            "true" => Token::True(String::from(ident)),
            "false" => Token::False(String::from(ident)),
            "if" => Token::If(String::from(ident)),
            "else" => Token::Else(String::from(ident)),
            "return" => Token::Return(String::from(ident)),
            _ => Token::Ident(String::from(ident)),
        }
    }
}
