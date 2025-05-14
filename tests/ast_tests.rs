use redhowler::interpreter::ast;
use redhowler::interpreter::ast::{Identifier, LetStatementBuilder, Program};
use redhowler::interpreter::token::{Token, TokenType};

#[test]
fn test_let_statement() {
    let mut p = Program::default();

    // Build LetStatement
    let mut builder = LetStatementBuilder::new(&Token::new(TokenType::Let, "let"));

    // Add name
    builder.name(Identifier::new(&Token::new(TokenType::Ident, "myVar")));

    // Add value
    let id_token = Token::new(TokenType::Ident, "anotherVar");
    let id = ast::Identifier::new(&id_token);
    builder.value(Some(Box::new(id)));
    let stmt = builder.build();
    p.push(Box::new(stmt));

    assert_eq!(p.string(), "let myVar = anotherVar;");
}
