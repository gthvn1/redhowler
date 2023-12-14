use crate::token::Token;

struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char,             // current char under examination
}

impl Lexer {
    #![allow(dead_code)]
    fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0 as char,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let tok: Token = match self.ch {
            '=' => Token::Assign(self.ch.to_string()),
            ';' => Token::Semicolon(self.ch.to_string()),
            '(' => Token::LParen(self.ch.to_string()),
            ')' => Token::RParen(self.ch.to_string()),
            ',' => Token::Comma(self.ch.to_string()),
            '+' => Token::Plus(self.ch.to_string()),
            '{' => Token::LBrace(self.ch.to_string()),
            '}' => Token::RBrace(self.ch.to_string()),
            '\0' => Token::EOF(self.ch.to_string()),
            _ => Token::Illegal(self.ch.to_string()),
        };
        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        let tests = vec![
            Token::Assign('='.to_string()),
            Token::Plus(String::from("+")),
            Token::LParen(String::from("(")),
            Token::RParen(String::from(")")),
            Token::LBrace(String::from("{")),
            Token::RBrace(String::from("}")),
            Token::Comma(String::from(",")),
            Token::Semicolon(String::from(";")),
            Token::EOF(String::from("\0")),
        ];
        let mut l = Lexer::new(input);
        for tt in tests {
            let tok = l.next_token();
            assert_eq!(tok, tt);
        }
    }
}
