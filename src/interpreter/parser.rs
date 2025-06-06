// A parser takes input data and builds a data structure, an AST in our case,
// giving a structural representation of the input, checking for correct syntax
// in the process.
// We are constructing a recursive descent parser, which is a type of top-down
// parsing.
use super::ast::{self};
use super::lexer::Lexer;
use super::token::{Token, TokenType};

use std::collections::HashMap;

// Pratt parser idea is to associate parsing functions with token types instead
// of grammar rules. This is called precedence climbing.

type PrefixParseFn = fn(&mut Parser) -> Option<Box<dyn ast::Expression>>;
type InfixParseFn = fn(&mut Parser, Box<dyn ast::Expression>) -> Option<Box<dyn ast::Expression>>;

// Defining precedence
#[allow(dead_code)]
#[derive(PartialOrd, PartialEq)]
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
pub struct Parser<'l> {
    lexer: Lexer<'l>,
    cur_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
}

// TODO: As we have the same lifetime as lexer maybe we can use a reference to
// Token instead of creating a new one and so creating new string. But maybe it is
// completely ok.
#[allow(dead_code)]
impl<'l> Parser<'l> {
    pub fn from_lexer(lexer: Lexer<'l>) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: Token::new(TokenType::Illegal, "Dummy"),
            peek_token: Token::new(TokenType::Illegal, "Dummy"),
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

        // Register infix parsing functions.
        p.register_infix(TokenType::Plus, |parser, left| {
            Parser::parse_infix_expression(parser, left)
        });
        p.register_infix(TokenType::Minus, |parser, left| {
            Parser::parse_infix_expression(parser, left)
        });
        p.register_infix(TokenType::Slash, |parser, left| {
            Parser::parse_infix_expression(parser, left)
        });
        p.register_infix(TokenType::Asterisk, |parser, left| {
            Parser::parse_infix_expression(parser, left)
        });
        p.register_infix(TokenType::Equal, |parser, left| {
            Parser::parse_infix_expression(parser, left)
        });
        p.register_infix(TokenType::NotEqual, |parser, left| {
            Parser::parse_infix_expression(parser, left)
        });
        p.register_infix(TokenType::LT, |parser, left| {
            Parser::parse_infix_expression(parser, left)
        });
        p.register_infix(TokenType::GT, |parser, left| {
            Parser::parse_infix_expression(parser, left)
        });

        // Read two tokens, so cur_token and peek_token will be both set.
        p.next_token();
        p.next_token();
        p
    }

    // This is the entry point for parsing a program.
    // We keep parsing statements until we reach the end of the input.
    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::default();

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
        // To be able to build it we pass a dummy expression.
        let dummy_expr = ast::Identifier::new(&self.cur_token);
        stmt_builder.value(Some(Box::new(dummy_expr)));

        while !self.cur_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        let let_stmt = stmt_builder.build();
        Some(Box::new(let_stmt))
    }

    // This is the entry point for parsing a return statement.
    // Return statement is of the form: return <expression>;
    fn parse_return_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        let mut stmt_builder = ast::ReturnStatementBuilder::new(&self.cur_token);

        self.next_token();

        // TODO: We're skipping the expressions until we encounter a semicolon.
        // To be able to build it we pass a dummy expression.
        let dummy_expr = ast::Identifier::new(&self.cur_token);
        stmt_builder.return_value(Some(Box::new(dummy_expr)));

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
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn ast::Expression>> {
        let prefix_opt = self.prefix_parse_fns.get(&self.cur_token.token_type);

        // Check if we have a parsing function associated with the current token. If we
        // do we call it, otherwise we return None.
        if let Some(prefix) = prefix_opt {
            let mut left_expr = prefix(self);

            while !self.peek_token_is(&TokenType::Semicolon) && precedence < self.peek_precedence()
            {
                let ipf = self
                    .infix_parse_fns
                    .get(&self.peek_token.token_type)
                    .cloned();
                match ipf {
                    Some(infix) => {
                        self.next_token();
                        left_expr = infix(self, left_expr.unwrap());
                    }
                    None => return left_expr,
                }
            }

            left_expr
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
        if let Ok(value) = self.cur_token.literal.parse::<i64>() {
            let lit = ast::IntegerLiteral::new(&self.cur_token, value);
            Some(Box::new(lit))
        } else {
            let msg = format!(
                "Could not parse {} as integer",
                self.cur_token.literal.as_str()
            );
            self.errors.push(msg);
            None
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn ast::Expression>> {
        let mut expr_builder = ast::PrefixExpressionBuilder::new(&self.cur_token);
        expr_builder.operator(self.cur_token.literal.clone());

        self.next_token();

        expr_builder.right(self.parse_expression(Precedence::Prefix));

        Some(Box::new(expr_builder.build()))
    }

    fn parse_infix_expression(
        &mut self,
        left: Box<dyn ast::Expression>,
    ) -> Option<Box<dyn ast::Expression>> {
        let mut expr_builder = ast::InfixExpressionBuilder::new(&self.cur_token);
        expr_builder.operator(self.cur_token.literal.clone());
        expr_builder.left(Some(left));

        let precedence: Precedence = self.cur_precedence();
        self.next_token();
        expr_builder.right(self.parse_expression(precedence));

        Some(Box::new(expr_builder.build()))
    }

    // ========================================================================
    // HELPERS FUNCTIONS
    // ========================================================================

    // Advance the lexer by one token and update the current and peek tokens.
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next().unwrap_or(Token::new(TokenType::EOF, ""));
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

    fn precedences(token_type: &TokenType) -> Precedence {
        match token_type {
            TokenType::Equal | TokenType::NotEqual => Precedence::Equals,
            TokenType::LT | TokenType::GT => Precedence::LessGreater,
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Slash | TokenType::Asterisk => Precedence::Product,
            TokenType::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn peek_precedence(&self) -> Precedence {
        Parser::precedences(&self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> Precedence {
        Parser::precedences(&self.cur_token.token_type)
    }
}
