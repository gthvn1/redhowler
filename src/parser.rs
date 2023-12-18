// A parser takes input data and builds a data structure, an AST in our case,
// giving a structural representation of the input, checking for correct syntax
// in the process.
// We are constructing a recursive descent parser, which is a type of top-down
// parsing.
use crate::ast::{self};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

use std::collections::HashMap;

// Pratt parser idea is to associate parsing functions with token types instead
// of grammar rules. This is called precedence climbing.

type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn ast::Expression>>;
type InfixParseFn = fn(&mut Parser, Box<dyn ast::Expression>) -> Option<Box<dyn ast::Expression>>;

// Defining precedence
#[allow(dead_code)]
enum Precedence {
    Lowest = 1,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}

#[allow(dead_code)]
struct Parser<'l> {
    lexer: Lexer<'l>,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
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
            errors: Vec::new(),
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        // Register prefix parsing functions.
        p.register_prefix(TokenType::Ident, |parser| Parser::parse_identifier(parser));
        p.register_prefix(TokenType::Int, |parser| {
            Parser::parse_integer_literal(parser)
        });
        p.register_prefix(TokenType::Bang, |parser| {
            Parser::parse_prefix_expression(parser)
        });
        p.register_prefix(TokenType::Minus, |parser| {
            Parser::parse_prefix_expression(parser)
        });

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

    // ========================================================================
    // PARSING STATEMENTS
    // ========================================================================

    // This is the entry point for parsing a statement.
    // In the current implementation we only support let statements. So if the token
    // matches let we parse a let statement, otherwise we return None.
    // TODO: support others statements like return.
    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        match self.cur_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
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

        if !self.expect_peek(&TokenType::Ident) {
            return None;
        }

        stmt_builder.name(ast::Identifier::new(&self.cur_token));

        if !self.expect_peek(&TokenType::Assign) {
            return None;
        }

        // TODO: We're skipping the expressions until we encounter a semicolon.
        while !self.cur_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        let let_stmt = stmt_builder.build();
        Some(Box::new(let_stmt))
    }

    // This is the entry point for parsing a return statement.
    // Return statement is of the form: return <expression>;
    fn parse_return_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        // TODO: add builder. Currently we are skipping the expression.
        let stmt_builder = ast::ReturnStatementBuilder::new(&self.cur_token);

        self.next_token();

        // TODO: We're skipping the expressions until we encounter a semicolon.
        while !self.cur_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        let ret_stmt = stmt_builder.build();
        Some(Box::new(ret_stmt))
    }

    // This is the entry point for parsing an expression statement.
    fn parse_expression_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let mut stmt_builder = ast::ExpressionStatementBuilder::new(&self.cur_token);

        stmt_builder.expression(self.parse_expression(Precedence::Lowest));

        // Semi colon is optional. If we have it we skip it but if we don't have
        // it it is fine.
        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        let expr_stmt = stmt_builder.build();
        Some(Box::new(expr_stmt))
    }

    // ========================================================================
    // PARSING EXPRESSIONS
    // ========================================================================
    fn parse_expression(&mut self, _precedence: Precedence) -> Option<Box<dyn ast::Expression>> {
        let prefix_opt = self.prefix_parse_fns.get(&self.cur_token.token_type);

        // Check if we have a parsing function associated with the current token. If we
        // do we call it, otherwise we return None.
        if let Some(prefix) = prefix_opt {
            prefix(self)
        } else {
            let msg = format!(
                "No prefix parse function found for {:?}",
                self.cur_token.token_type
            );
            self.errors.push(msg);
            None
        }
    }

    fn parse_identifier(&mut self) -> Option<Box<dyn ast::Expression>> {
        Some(Box::new(ast::Identifier::new(&self.cur_token)))
    }

    fn parse_integer_literal(&mut self) -> Option<Box<dyn ast::Expression>> {
        return if let Ok(value) = self.cur_token.literal.parse::<i64>() {
            let lit = ast::IntegerLiteral::new(&self.cur_token, value);
            Some(Box::new(lit))
        } else {
            let msg = format!(
                "Could not parse {} as integer",
                self.cur_token.literal.as_str()
            );
            self.errors.push(msg);
            None
        };
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn ast::Expression>> {
        let mut expr_builder = ast::PrefixExpressionBuilder::new(&self.cur_token);
        expr_builder.operator(self.cur_token.literal.clone());

        self.next_token();

        expr_builder.right(self.parse_expression(Precedence::Prefix));

        Some(Box::new(expr_builder.build()))
    }

    // ========================================================================
    // HELPERS FUNCTIONS
    // ========================================================================

    // Advance the lexer by one token and update the current and peek tokens.
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    // Check if the current token is of the expected type.
    fn cur_token_is(&self, token_type: &TokenType) -> bool {
        self.cur_token.token_type == *token_type
    }

    // Check if the next token is of the expected type.
    fn peek_token_is(&self, token_type: &TokenType) -> bool {
        self.peek_token.token_type == *token_type
    }

    // If the next token is the expected one then we advance to next token
    // and return true, otherwise we don't read the next token and return false.
    fn expect_peek(&mut self, token_type: &TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            true
        } else {
            self.peek_error(token_type);
            false
        }
    }

    fn peek_error(&mut self, token_type: &TokenType) {
        let msg = format!(
            "Expected next token to be {:?}, got {:?} instead",
            *token_type, self.peek_token.token_type
        );
        self.errors.push(msg);
    }

    fn register_prefix(&mut self, token_type: TokenType, func: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, func);
    }

    fn register_infix(&mut self, token_type: TokenType, func: InfixParseFn) {
        self.infix_parse_fns.insert(token_type, func);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ExpressionStatement, LetStatement, PrefixExpression};

    #[test]
    fn test_parsing_prefix_expressions() {
        #[allow(dead_code)]
        struct PrefixTest {
            input: &'static str,
            operator: &'static str,
            value: i64,
        }

        let prefix_tests = vec![
            PrefixTest {
                input: "!5;",
                operator: "!",
                value: 5,
            },
            PrefixTest {
                input: "-15;",
                operator: "-",
                value: 15,
            },
        ];

        prefix_tests.iter().for_each(|tt| {
            let l = Lexer::new(tt.input);
            let mut p = Parser::new(l);

            let program = p.parse_program();

            // Check that parser didn't encounter any errors but before print
            // them if any.
            p.errors.iter().for_each(|e| eprintln!("{}", e));
            assert!(p.errors.is_empty());

            assert_eq!(program.statements.len(), 1);

            let stmt = program.statements.get(0).unwrap();
            if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
                if let Some(prefix_expr) = expr_stmt
                    .expression
                    .as_any()
                    .downcast_ref::<PrefixExpression>()
                {
                    assert_eq!(prefix_expr.operator, tt.operator);
                    //assert_eq!(prefix_expr.right., tt.value);
                } else {
                    panic!("Expected PrefixExpression");
                }
            } else {
                panic!("Expected ExpressionStatement");
            }
        });
    }

    #[test]
    fn test_integer_literal() {
        let input = "5;";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        // Check that parser didn't encounter any errors but before print them
        // if any.
        p.errors.iter().for_each(|e| eprintln!("{}", e));
        assert!(p.errors.is_empty());

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements.get(0).unwrap();
        assert_eq!(stmt.token_literal(), "5");
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        // Check that parser didn't encounter any errors but before print them
        // if any.
        p.errors.iter().for_each(|e| eprintln!("{}", e));
        assert!(p.errors.is_empty());

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements.get(0).unwrap();
        assert_eq!(stmt.token_literal(), "foobar");
    }

    #[test]
    fn test_return_statements() {
        let input = "
            return 5;
            return 10;
            return 993322;
        ";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();

        // Check that parser didn't encounter any errors but before print them
        // if any.
        p.errors.iter().for_each(|e| eprintln!("{}", e));
        assert!(p.errors.is_empty());

        assert_eq!(program.statements.len(), 3);
    }

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

        // Check that parser didn't encounter any errors but before print them
        // if any.
        p.errors.iter().for_each(|e| eprintln!("{}", e));
        assert!(p.errors.is_empty());

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
