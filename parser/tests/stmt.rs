use ast::*;
use lexer::token::{Token, TokenType};
use parser::*;

macro_rules! loc {
    ($row: expr, $column: expr) => {
        Location::new($row, $column)
    };
}
macro_rules! strf {
    ($str: expr) => {
        String::from($str)
    };
}

fn error(curr: Token, code: &str, msg: String) -> String {
    format!(
        "Line {}:{} {} - {}, found {}",
        curr.row, curr.column, code, msg, curr
    )
}

#[test]
fn parse_empty_stmt() {
    let input = "
    
1
2;

3;;
;;;;a
;
    b
";

    let expected: Vec<Stmt> = vec![
        Stmt::Expr {
            expr: Expr::Int {
                loc:   loc!(3, 1),
                value: 1,
            },
        },
        Stmt::Expr {
            expr: Expr::Int {
                loc:   loc!(4, 1),
                value: 2,
            },
        },
        Stmt::Expr {
            expr: Expr::Int {
                loc:   loc!(6, 1),
                value: 3,
            },
        },
        Stmt::Expr {
            expr: Expr::Ident {
                loc:  loc!(7, 5),
                name: strf!("a"),
            },
        },
        Stmt::Expr {
            expr: Expr::Ident {
                loc:  loc!(9, 5),
                name: strf!("b"),
            },
        },
    ];

    let program = Parser::new(input).parse().unwrap();

    assert_eq!(program.stmts.len(), expected.len());

    for (stmt, exp) in program.stmts.iter().zip(expected.iter()) {
        assert_eq!(stmt, exp);
    }
}

#[test]
fn parse_let_stmt() {
    let input = "
let five = 5;
let ten1 = ten2;
";

    let expected: Vec<Stmt> = vec![
        Stmt::Let {
            name: Expr::Ident {
                loc:  loc!(2, 5),
                name: strf!("five"),
            },
            expr: Expr::Int {
                loc:   loc!(2, 12),
                value: 5,
            },
        },
        Stmt::Let {
            name: Expr::Ident {
                loc:  loc!(3, 5),
                name: strf!("ten1"),
            },
            expr: Expr::Ident {
                loc:  loc!(3, 12),
                name: strf!("ten2"),
            },
        },
    ];

    let program = Parser::new(input).parse().unwrap();

    assert_eq!(program.stmts.len(), expected.len());

    for (stmt, exp) in program.stmts.iter().zip(expected.iter()) {
        assert_eq!(stmt, exp);
    }
}

#[test]
fn parse_let_stmt_error() {
    let input = "
let five = +;
    let = ten2;
let b = 3;
let a 5;
";

    let expected: Vec<String> = vec![
        error(
            Token::new(TokenType::Plus, "+", 2, 12),
            "PAR:3001",
            format!("Cannot parse prefix"),
        ),
        error(
            Token::new(TokenType::Assign, "=", 3, 9),
            "PAR:3011",
            format!("expected {}", TokenType::Ident),
        ),
        error(
            Token::new(TokenType::Int, "5", 5, 7),
            "PAR:2012",
            format!("expected {}", TokenType::Assign),
        ),
    ];

    let program = Parser::new(input).parse().unwrap_err();

    assert_eq!(program.len(), expected.len());

    for (stmt, exp) in program.iter().zip(expected.iter()) {
        assert_eq!(stmt, exp);
    }
}

#[test]
fn parse_return_stmt() {
    let input = "
return 5;
    return a
    a
    return b;
";

    let expected: Vec<Stmt> = vec![
        Stmt::Return {
            expr: Expr::Int {
                loc:   loc!(2, 8),
                value: 5,
            },
        },
        Stmt::Return {
            expr: Expr::Ident {
                loc:  loc!(3, 12),
                name: strf!("a"),
            },
        },
        Stmt::Expr {
            expr: Expr::Ident {
                loc:  loc!(4, 5),
                name: strf!("a"),
            },
        },
        Stmt::Return {
            expr: Expr::Ident {
                loc:  loc!(5, 12),
                name: strf!("b"),
            },
        },
    ];

    let program = Parser::new(input).parse().unwrap();

    assert_eq!(program.stmts.len(), expected.len());

    for (stmt, exp) in program.stmts.iter().zip(expected.iter()) {
        assert_eq!(stmt, exp);
    }
}

