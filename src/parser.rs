// A parser takes input data and builds a data structure, an AST in our case,
// giving a structural representation of the input, checking for correct syntax
// in the process.
// We are constructing a recursive descent parser, which is a type of top-down
// parsing.
use crate::ast::{self, Node};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[allow(dead_code)]
struct Parser<'l> {
    lexer: Lexer<'l>,
    cur_token: Token,
    peek_token: Token,
}

#[allow(dead_code)]
impl<'l> Parser<'l> {
    fn new(lexer: Lexer<'l>) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: Token {
                token_type: TokenType::Illegal,
                literal: String::from("Dummy"),
            },
            peek_token: Token {
                token_type: TokenType::Illegal,
                literal: String::from("Dummy"),
            },
        };

        // Read two tokens, so cur_token and peek_token are both set.
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::new();

        while self.cur_token.token_type != TokenType::EOF {
            let stmt_opt = self.parse_statement();
            if let Some(stmt) = stmt_opt {
                program.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.cur_token.token_type {
            TokenType::Let => {
                eprintln!("Parsing let statement");
                self.parse_let_statement()
            }
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let mut stmt_builder = ast::LetStatementBuilder::new(&self.cur_token);

        if !self.expect_peek(TokenType::Ident) {
            eprintln!("Expected identifier, got {}", self.peek_token.literal());
            return None;
        }

        stmt_builder.name(ast::Identifier::new(&self.cur_token));

        if !self.expect_peek(TokenType::Assign) {
            eprintln!("Expected assign, got {}", self.peek_token.literal());
            return None;
        }

        // TODO: We're skipping the expressions until we encounter a semicolon.
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        let stmt = stmt_builder.build();
        eprintln!("Let statement: {}", stmt.token_literal());
        Some(Box::new(stmt))
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    // If the next token is of expected type then we advance to next token
    // and return true, otherwise we return false.
    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = "
            let x = 5;
            let y = 10;
            let foobar = 838383;
        ";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        program.statements.iter().for_each(|stmt| {
            eprintln!("{}", stmt.token_literal());
        });

        assert_eq!(program.statements.len(), 3);
    }
}
