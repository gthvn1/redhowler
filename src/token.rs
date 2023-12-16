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
    pub fn literal(&self) -> String {
        match self {
            Token::Illegal(literal) => literal.clone(),
            Token::EOF(literal) => literal.clone(),
            Token::Ident(literal) => literal.clone(),
            Token::Int(literal) => literal.clone(),
            Token::Assign(literal) => literal.clone(),
            Token::Plus(literal) => literal.clone(),
            Token::Minus(literal) => literal.clone(),
            Token::Bang(literal) => literal.clone(),
            Token::Asterisk(literal) => literal.clone(),
            Token::Slash(literal) => literal.clone(),
            Token::LT(literal) => literal.clone(),
            Token::GT(literal) => literal.clone(),
            Token::Equal(literal) => literal.clone(),
            Token::NotEqual(literal) => literal.clone(),
            Token::Comma(literal) => literal.clone(),
            Token::Semicolon(literal) => literal.clone(),
            Token::LParen(literal) => literal.clone(),
            Token::RParen(literal) => literal.clone(),
            Token::LBrace(literal) => literal.clone(),
            Token::RBrace(literal) => literal.clone(),
            Token::Function(literal) => literal.clone(),
            Token::Let(literal) => literal.clone(),
            Token::True(literal) => literal.clone(),
            Token::False(literal) => literal.clone(),
            Token::If(literal) => literal.clone(),
            Token::Else(literal) => literal.clone(),
            Token::Return(literal) => literal.clone(),
        }
    }
}