#[test]
fn parse_expr_stmt() {
    let input = "
    a
    b;
    3
    4
";

    let expected: Vec<Stmt> = vec![
        Stmt::Expr {
            expr: Expr::Ident {
                loc:  loc!(2, 5),
                name: strf!("a"),
            },
        },
        Stmt::Expr {
            expr: Expr::Ident {
                loc:  loc!(3, 5),
                name: strf!("b"),
            },
        },
        Stmt::Expr {
            expr: Expr::Int {
                loc:   loc!(4, 5),
                value: 3,
            },
        },
        Stmt::Expr {
            expr: Expr::Int {
                loc:   loc!(5, 5),
                value: 4,
            },
        },
    ];

    let program = Parser::new(input).parse().unwrap();

    assert_eq!(program.stmts.len(), expected.len());

    for (stmt, exp) in program.stmts.iter().zip(expected.iter()) {
        assert_eq!(stmt, exp);
    }
}

#[test]
fn parse_block_stmts() {
    let input = "{
    let b = 3;
    ;;;;;b;;;;;
    4
    return b;
    }";

    let expected: Vec<Stmt> = vec![
        Stmt::Let {
            name: Expr::Ident {
                loc:  loc!(2, 9),
                name: strf!("b"),
            },
            expr: Expr::Int {
                loc:   loc!(2, 13),
                value: 3,
            },
        },
        Stmt::Expr {
            expr: Expr::Ident {
                loc:  loc!(3, 10),
                name: strf!("b"),
            },
        },
        Stmt::Expr {
            expr: Expr::Int {
                loc:   loc!(4, 5),
                value: 4,
            },
        },
        Stmt::Return {
            expr: Expr::Ident {
                loc:  loc!(5, 12),
                name: strf!("b"),
            },
        },
    ];

    let block_stmts = Parser::new(input).parse_block_stmts().unwrap();

    assert_eq!(block_stmts.len(), expected.len());

    for (stmt, exp) in block_stmts.iter().zip(expected.iter()) {
        assert_eq!(stmt, exp);
    }
}

