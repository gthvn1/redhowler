#[cfg(test)]
mod tests {

    use redhowler::interpreter::ast;
    use redhowler::interpreter::ast::{ExpressionStatement, LetStatement, PrefixExpression};
    use redhowler::interpreter::lexer::Lexer;
    use redhowler::interpreter::parser::Parser;

    #[test]
    fn test_operator_precedence_parsing() {
        #[allow(dead_code)]
        struct OperatorPrecedenceTest {
            input: &'static str,
            expected: &'static str,
        }

        let tests = vec![
            OperatorPrecedenceTest {
                input: "-a * b",
                expected: "((-a) * b)",
            },
            OperatorPrecedenceTest {
                input: "!-a",
                expected: "(!(-a))",
            },
            OperatorPrecedenceTest {
                input: "a + b + c",
                expected: "((a + b) + c)",
            },
            OperatorPrecedenceTest {
                input: "a + b - c",
                expected: "((a + b) - c)",
            },
            OperatorPrecedenceTest {
                input: "a * b * c",
                expected: "((a * b) * c)",
            },
            OperatorPrecedenceTest {
                input: "a * b / c",
                expected: "((a * b) / c)",
            },
            OperatorPrecedenceTest {
                input: "a + b / c",
                expected: "(a + (b / c))",
            },
            OperatorPrecedenceTest {
                input: "a + b * c + d / e - f",
                expected: "(((a + (b * c)) + (d / e)) - f)",
            },
            OperatorPrecedenceTest {
                input: "3 + 4; -5 * 5",
                expected: "(3 + 4)((-5) * 5)",
            },
            OperatorPrecedenceTest {
                input: "5 > 4 == 3 < 4",
                expected: "((5 > 4) == (3 < 4))",
            },
            OperatorPrecedenceTest {
                input: "5 < 4 != 3 > 4",
                expected: "((5 < 4) != (3 > 4))",
            },
            OperatorPrecedenceTest {
                input: "3 + 4 * 5 == 3 * 1 + 4 * 5",
                expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            },
        ];

        for tt in tests.iter() {
            let l = Lexer::from_str(tt.input);
            let mut p = Parser::from_lexer(l);

            let program = p.parse_program();

            // Check that parser didn't encounter any errors but before print
            // them if any.
            p.errors.iter().for_each(|e| eprintln!("{}", e));
            assert!(p.errors.is_empty());

            assert_eq!(program.string(), tt.expected);
        }
    }
    #[test]
    fn test_parsing_infix_expressions() {
        #[allow(dead_code)]
        struct InfixTest {
            input: &'static str,
            left_value: i64,
            operator: &'static str,
            right_value: i64,
        }

        let infix_tests = vec![
            InfixTest {
                input: "5 + 5;",
                left_value: 5,
                operator: "+",
                right_value: 5,
            },
            InfixTest {
                input: "5 - 5;",
                left_value: 5,
                operator: "-",
                right_value: 5,
            },
            InfixTest {
                input: "5 * 5;",
                left_value: 5,
                operator: "*",
                right_value: 5,
            },
            InfixTest {
                input: "5 / 5;",
                left_value: 5,
                operator: "/",
                right_value: 5,
            },
            InfixTest {
                input: "5 > 5;",
                left_value: 5,
                operator: ">",
                right_value: 5,
            },
            InfixTest {
                input: "5 < 5;",
                left_value: 5,
                operator: "<",
                right_value: 5,
            },
            InfixTest {
                input: "5 == 5;",
                left_value: 5,
                operator: "==",
                right_value: 5,
            },
            InfixTest {
                input: "5 != 5;",
                left_value: 5,
                operator: "!=",
                right_value: 5,
            },
        ];

        for tt in infix_tests.iter() {
            let l = Lexer::from_str(tt.input);
            let mut p = Parser::from_lexer(l);

            let program = p.parse_program();

            // Check that parser didn't encounter any errors but before print
            // them if any.
            p.errors.iter().for_each(|e| eprintln!("{}", e));
            assert!(p.errors.is_empty());

            assert_eq!(program.statements.len(), 1);

            let stmt = program.statements.get(0).unwrap();
            if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
                if let Some(infix_expr) = expr_stmt
                    .expression
                    .as_any()
                    .downcast_ref::<ast::InfixExpression>()
                {
                    let left = infix_expr
                        .left
                        .as_any()
                        .downcast_ref::<ast::IntegerLiteral>();
                    let right = infix_expr
                        .right
                        .as_any()
                        .downcast_ref::<ast::IntegerLiteral>();
                    assert_eq!(infix_expr.operator, tt.operator);
                    assert_eq!(left.unwrap().value(), tt.left_value);
                    assert_eq!(right.unwrap().value(), tt.right_value);
                } else {
                    panic!("Expected InfixExpression");
                }
            } else {
                panic!("Expected ExpressionStatement");
            }
        }
    }

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
            let l = Lexer::from_str(tt.input);
            let mut p = Parser::from_lexer(l);

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

        let l = Lexer::from_str(input);
        let mut p = Parser::from_lexer(l);

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

        let l = Lexer::from_str(input);
        let mut p = Parser::from_lexer(l);

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

        let l = Lexer::from_str(input);
        let mut p = Parser::from_lexer(l);

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

        let l = Lexer::from_str(input);
        let mut p = Parser::from_lexer(l);

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
