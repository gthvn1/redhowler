// AST is Nodes connected each other.
use crate::token::Token;
use std::any::Any;

// Every node in our AST has to implement the Node trait.
pub trait Node {
    // Returns the literal value of the token.
    fn token_literal(&self) -> String;
}

// Statement does not produce value.
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
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
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
