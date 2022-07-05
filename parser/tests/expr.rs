use ast::*;
use lexer::token::Token;
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

    assert_eq!(program.stmts.len(), expected.len());

    for (stmt, exp) in program.stmts.iter().zip(expected.iter()) {
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

    assert_eq!(program.stmts.len(), expected.len());

    for (stmt, exp) in program.stmts.iter().zip(expected.iter()) {
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

    assert_eq!(program.stmts.len(), expected.len());

    for (stmt, exp) in program.stmts.iter().zip(expected.iter()) {
        assert_eq!(stmt, exp);
    }
}
