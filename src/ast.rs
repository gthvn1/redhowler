// AST is Nodes connected each other.
use crate::token::Token;
use std::any::Any;

// Every node in our AST has to implement the Node trait.
pub trait Node {
    // Returns the literal value of the token.
    fn token_literal(&self) -> String;
    // print AST nodes for debugging and to compare them with other AST nodes.
    fn string(&self) -> String;
}

// Statement does not produce value.
// We will have
//   - LetStatement
//   - ReturnStatement
//   - ExpressionStatement: An expression statement is one that evaluates an
//   expression and ignores its result
pub trait Statement: Node {
    // This dummy method is used for debugging.
    fn statement_node(&self);
    fn as_any(&self) -> &dyn Any;
}

// Expression produces value.
pub trait Expression: Node {
    // This dummy method is used for debugging.
    fn expression_node(&self) {}
}

// ============================================================================
// PROGRAM
// ============================================================================
// This is the root of our AST.
#[allow(dead_code)]
pub struct Program {
    // 1. As we are using a trait as a type we need to use dynamic dispatch to
    // allow compiler to decide at runtime which type to use.
    // 2. Size of Statement is not known at compile time because different types
    // can implement the Statement. To solve that we can use Box smartpointer
    // that allocates the data on the Heap. So know the size is the size of the
    // smart pointer and it is known at compile time.
    pub statements: Vec<Box<dyn Statement>>,
}

#[allow(dead_code)]
impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn push(&mut self, stmt: Box<dyn Statement>) {
        self.statements.push(stmt);
    }

    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }

    pub fn string(&self) -> String {
        let mut out = String::new();
        for stmt in &self.statements {
            out.push_str(&stmt.string());
        }
        out
    }
}

// ============================================================================
// LET STATEMENT
// ============================================================================
// LetStatement binds a value to a name.
// Let's have a look to `let x = 5 * 5;`
// - We need a node for the token `let`.
// - We need a node for the variable name `x`.
// - We need a node for the expression that produces the value.

#[allow(dead_code)]
pub struct LetStatementBuilder {
    token: Token,
    name: Option<Identifier>,
    //value: Option<Box<dyn Expression>>,
}

#[allow(dead_code)]
impl LetStatementBuilder {
    pub fn new(token: &Token) -> Self {
        LetStatementBuilder {
            token: token.clone(),
            name: None,
            //value: None,
        }
    }

    pub fn name(&mut self, name: Identifier) {
        self.name = Some(name);
    }

    pub fn build(self) -> LetStatement {
        LetStatement {
            token: self.token,
            name: self.name.unwrap(),
            //value: self.value.unwrap(),
        }
    }
}

#[allow(dead_code)]
pub struct LetStatement {
    token: Token, // The token.LET token.
    name: Identifier,
    //value: Box<dyn Expression>, // TODO: Implement Expression.
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.value);
        out.push_str(" = ");
        // TODO: Add expresionn when implemented
        //out.push_str(&self.value.string());
        out.push_str("<expression will go here>");
        out.push_str(";");
        out
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[allow(dead_code)]
impl LetStatement {
    pub fn name(&self) -> &str {
        self.name.value.as_str()
    }
}

// ============================================================================
// RETURN STATEMENT
// ============================================================================
#[allow(dead_code)]
pub struct ReturnStatementBuilder {
    token: Token,
    //return_value: Option<Box<dyn Expression>>,
}

impl ReturnStatementBuilder {
    pub fn new(token: &Token) -> Self {
        ReturnStatementBuilder {
            token: token.clone(),
            //return_value: None,
        }
    }

    pub fn build(self) -> ReturnStatement {
        ReturnStatement {
            token: self.token,
            //return_value: self.return_value.unwrap(),
        }
    }
}

#[allow(dead_code)]

pub struct ReturnStatement {
    pub token: Token, // The token.RETURN token.
                      //pub return_value: Box<dyn Expression>, // TODO: Implement Expression.
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out.push_str(" ");
        // TODO: Add expresionn when implemented
        out.push_str("<return value will go here>");
        out.push_str(";");
        out
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ============================================================================
// EXPRESSION STATEMENT
// ============================================================================
#[allow(dead_code)]
pub struct ExpressionStatementBuilder {
    token: Token,
    expression: Option<Box<dyn Expression>>,
}

#[allow(dead_code)]
impl ExpressionStatementBuilder {
    pub fn new(token: &Token) -> Self {
        ExpressionStatementBuilder {
            token: token.clone(),
            expression: None,
        }
    }

    pub fn expression(&mut self, expression: Option<Box<dyn Expression>>) {
        self.expression = expression;
    }

    pub fn build(self) -> ExpressionStatement {
        ExpressionStatement {
            token: self.token,
            expression: self.expression.unwrap(),
        }
    }
}

pub struct ExpressionStatement {
    pub token: Token, // The first token of the expression.
    pub expression: Box<dyn Expression>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.token_literal());
        out
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ============================================================================
// IDENTIFIER
// ============================================================================
// Identifier is a node that holds the name of the variable.
#[allow(dead_code)]
pub struct Identifier {
    token: Token,  // The token.IDENT token.
    value: String, // The value of the identifier.
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

#[allow(dead_code)]
impl Identifier {
    pub fn new(token: &Token) -> Self {
        Identifier {
            token: token.clone(),
            value: token.literal(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_let_statement() {
        let mut p = Program::new();

        // Build LetStatement
        let mut builder = LetStatementBuilder::new(&Token {
            token_type: TokenType::Let,
            literal: "let".to_string(),
        });

        // Add name
        builder.name(Identifier::new(&Token {
            token_type: TokenType::Ident,
            literal: "myVar".to_string(),
        }));

        // TODO: add expression
        let stmt = builder.build();
        p.push(Box::new(stmt));

        assert_eq!(p.string(), "let myVar = <expression will go here>;");
    }
}
