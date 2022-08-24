use ast::*;
use lexer::{Lexer, Token, TokenType};
use parser::*;

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

    let tokens = Lexer::tokenize(input);
    let ast = Parser::parse(tokens).unwrap();

    assert_eq!(ast.program.len(), expected.len());

    for (stmt, exp) in ast.program.iter().zip(expected.iter()) {
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

    let tokens = Lexer::tokenize(input);
    let ast = Parser::parse(tokens).unwrap();

    assert_eq!(ast.program.len(), expected.len());

    for (stmt, exp) in ast.program.iter().zip(expected.iter()) {
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
            format!("expected {:?}", TokenType::Ident),
        ),
        error(
            Token::new(TokenType::Int, "5", 5, 7),
            "PAR:2012",
            format!("expected {:?}", TokenType::Assign),
        ),
    ];

    let tokens = Lexer::tokenize(input);
    let err = Parser::parse(tokens).unwrap_err();

    assert_eq!(err.len(), expected.len());

    for (msg, exp) in err.iter().zip(expected.iter()) {
        assert_eq!(msg, exp);
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

    let tokens = Lexer::tokenize(input);
    let ast = Parser::parse(tokens).unwrap();

    assert_eq!(ast.program.len(), expected.len());

    for (stmt, exp) in ast.program.iter().zip(expected.iter()) {
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

    let tokens = Lexer::tokenize(input);
    let ast = Parser::parse(tokens).unwrap();

    assert_eq!(ast.program.len(), expected.len());

    for (stmt, exp) in ast.program.iter().zip(expected.iter()) {
        assert_eq!(stmt, exp);
    }
}

// #[test]
// fn parse_block_stmts() {
//     let input = "{
//     let b = 3;
//     ;;;;;b;;;;;
//     4
//     return b;
//     }";

//     let expected: Vec<Stmt> = vec![
//         Stmt::Let {
//             name: Expr::Ident {
//                 loc:  loc!(2, 9),
//                 name: strf!("b"),
//             },
//             expr: Expr::Int {
//                 loc:   loc!(2, 13),
//                 value: 3,
//             },
//         },
//         Stmt::Expr {
//             expr: Expr::Ident {
//                 loc:  loc!(3, 10),
//                 name: strf!("b"),
//             },
//         },
//         Stmt::Expr {
//             expr: Expr::Int {
//                 loc:   loc!(4, 5),
//                 value: 4,
//             },
//         },
//         Stmt::Return {
//             expr: Expr::Ident {
//                 loc:  loc!(5, 12),
//                 name: strf!("b"),
//             },
//         },
//     ];

//     let tokens = Lexer::tokenize(input);
//     let ast = Parser::parse(tokens).unwrap();

//     let block_stmts = Parser::new(tokens).parse_block_stmts().unwrap();

//     assert_eq!(block_stmts.len(), expected.len());

//     for (stmt, exp) in block_stmts.iter().zip(expected.iter()) {
//         assert_eq!(stmt, exp);
//     }
// }
