// AST is Nodes connected each other.
use crate::token::Token;

// Every node in our AST has to implement the Node trait.
trait Node {
    // Returns the literal value of the token.
    fn token_literal(&self) -> String;
}

trait Statement: Node {
    // This dummy method is used for debugging.
    fn statement_node(&self) {}
}

trait Expression: Node {
    // This dummy method is used for debugging.
    fn expression_node(&self) {}
}

// This is the root of our AST.
struct Program {
    // 1. As we are using a trait as a type we need to use dynamic dispatch to
    // allow compiler to decide at runtime which type to use.
    // 2. Size of Statement is not known at compile time because different types
    // can implement the Statement. To solve that we can use Box smartpointer
    // that allocates the data on the Heap. So know the size is the size of the
    // smart pointer and it is known at compile time.
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}

// LetStatement is a statement that binds a value to a name.
// Let's have a look to `let x = 5 * 5;`
// - We need a node for the token `let`.
// - We need a node for the variable name `x`.
// - We need a node for the expression that produces the value.
struct LetStatement {
    token: Token,               // The token.LET token.
    name: Identifier,           // Node for the variable name.
    value: Box<dyn Expression>, // Node for the value expression.
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

// Identifier is a node that holds the name of the variable.
struct Identifier {
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
