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

    // Read the next character and advance our position in the input string.
    // position points to the current char, read_position points to the next
    // char.
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

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
            _ => {
                if self.ch.is_alphabetic() {
                    // read_identifier() returns a slice of the input string
                    // We return directly because we already did the self.read_char()
                    // so we don't want to do another one.
                    return Token::lookup_ident(self.read_identifier());
                }

                if self.ch.is_digit(10) {
                    // read_number() returns a new String from slice of input
                    // string. And as above, we return directly because we already
                    // did the self.read_char().
                    return Token::Int(self.read_number());
                }

                Token::Illegal(self.ch.to_string())
            }
        };

        self.read_char();
        tok
    }

    // Return a slice of the input string from the current position until
    // the next non-alphabetic character.
    fn read_identifier(&mut self) -> &str {
        let pos = self.position;
        while self.ch.is_alphabetic() {
            self.read_char();
        }
        &self.input[pos..self.position]
    }

    // Return a slice of the number in base 10 from the current position.
    fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[pos..self.position].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace() {
        // Expected whitespace characters
        assert!('\n'.is_whitespace());
        assert!('\t'.is_whitespace());
        assert!(' '.is_whitespace());
        assert!('\r'.is_whitespace());

        // Expected not whitespace characters
        assert!(!'a'.is_whitespace());
        assert!(!'\0'.is_whitespace());
    }

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

    #[test]
    fn test_next_token_source() {
        let input = String::from(
            "
            let five = 5;
            let ten = 10;
 
            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            ",
        );
        let tests = vec![
            Token::Let(String::from("let")),
            Token::Ident(String::from("five")),
            Token::Assign(String::from("=")),
            Token::Int(String::from("5")),
            Token::Semicolon(String::from(";")),
            Token::Let(String::from("let")),
            Token::Ident(String::from("ten")),
            Token::Assign(String::from("=")),
            Token::Int(String::from("10")),
            Token::Semicolon(String::from(";")),
            Token::Let(String::from("let")),
            Token::Ident(String::from("add")),
            Token::Assign(String::from("=")),
            Token::Function(String::from("fn")),
            Token::LParen(String::from("(")),
            Token::Ident(String::from("x")),
            Token::Comma(String::from(",")),
            Token::Ident(String::from("y")),
            Token::RParen(String::from(")")),
            Token::LBrace(String::from("{")),
            Token::Ident(String::from("x")),
            Token::Plus(String::from("+")),
            Token::Ident(String::from("y")),
            Token::Semicolon(String::from(";")),
            Token::RBrace(String::from("}")),
            Token::Semicolon(String::from(";")),
            Token::Let(String::from("let")),
            Token::Ident(String::from("result")),
            Token::Assign(String::from("=")),
            Token::Ident(String::from("add")),
            Token::LParen(String::from("(")),
            Token::Ident(String::from("five")),
            Token::Comma(String::from(",")),
            Token::Ident(String::from("ten")),
            Token::RParen(String::from(")")),
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
