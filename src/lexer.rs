use crate::token::Token;

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
            b'=' => Token::Assign(self.ch.to_string()),
            b';' => Token::Semicolon(self.ch.to_string()),
            b'(' => Token::LParen(self.ch.to_string()),
            b')' => Token::RParen(self.ch.to_string()),
            b',' => Token::Comma(self.ch.to_string()),
            b'+' => Token::Plus(self.ch.to_string()),
            b'{' => Token::LBrace(self.ch.to_string()),
            b'}' => Token::RBrace(self.ch.to_string()),
            0 => Token::EOF(self.ch.to_string()),
            _ => Token::Illegal(self.ch.to_string()),
        };
        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {}
