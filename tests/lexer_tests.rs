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

        let expected = vec![
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Plus, "+"),
            Token::new(TokenType::LParen, "("),
            Token::new(TokenType::RParen, ")"),
            Token::new(TokenType::LBrace, "{"),
            Token::new(TokenType::RBrace, "}"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Semicolon, ";"),
        ];

        let tokens: Vec<_> = Lexer::new(input).collect();
        assert_eq!(tokens, expected);
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
        let expected = vec![
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Ident, "five"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Ident, "ten"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Ident, "add"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Function, "fn"),
            Token::new(TokenType::LParen, "("),
            Token::new(TokenType::Ident, "x"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Ident, "y"),
            Token::new(TokenType::RParen, ")"),
            Token::new(TokenType::LBrace, "{"),
            Token::new(TokenType::Ident, "x"),
            Token::new(TokenType::Plus, "+"),
            Token::new(TokenType::Ident, "y"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RBrace, "}"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Ident, "result"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Ident, "add"),
            Token::new(TokenType::LParen, "("),
            Token::new(TokenType::Ident, "five"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Ident, "ten"),
            Token::new(TokenType::RParen, ")"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Bang, "!"),
            Token::new(TokenType::Minus, "-"),
            Token::new(TokenType::Slash, "/"),
            Token::new(TokenType::Asterisk, "*"),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::LT, "<"),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::GT, ">"),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::If, "if"),
            Token::new(TokenType::LParen, "("),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::LT, "<"),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::RParen, ")"),
            Token::new(TokenType::LBrace, "{"),
            Token::new(TokenType::Return, "return"),
            Token::new(TokenType::True, "true"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RBrace, "}"),
            Token::new(TokenType::Else, "else"),
            Token::new(TokenType::LBrace, "{"),
            Token::new(TokenType::Return, "return"),
            Token::new(TokenType::False, "false"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RBrace, "}"),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::Equal, "=="),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::NotEqual, "!="),
            Token::new(TokenType::Int, "9"),
            Token::new(TokenType::Semicolon, ";"),
        ];

        let tokens: Vec<_> = Lexer::new(input).collect();
        assert_eq!(tokens, expected);
    }
}
