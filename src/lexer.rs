use crate::token::{Token, TokenType};

struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: u8,               // current char under examination
}

impl Lexer {
    #![allow(dead_code)]
    fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0 as u8,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as u8;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let tok: Token = match self.ch {
            b'=' => new_token(TokenType::Assign, self.ch.to_string()),
            b';' => new_token(TokenType::Semicolon, self.ch.to_string()),
            b'(' => new_token(TokenType::LParen, self.ch.to_string()),
            b')' => new_token(TokenType::RParen, self.ch.to_string()),
            b',' => new_token(TokenType::Comma, self.ch.to_string()),
            b'+' => new_token(TokenType::Plus, self.ch.to_string()),
            b'{' => new_token(TokenType::LBrace, self.ch.to_string()),
            b'}' => new_token(TokenType::RBrace, self.ch.to_string()),
            0 => new_token(TokenType::EOF, self.ch.to_string()),
            _ => new_token(TokenType::Illegal, self.ch.to_string()),
        };
        self.read_char();
        tok
    }
}

pub fn new_token(token_type: TokenType, literal: String) -> Token {
    Token {
        token_type,
        literal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_token() {
        let tok = new_token(TokenType::Ident, "foobar".to_string());
        assert_eq!(tok.token_type, TokenType::Ident);
        assert_eq!(tok.literal, "foobar".to_string());
    }
}
