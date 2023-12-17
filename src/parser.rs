// A parser takes input data and builds a data structure, an AST in our case,
// giving a structural representation of the input, checking for correct syntax
// in the process.
// We are constructing a recursive descent parser, which is a type of top-down
// parsing.
use crate::ast::{self};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[allow(dead_code)]
struct Parser<'l> {
    lexer: Lexer<'l>,
    cur_token: Token,
    peek_token: Token,
}

// TODO: As we have the same lifetime as lexer maybe we can use a reference to
// Token instead of creating a new one and so creating new string. But maybe it is
// completely ok.
#[allow(dead_code)]
impl<'l> Parser<'l> {
    pub fn new(lexer: Lexer<'l>) -> Self {
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

        // Read two tokens, so cur_token and peek_token will be both set.
        p.next_token();
        p.next_token();
        p
    }

    // This is the entry point for parsing a program.
    // We keep parsing statements until we reach the end of the input.
    pub fn parse_program(&mut self) -> ast::Program {
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

    // This is the entry point for parsing a statement.
    // In the current implementation we only support let statements. So if the token
    // matches let we parse a let statement, otherwise we return None.
    // TODO: support others statements like return.
    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.cur_token.token_type {
            TokenType::Let => {
                eprintln!("Parsing let statement");
                self.parse_let_statement()
            }
            _ => None,
        }
    }

    // This is the entry point for parsing a let statement.
    // Let statement is of the form: let <identifier> = <expression>;
    // So we expect:
    // - let token
    // - identifier token
    // - assign token
    // - expression (TODO: parse expression, currently we skip it)
    // - semicolon token
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
        Some(Box::new(stmt))
    }

    // Advance the lexer by one token and update the current and peek tokens.
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    // Check if the current token is of the expected type.
    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.token_type == token_type
    }

    // Check if the next token is of the expected type.
    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    // If the next token is the expected one then we advance to next token
    // and return true, otherwise we don't read the next token and return false.
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
    use crate::ast::LetStatement;

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

        let expected_identifiers = vec!["x", "y", "foobar"];
        program
            .statements
            .iter()
            .zip(expected_identifiers.iter())
            .for_each(|(stmt, expected_ident)| {
                assert_eq!(stmt.token_literal(), "let");
                if let Some(let_stmt) = stmt.as_any().downcast_ref::<LetStatement>() {
                    assert_eq!(let_stmt.name(), *expected_ident);
                } else {
                    panic!("Expected LetStatement");
                }
            });
    }
}
