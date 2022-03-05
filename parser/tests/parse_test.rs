#[cfg(test)]
mod parser_tests {

    #[test]
    fn test_let() {
        let input = "
            let five = 5;
            let ten = 10;
            let result = add(five, ten);
        "
        .to_string();

        let expected_ident: Vec<&str> = vec!["five", "ten", "result"];

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), 3);

        for (i, exp) in expected_ident.iter().enumerate() {
            let stmt = &program.statements[i];

            match stmt {
                ast::Statement::Let { token, name, value } => {
                    assert_eq!(*token, token::LET);
                    assert_eq!(name.token, token::IDENT);
                    assert_eq!(name.value, *exp);
                }
                _ => panic!("Not a Let statement"),
            };
        }
    }

    #[test]
    fn test_return() {
        let input = "
            return five = 5;
            return ten;
            return 123;
            return add(five, ten);
            return add(five, 15);
        "
        .to_string();

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), 5);

        for stmt in program.statements {
            match stmt {
                ast::Statement::Return { token, value } => {
                    assert_eq!(token, token::RETURN);
                }
                _ => panic!("Not a Let statement"),
            };
        }
    }

    #[test]
    fn test_ident_expression() {
        let input = "
            foobar;
        "
        .to_string();

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), 1);

        for stmt in program.statements {
            match stmt {
                ast::Statement::Expr { token, expression } => {
                    assert_eq!(token, token::IDENT);
                    match expression {
                        ast::Expression::Ident(ident) => {
                            assert_eq!(ident.token, token::IDENT);
                            assert_eq!(ident.value, "foobar");
                        }
                        _ => panic!("Not a Expression::Ident"),
                    }
                }
                _ => panic!("Not a Expr statement"),
            };
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "
            12345;
        "
        .to_string();

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), 1);

        for stmt in program.statements {
            match stmt {
                ast::Statement::Expr { token, expression } => {
                    assert_eq!(token, token::INT);
                    match expression {
                        ast::Expression::IntegerLiteral { token, value } => {
                            assert_eq!(token, token::INT);
                            assert_eq!(value, 12345);
                        }
                        _ => panic!("Not a Expression::IntegerLiteral"),
                    }
                }
                _ => panic!("Not a Expr statement"),
            };
        }
    }

    #[test]
    fn test_prefix_expression() {
        let input = "
            -15;
            !5;
            !true;
            !false;
        "
        .to_string();

        let expected: Vec<ast::Expression> = vec![
            ast::Expression::Prefix {
                token:    token::MINUS,
                operator: String::from("-"),
                right:    Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 15,
                }),
            },
            ast::Expression::Prefix {
                token:    token::BANG,
                operator: String::from("!"),
                right:    Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 5,
                }),
            },
            ast::Expression::Prefix {
                token:    token::BANG,
                operator: String::from("!"),
                right:    Box::new(ast::Expression::Bool {
                    token: token::TRUE,
                    value: true,
                }),
            },
            ast::Expression::Prefix {
                token:    token::BANG,
                operator: String::from("!"),
                right:    Box::new(ast::Expression::Bool {
                    token: token::FALSE,
                    value: false,
                }),
            },
        ];

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), 4);

        for (i, exp) in expected.iter().enumerate() {
            let stmt = &program.statements[i];

            match stmt {
                ast::Statement::Expr { token, expression } => {
                    assert_eq!(expression, exp);
                }
                _ => panic!("Not a Expr Statement"),
            };
        }
    }

    #[test]
    fn test_infix_expression() {
        let input = "
            1 + 1;
            1 - 1;
            1 * 1;
            1 / 1;
            1 < 1;
            1 > 1;
            1 == 1;
            1 != 1;
        "
        .to_string();

        let expected: Vec<ast::Expression> = vec![
            ast::Expression::Infix {
                token:    token::PLUS,
                left:     Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
                operator: String::from("+"),
                right:    Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
            },
            ast::Expression::Infix {
                token:    token::MINUS,
                left:     Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
                operator: String::from("-"),
                right:    Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
            },
            ast::Expression::Infix {
                token:    token::ASTERISK,
                left:     Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
                operator: String::from("*"),
                right:    Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
            },
            ast::Expression::Infix {
                token:    token::SLASH,
                left:     Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
                operator: String::from("/"),
                right:    Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
            },
            ast::Expression::Infix {
                token:    token::LT,
                left:     Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
                operator: String::from("<"),
                right:    Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
            },
            ast::Expression::Infix {
                token:    token::GT,
                left:     Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
                operator: String::from(">"),
                right:    Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
            },
            ast::Expression::Infix {
                token:    token::EQ,
                left:     Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
                operator: String::from("=="),
                right:    Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
            },
            ast::Expression::Infix {
                token:    token::NEQ,
                left:     Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
                operator: String::from("!="),
                right:    Box::new(ast::Expression::IntegerLiteral {
                    token: token::INT,
                    value: 1,
                }),
            },
        ];

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), 8);

        for (i, exp) in expected.iter().enumerate() {
            let stmt = &program.statements[i];

            match stmt {
                ast::Statement::Expr {
                    token: _,
                    expression,
                } => {
                    assert_eq!(expression, exp);
                }
                _ => panic!("Not a Expr Statement"),
            };
        }
    }

    #[test]
    fn test_operator_precedence_expression() {
        let input = "
            !-a;
            -a * b;
            a + b + c;
            a + b - c;
            a * b * c;
            a * b / c;
            a + b / c;
            a + b * c + d / e - f;
            5 > 4 != 3 < 4;
            3 + 4 * 5 == 3 * 1 + 4 * 5;
            3 > 5 == false;
            true == 3 < 5;
            -1 + ((2 + 3) - 4) * 5;
            1 + (2 + 3) / 4;
            -(1 + 1);
            !(true == !false);
        "
        .to_string();

        let expected: Vec<&str> = vec![
            "(!(-a))",
            "((-a) * b)",
            "((a + b) + c)",
            "((a + b) - c)",
            "((a * b) * c)",
            "((a * b) / c)",
            "(a + (b / c))",
            "(((a + (b * c)) + (d / e)) - f)",
            "((5 > 4) != (3 < 4))",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            "((3 > 5) == false)",
            "(true == (3 < 5))",
            "((-1) + (((2 + 3) - 4) * 5))",
            "(1 + ((2 + 3) / 4))",
            "(-(1 + 1))",
            "(!(true == (!false)))",
        ];

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), expected.len());

        for (i, exp) in expected.iter().enumerate() {
            let stmt = &program.statements[i];
            assert_eq!(stmt.to_string(), *exp);
        }
    }

    #[test]
    fn test_boolean_expression() {
        let input = "
            true;
            false;
        "
        .to_string();

        let expected: Vec<ast::Expression> = vec![
            ast::Expression::Bool {
                token: token::TRUE,
                value: true,
            },
            ast::Expression::Bool {
                token: token::FALSE,
                value: false,
            },
        ];

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), 2);

        for (i, exp) in expected.iter().enumerate() {
            let stmt = &program.statements[i];
            match stmt {
                ast::Statement::Expr {
                    token: _,
                    expression,
                } => {
                    assert_eq!(expression, exp);
                }
                _ => panic!("Not a Expr Statement"),
            }
        }
    }
}
