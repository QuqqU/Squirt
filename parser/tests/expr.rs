use ast::*;
use lexer::Token;
use parser::*;

macro_rules! strf {
    ($str: expr) => {
        String::from($str)
    };
}

fn _error(curr: Token, code: &str, msg: String) -> String {
    format!(
        "Line {}:{} {} - {}, found {}",
        curr.row, curr.column, code, msg, curr
    )
}

#[test]
fn parse_ident() {
    let input = "
    aaa;
    bbb
    ccc;
    ddd
    eee
";

    let expected: Vec<Stmt> = vec![
        Stmt::Expr {
            expr: Expr::Ident {
                loc:  loc!(2, 5),
                name: strf!("aaa"),
            },
        },
        Stmt::Expr {
            expr: Expr::Ident {
                loc:  loc!(3, 5),
                name: strf!("bbb"),
            },
        },
        Stmt::Expr {
            expr: Expr::Ident {
                loc:  loc!(4, 5),
                name: strf!("ccc"),
            },
        },
        Stmt::Expr {
            expr: Expr::Ident {
                loc:  loc!(5, 5),
                name: strf!("ddd"),
            },
        },
        Stmt::Expr {
            expr: Expr::Ident {
                loc:  loc!(6, 5),
                name: strf!("eee"),
            },
        },
    ];

    let program = Parser::new(input).parse().unwrap();

    assert_eq!(program.stmts.0.len(), expected.len());

    for (stmt, exp) in program.stmts.0.iter().zip(expected.iter()) {
        assert_eq!(stmt, exp);
    }
}

