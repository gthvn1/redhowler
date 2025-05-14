use super::token::{Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,      // Current position in input (points to current char).
    read_position: usize, // Current reading position in input (after current char).
    ch: char,             // Current char under examination.
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0 as char,
        };

        // Initialize the lexer by reading the first character before
        // returning.
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
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        &self.input[pos..self.position]
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

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
            '\0' => return None,
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
                    return Some(Token {
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
                    });
                } else if token.is_ascii_digit() {
                    // read_number() returns a new String from slice of input
                    // string. And as above, we return directly because we already
                    // did the self.read_char().
                    return Some(Token {
                        token_type: TokenType::Int,
                        literal: String::from(self.read_number()),
                    });
                } else {
                    TokenType::Illegal
                }
            }
        };

        self.read_char();

        Some(Token {
            token_type,
            literal,
        })
    }
}