/*

// #[test]
// fn test_return() {
//     let input = "
//             return five;
//             return ten;
//             return 123;
//             return add(five, ten);
//             return add(five, 15);
//         "
//     .to_string();

//     let program = Parser::parse(input);

//     assert_eq!(program.statements.len(), 5);

//     for stmt in program.statements {
//         match stmt {
//             ast::Statement::Return { token, value: _ } => {
//                 assert_eq!(token, token::RETURN);
//             }
//             _ => panic!("Not a Let statement"),
//         };
//     }
// }
//     #[test]
//     fn test_ident_expression() {
//         let input = "
//             foobar;
//         "
//         .to_string();

//         let program = Parser::parse(input);

//         assert_eq!(program.statements.len(), 1);

//         for stmt in program.statements {
//             match stmt {
//                 ast::Statement::Expr { token, expression } => {
//                     assert_eq!(token, token::IDENT);
//                     match expression {
//                         ast::Expression::Ident(ident) => {
//                             assert_eq!(ident.token, token::IDENT);
//                             assert_eq!(ident.value, "foobar");
//                         }
//                         _ => panic!("Not a Expression::Ident"),
//                     }
//                 }
//                 _ => panic!("Not a Expr statement"),
//             };
//         }
//     }

//     #[test]
//     fn test_integer_literal_expression() {
//         let input = "
//             12345;
//         "
//         .to_string();

//         let program = Parser::parse(input);

//         assert_eq!(program.statements.len(), 1);

//         for stmt in program.statements {
//             match stmt {
//                 ast::Statement::Expr { token, expression } => {
//                     assert_eq!(token, token::INT);
//                     match expression {
//                         ast::Expression::IntegerLiteral { token, value } => {
//                             assert_eq!(token, token::INT);
//                             assert_eq!(value, 12345);
//                         }
//                         _ => panic!("Not a Expression::IntegerLiteral"),
//                     }
//                 }
//                 _ => panic!("Not a Expr statement"),
//             };
//         }
//     }

//     #[test]
//     fn test_prefix_expression() {
//         let input = "
//             -15;
//             !5;
//             !true;
//             !false;
//         "
//         .to_string();

//         let expected: Vec<ast::Expression> = vec![
//             ast::Expression::Prefix {
//                 token:    token::MINUS,
//                 operator: String::from("-"),
//                 right:    Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 15,
//                 }),
//             },
//             ast::Expression::Prefix {
//                 token:    token::BANG,
//                 operator: String::from("!"),
//                 right:    Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 5,
//                 }),
//             },
//             ast::Expression::Prefix {
//                 token:    token::BANG,
//                 operator: String::from("!"),
//                 right:    Box::new(ast::Expression::Bool {
//                     token: token::TRUE,
//                     value: true,
//                 }),
//             },
//             ast::Expression::Prefix {
//                 token:    token::BANG,
//                 operator: String::from("!"),
//                 right:    Box::new(ast::Expression::Bool {
//                     token: token::FALSE,
//                     value: false,
//                 }),
//             },
//         ];

//         let program = Parser::parse(input);

//         assert_eq!(program.statements.len(), 4);

//         for (i, exp) in expected.iter().enumerate() {
//             let stmt = &program.statements[i];

//             match stmt {
//                 ast::Statement::Expr {
//                     token: _,
//                     expression,
//                 } => {
//                     assert_eq!(expression, exp);
//                 }
//                 _ => panic!("Not a Expr Statement"),
//             };
//         }
//     }

//     #[test]
//     fn test_infix_expression() {
//         let input = "
//             1 + 1;
//             1 - 1;
//             1 * 1;
//             1 / 1;
//             1 < 1;
//             1 > 1;
//             1 == 1;
//             1 != 1;
//         "
//         .to_string();

//         let expected: Vec<ast::Expression> = vec![
//             ast::Expression::Infix {
//                 token:    token::PLUS,
//                 left:     Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//                 operator: String::from("+"),
//                 right:    Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//             },
//             ast::Expression::Infix {
//                 token:    token::MINUS,
//                 left:     Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//                 operator: String::from("-"),
//                 right:    Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//             },
//             ast::Expression::Infix {
//                 token:    token::ASTERISK,
//                 left:     Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//                 operator: String::from("*"),
//                 right:    Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//             },
//             ast::Expression::Infix {
//                 token:    token::SLASH,
//                 left:     Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//                 operator: String::from("/"),
//                 right:    Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//             },
//             ast::Expression::Infix {
//                 token:    token::LT,
//                 left:     Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//                 operator: String::from("<"),
//                 right:    Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//             },
//             ast::Expression::Infix {
//                 token:    token::GT,
//                 left:     Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//                 operator: String::from(">"),
//                 right:    Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//             },
//             ast::Expression::Infix {
//                 token:    token::EQ,
//                 left:     Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//                 operator: String::from("=="),
//                 right:    Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//             },
//             ast::Expression::Infix {
//                 token:    token::NEQ,
//                 left:     Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//                 operator: String::from("!="),
//                 right:    Box::new(ast::Expression::IntegerLiteral {
//                     token: token::INT,
//                     value: 1,
//                 }),
//             },
//         ];

//         let program = Parser::parse(input);

//         assert_eq!(program.statements.len(), 8);

//         for (i, exp) in expected.iter().enumerate() {
//             let stmt = &program.statements[i];

//             match stmt {
//                 ast::Statement::Expr {
//                     token: _,
//                     expression,
//                 } => {
//                     assert_eq!(expression, exp);
//                 }
//                 _ => panic!("Not a Expr Statement"),
//             };
//         }
//     }

//     #[test]
//     fn test_operator_precedence_expression() {
//         let input = "
//             !-a;
//             -a * b;
//             a + b + c;
//             a + b - c;
//             a * b * c;
//             a * b / c;
//             a + b / c;
//             a + b * c + d / e - f;
//             5 > 4 != 3 < 4;
//             3 + 4 * 5 == 3 * 1 + 4 * 5;
//             3 > 5 == false;
//             true == 3 < 5;
//             -1 + ((2 + 3) - 4) * 5;
//             1 + (2 + 3) / 4;
//             -(1 + 1);
//             !(true == !false);
//             1 + add(1 * 1) + d;
//             add(1, 2, 3 * 4, sub(5 + 6 * 7, 8), 10 * 11);
//         "
//         .to_string();

//         let expected: Vec<&str> = vec![
//             "(!(-a))",
//             "((-a) * b)",
//             "((a + b) + c)",
//             "((a + b) - c)",
//             "((a * b) * c)",
//             "((a * b) / c)",
//             "(a + (b / c))",
//             "(((a + (b * c)) + (d / e)) - f)",
//             "((5 > 4) != (3 < 4))",
//             "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
//             "((3 > 5) == false)",
//             "(true == (3 < 5))",
//             "((-1) + (((2 + 3) - 4) * 5))",
//             "(1 + ((2 + 3) / 4))",
//             "(-(1 + 1))",
//             "(!(true == (!false)))",
//             "((1 + add((1 * 1))) + d)",
//             "add(1, 2, (3 * 4), sub((5 + (6 * 7)), 8), (10 * 11))",
//         ];

//         let program = Parser::parse(input);

//         assert_eq!(program.statements.len(), expected.len());

//         for (i, exp) in expected.iter().enumerate() {
//             let stmt = &program.statements[i];
//             assert_eq!(stmt.to_string(), *exp);
//         }
//     }

//     #[test]
//     fn test_boolean_expression() {
//         let input = "
//             true;
//             false;
//         "
//         .to_string();

//         let expected: Vec<ast::Expression> = vec![
//             ast::Expression::Bool {
//                 token: token::TRUE,
//                 value: true,
//             },
//             ast::Expression::Bool {
//                 token: token::FALSE,
//                 value: false,
//             },
//         ];

//         let program = Parser::parse(input);

//         assert_eq!(program.statements.len(), 2);

//         for (i, exp) in expected.iter().enumerate() {
//             let stmt = &program.statements[i];
//             match stmt {
//                 ast::Statement::Expr {
//                     token: _,
//                     expression,
//                 } => {
//                     assert_eq!(expression, exp);
//                 }
//                 _ => panic!("Not a Expr Statement"),
//             }
//         }
//     }

//     #[test]
//     fn test_if_expression() {
//         let input = "
//             if (x < y) { x }
//         "
//         .to_string();

//         let expected = ast::Expression::If {
//             token:       token::IF,
//             condition:   Box::new(ast::Expression::Infix {
//                 token:    token::LT,
//                 left:     Box::new(ast::Expression::Ident(ast::Identifier {
//                     token: token::IDENT,
//                     value: "x".to_owned(),
//                 })),
//                 operator: "<".to_owned(),
//                 right:    Box::new(ast::Expression::Ident(ast::Identifier {
//                     token: token::IDENT,
//                     value: "y".to_owned(),
//                 })),
//             }),
//             consequence: vec![ast::Statement::Expr {
//                 token:      token::IDENT,
//                 expression: ast::Expression::Ident(ast::Identifier {
//                     token: token::IDENT,
//                     value: "x".to_owned(),
//                 }),
//             }],
//             alternative: vec![],
//         };

//         let program = Parser::parse(input);

//         assert_eq!(program.statements.len(), 1);

//         let stmt = &program.statements[0];
//         match stmt {
//             ast::Statement::Expr {
//                 token: _,
//                 expression,
//             } => assert_eq!(*expression, expected),
//             _ => panic!("Not a Expr Statement"),
//         }
//     }

//     #[test]
//     fn test_if_else_expression() {
//         let input = "
//             if (x < y) { x } else { y }
//             if (x < y) { x } else { y }
//             if (x < y) { x } else { y };
//             if (x < y) { x } else { y };
//             if (x < y) { x } else { y }
//         "
//         .to_string();

//         let expected = ast::Expression::If {
//             token:       token::IF,
//             condition:   Box::new(ast::Expression::Infix {
//                 token:    token::LT,
//                 left:     Box::new(ast::Expression::Ident(ast::Identifier {
//                     token: token::IDENT,
//                     value: "x".to_owned(),
//                 })),
//                 operator: "<".to_owned(),
//                 right:    Box::new(ast::Expression::Ident(ast::Identifier {
//                     token: token::IDENT,
//                     value: "y".to_owned(),
//                 })),
//             }),
//             consequence: vec![ast::Statement::Expr {
//                 token:      token::IDENT,
//                 expression: ast::Expression::Ident(ast::Identifier {
//                     token: token::IDENT,
//                     value: "x".to_owned(),
//                 }),
//             }],
//             alternative: vec![ast::Statement::Expr {
//                 token:      token::IDENT,
//                 expression: ast::Expression::Ident(ast::Identifier {
//                     token: token::IDENT,
//                     value: "y".to_owned(),
//                 }),
//             }],
//         };

//         let program = Parser::parse(input);

//         assert_eq!(program.statements.len(), 5);

//         for stmt in &program.statements {
//             match stmt {
//                 ast::Statement::Expr {
//                     token: _,
//                     expression,
//                 } => assert_eq!(*expression, expected),
//                 _ => panic!("Not a Expr Statement"),
//             }
//         }
//     }

//     #[test]
//     fn test_function_expression() {
//         let input = "
//             fn(x, y, z) { return x + y + z; };
//             fn() { return 0; };
//             fn(x) { return x; };
//         "
//         .to_string();

//         let expected = vec![
//             ast::Expression::FunctionLiteral {
//                 token:      token::FUNC,
//                 parameters: vec![
//                     Identifier {
//                         token: token::IDENT,
//                         value: "x".to_owned(),
//                     },
//                     Identifier {
//                         token: token::IDENT,
//                         value: "y".to_owned(),
//                     },
//                     Identifier {
//                         token: token::IDENT,
//                         value: "z".to_owned(),
//                     },
//                 ],
//                 body:       vec![ast::Statement::Return {
//                     token: token::RETURN,
//                     value: ast::Expression::Infix {
//                         token:    token::PLUS,
//                         left:     Box::new(ast::Expression::Infix {
//                             token:    token::PLUS,
//                             left:     Box::new(ast::Expression::Ident(ast::Identifier {
//                                 token: token::IDENT,
//                                 value: "x".to_owned(),
//                             })),
//                             operator: "+".to_owned(),
//                             right:    Box::new(ast::Expression::Ident(ast::Identifier {
//                                 token: token::IDENT,
//                                 value: "y".to_owned(),
//                             })),
//                         }),
//                         operator: "+".to_owned(),
//                         right:    Box::new(ast::Expression::Ident(ast::Identifier {
//                             token: token::IDENT,
//                             value: "z".to_owned(),
//                         })),
//                     },
//                 }],
//             },
//             ast::Expression::FunctionLiteral {
//                 token:      token::FUNC,
//                 parameters: vec![],
//                 body:       vec![ast::Statement::Return {
//                     token: token::RETURN,
//                     value: ast::Expression::IntegerLiteral {
//                         token: token::INT,
//                         value: 0,
//                     },
//                 }],
//             },
//             ast::Expression::FunctionLiteral {
//                 token:      token::FUNC,
//                 parameters: vec![Identifier {
//                     token: token::IDENT,
//                     value: "x".to_owned(),
//                 }],
//                 body:       vec![ast::Statement::Return {
//                     token: token::RETURN,
//                     value: ast::Expression::Ident(ast::Identifier {
//                         token: token::IDENT,
//                         value: "x".to_owned(),
//                     }),
//                 }],
//             },
//         ];

//         let program = Parser::parse(input);

//         assert_eq!(program.statements.len(), expected.len());

//         for (i, stmt) in program.statements.iter().enumerate() {
//             match stmt {
//                 ast::Statement::Expr {
//                     token: _,
//                     expression,
//                 } => assert_eq!(*expression, expected[i]),
//                 _ => panic!("Not a Expr Statement"),
//             }
//         }
//     }
// */