#[test]
fn parse_int() {
    let input = "
    1;
    2
    3
    4;
    5;
";

    let expected: Vec<Stmt> = vec![
        Stmt::Expr {
            expr: Expr::Int {
                loc:   loc!(2, 5),
                value: 1,
            },
        },
        Stmt::Expr {
            expr: Expr::Int {
                loc:   loc!(3, 5),
                value: 2,
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
        Stmt::Expr {
            expr: Expr::Int {
                loc:   loc!(6, 5),
                value: 5,
            },
        },
    ];

    let program = Parser::new(input).parse().unwrap();

    assert_eq!(program.stmts.0.len(), expected.len());

    for (stmt, exp) in program.stmts.0.iter().zip(expected.iter()) {
        assert_eq!(stmt, exp);
    }
}

#[test]
fn parse_bool() {
    let input = "
    true
    true;
    false
    false;
";

    let expected: Vec<Stmt> = vec![
        Stmt::Expr {
            expr: Expr::Bool {
                loc:   loc!(2, 5),
                value: true,
            },
        },
        Stmt::Expr {
            expr: Expr::Bool {
                loc:   loc!(3, 5),
                value: true,
            },
        },
        Stmt::Expr {
            expr: Expr::Bool {
                loc:   loc!(4, 5),
                value: false,
            },
        },
        Stmt::Expr {
            expr: Expr::Bool {
                loc:   loc!(5, 5),
                value: false,
            },
        },
    ];

    let program = Parser::new(input).parse().unwrap();

    assert_eq!(program.stmts.0.len(), expected.len());

    for (stmt, exp) in program.stmts.0.iter().zip(expected.iter()) {
        assert_eq!(stmt, exp);
    }
}

#[test]
fn parse_if_expr() {
    let input = "
    if (1 == 2) {
        return 3;
    }
";

    let expected: Stmt = Stmt::Expr {
        expr: Expr::If {
            loc:         loc!(2, 5),
            condition:   Box::new(Expr::Infix {
                loc:      loc!(2, 11),
                left:     Box::new(Expr::Int {
                    loc:   loc!(2, 9),
                    value: 1,
                }),
                operator: InfixType::Eq,
                right:    Box::new(Expr::Int {
                    loc:   loc!(2, 14),
                    value: 2,
                }),
            }),
            consequence: BlockStmts(vec![Stmt::Return {
                expr: Expr::Int {
                    loc:   loc!(3, 16),
                    value: 3,
                },
            }]),
            alternative: BlockStmts(vec![]),
        },
    };

    let program = Parser::new(input).parse().unwrap();

    assert_eq!(program.stmts.0.len(), 1);
    assert_eq!(program.stmts.0.first().unwrap(), &expected);
}

#[test]
fn parse_if_else_expr() {
    let input = "
    if (1 == 2) {
        return 3;
    } else {
        return 5;
    }
";

    let expected: Stmt = Stmt::Expr {
        expr: Expr::If {
            loc:         loc!(2, 5),
            condition:   Box::new(Expr::Infix {
                loc:      loc!(2, 11),
                left:     Box::new(Expr::Int {
                    loc:   loc!(2, 9),
                    value: 1,
                }),
                operator: InfixType::Eq,
                right:    Box::new(Expr::Int {
                    loc:   loc!(2, 14),
                    value: 2,
                }),
            }),
            consequence: BlockStmts(vec![Stmt::Return {
                expr: Expr::Int {
                    loc:   loc!(3, 16),
                    value: 3,
                },
            }]),
            alternative: BlockStmts(vec![Stmt::Return {
                expr: Expr::Int {
                    loc:   loc!(5, 16),
                    value: 5,
                },
            }]),
        },
    };

    let program = Parser::new(input).parse().unwrap();

    assert_eq!(program.stmts.0.len(), 1);
    assert_eq!(program.stmts.0.first().unwrap(), &expected);
}

#[test]
fn parse_func_literal_expr() {
    let input = "
    let f = fn(a, b, c) {
        a = 1;
        b = 2;
        return c = 3;
    };
";

    let expected: Stmt = Stmt::Let {
        name: Expr::Ident {
            loc:  loc!(2, 9),
            name: "f".to_string(),
        },
        expr: Expr::FuncLiteral {
            loc:        loc!(2, 13),
            parameters: Params(vec![
                Expr::Ident {
                    loc:  loc!(2, 16),
                    name: "a".to_string(),
                },
                Expr::Ident {
                    loc:  loc!(2, 19),
                    name: "b".to_string(),
                },
                Expr::Ident {
                    loc:  loc!(2, 22),
                    name: "c".to_string(),
                },
            ]),
            body:       BlockStmts(vec![
                Stmt::Expr {
                    expr: Expr::Infix {
                        loc:      loc!(3, 11),
                        left:     Box::new(Expr::Ident {
                            loc:  loc!(3, 9),
                            name: "a".to_string(),
                        }),
                        operator: InfixType::Assign,
                        right:    Box::new(Expr::Int {
                            loc:   loc!(3, 13),
                            value: 1,
                        }),
                    },
                },
                Stmt::Expr {
                    expr: Expr::Infix {
                        loc:      loc!(4, 11),
                        left:     Box::new(Expr::Ident {
                            loc:  loc!(4, 9),
                            name: "b".to_string(),
                        }),
                        operator: InfixType::Assign,
                        right:    Box::new(Expr::Int {
                            loc:   loc!(4, 13),
                            value: 2,
                        }),
                    },
                },
                Stmt::Return {
                    expr: Expr::Infix {
                        loc:      loc!(5, 18),
                        left:     Box::new(Expr::Ident {
                            loc:  loc!(5, 16),
                            name: "c".to_string(),
                        }),
                        operator: InfixType::Assign,
                        right:    Box::new(Expr::Int {
                            loc:   loc!(5, 20),
                            value: 3,
                        }),
                    },
                },
            ]),
        },
    };

    let program = Parser::new(input).parse().unwrap();

    assert_eq!(program.stmts.0.len(), 1);
    assert_eq!(program.stmts.0.first().unwrap(), &expected);
}

#[test]
fn parse_func_call_expr() {
    let input = "
    let f = abc(a, 2, c + 3);
";

    let expected: Stmt = Stmt::Let {
        name: Expr::Ident {
            loc:  loc!(2, 9),
            name: "f".to_string(),
        },
        expr: Expr::FuncCall {
            loc:   loc!(2, 16),
            ident: Box::new(Expr::Ident {
                loc:  loc!(2, 13),
                name: "abc".to_string(),
            }),
            args:  Args(vec![
                Expr::Ident {
                    loc:  loc!(2, 17),
                    name: "a".to_string(),
                },
                Expr::Int {
                    loc:   loc!(2, 20),
                    value: 2,
                },
                Expr::Infix {
                    loc:      loc!(2, 25),
                    left:     Box::new(Expr::Ident {
                        loc:  loc!(2, 23),
                        name: "c".to_string(),
                    }),
                    operator: InfixType::Plus,
                    right:    Box::new(Expr::Int {
                        loc:   loc!(2, 27),
                        value: 3,
                    }),
                },
            ]),
        },
    };

    let program = Parser::new(input).parse().unwrap();

    assert_eq!(program.stmts.0.len(), 1);
    assert_eq!(program.stmts.0.first().unwrap(), &expected);
}
