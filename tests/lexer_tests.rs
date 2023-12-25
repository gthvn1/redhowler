#[cfg(test)]
mod tests {

    use redhowler::interpreter::lexer::Lexer;
    use redhowler::interpreter::token::{Token, TokenType};

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
