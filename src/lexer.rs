use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,      // Current position in input (points to current char).
    read_position: usize, // Current reading position in input (after current char).
    ch: char,             // Current char under examination.
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0 as char,
        };

        // Initialize the lexer by reading the first character before
        // returing.
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        println!("next_token ch: {}", self.ch);

        let token = self.ch;
        let mut literal = token.to_string();
        let token_type = match token {
            ';' => TokenType::Semicolon,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            ',' => TokenType::Comma,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '/' => TokenType::Slash,
            '*' => TokenType::Asterisk,
            '<' => TokenType::LT,
            '>' => TokenType::GT,
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
            '\0' => TokenType::EOF,
            '=' => {
                // Here we don't know yet if it assign or equal. We need to
                // peek next char to know. If it is an equal sign then we know
                // that we need to return an Equal token.
                if self.peek_char() == '=' {
                    self.read_char();
                    literal.push(self.ch);
                    TokenType::Equal
                } else {
                    TokenType::Assign
                }
            }
            '!' => {
                // Here we don't know yet if it assign or equal. We need to
                // peek next char to know. If it is an equal sign then we know
                // that we need to return a NotEqual token.
                if self.peek_char() == '=' {
                    self.read_char();
                    literal.push(self.ch);
                    TokenType::NotEqual
                } else {
                    TokenType::Bang
                }
            }
            _ => {
                if token.is_alphabetic() {
                    // read_identifier() returns a slice of the input string
                    // We return directly because we already did the self.read_char()
                    // so we don't want to do another one.
                    let ident = self.read_identifier();
                    return Token {
                        token_type: match ident {
                            "fn" => TokenType::Function,
                            "let" => TokenType::Let,
                            "true" => TokenType::True,
                            "false" => TokenType::False,
                            "if" => TokenType::If,
                            "else" => TokenType::Else,
                            "return" => TokenType::Return,
                            _ => TokenType::Ident,
                        },
                        literal: String::from(ident),
                    };
                } else if token.is_digit(10) {
                    // read_number() returns a new String from slice of input
                    // string. And as above, we return directly because we already
                    // did the self.read_char().
                    return Token {
                        token_type: TokenType::Int,
                        literal: String::from(self.read_number()),
                    };
                } else {
                    return Token {
                        token_type: TokenType::Illegal,
                        literal: token.to_string(),
                    };
                }
            }
        };

        self.read_char();
        Token {
            token_type,
            literal,
        }
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

    // Return the next character without advancing our position in the input.
    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            0 as char
        } else {
            self.input.as_bytes()[self.read_position] as char
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
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
    fn read_number(&mut self) -> &str {
        let pos = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        &self.input[pos..self.position]
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
    pub fn test_next_token() {
        let input = "=+(){},;";

        let tests = vec![
            Token {
                token_type: TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::Plus,
                literal: String::from("+"),
            },
            Token {
                token_type: TokenType::LParen,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::RParen,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::LBrace,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::RBrace,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::Comma,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::EOF,
                literal: String::from("\0"),
            },
        ];

        let mut l = Lexer::new(input);

        for tt in tests {
            let tok = l.next_token();
            assert_eq!(tok.token_type, tt.token_type);
            assert_eq!(tok.literal, tt.literal);
        }
    }

    #[test]
    fn test_next_token_source() {
        let input = "
            let five = 5;
            let ten = 10;
 
            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
            ";
        let tests = vec![
            Token {
                token_type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("five"),
            },
            Token {
                token_type: TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("ten"),
            },
            Token {
                token_type: TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("add"),
            },
            Token {
                token_type: TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::Function,
                literal: String::from("fn"),
            },
            Token {
                token_type: TokenType::LParen,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("x"),
            },
            Token {
                token_type: TokenType::Comma,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("y"),
            },
            Token {
                token_type: TokenType::RParen,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::LBrace,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("x"),
            },
            Token {
                token_type: TokenType::Plus,
                literal: String::from("+"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("y"),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::RBrace,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("result"),
            },
            Token {
                token_type: TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("add"),
            },
            Token {
                token_type: TokenType::LParen,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("five"),
            },
            Token {
                token_type: TokenType::Comma,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("ten"),
            },
            Token {
                token_type: TokenType::RParen,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Bang,
                literal: String::from("!"),
            },
            Token {
                token_type: TokenType::Minus,
                literal: String::from("-"),
            },
            Token {
                token_type: TokenType::Slash,
                literal: String::from("/"),
            },
            Token {
                token_type: TokenType::Asterisk,
                literal: String::from("*"),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::LT,
                literal: String::from("<"),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::GT,
                literal: String::from(">"),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::If,
                literal: String::from("if"),
            },
            Token {
                token_type: TokenType::LParen,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::LT,
                literal: String::from("<"),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::RParen,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::LBrace,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::Return,
                literal: String::from("return"),
            },
            Token {
                token_type: TokenType::True,
                literal: String::from("true"),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::RBrace,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::Else,
                literal: String::from("else"),
            },
            Token {
                token_type: TokenType::LBrace,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::Return,
                literal: String::from("return"),
            },
            Token {
                token_type: TokenType::False,
                literal: String::from("false"),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::RBrace,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::Equal,
                literal: String::from("=="),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::NotEqual,
                literal: String::from("!="),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("9"),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::EOF,
                literal: String::from("\0"),
            },
        ];

        let mut l = Lexer::new(&input);
        for tt in tests {
            let tok = l.next_token();
            assert_eq!(tok.literal, tt.literal);
            assert_eq!(tok.token_type, tt.token_type);
        }
    }
}
