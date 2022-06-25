use lexer::Lexer;
use token::Token;

type Answer = Vec<Token>;

#[test]
fn simple_literal() {
    let input = "=+(){},;";

    let expected: Answer = vec![
        Token::Assign,
        Token::Plus,
        Token::Lparen,
        Token::Rparen,
        Token::Lbrace,
        Token::Rbrace,
        Token::Comma,
        Token::Semicolon,
        Token::Eof,
    ];

    let mut l = Lexer::new(input);

    for exp in expected.iter() {
        let tok = l.next_token();

        assert_eq!(&tok, exp);
    }
}

#[test]
fn illegal_literal() {
    let input = "let a = ";

    let expected: Answer = vec![
        Token::Let,
        Token::Ident("a".to_string()),
        Token::Assign,
        Token::Illegal(''),
    ];

    let mut l = Lexer::new(input);

    for exp in expected.iter() {
        let tok = l.next_token();

        assert_eq!(&tok, exp);
    }
}

#[test]
fn integer() {
    let input = "let a = 1024";

    let expected: Answer = vec![
        Token::Let,
        Token::Ident("a".to_string()),
        Token::Assign,
        Token::Int(1024),
    ];

    let mut l = Lexer::new(input);

    for exp in expected.iter() {
        let tok = l.next_token();

        assert_eq!(&tok, exp);
    }
}

#[test]
fn let_and_func() {
    let input = "
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        ";

    let expected: Answer = vec![
        Token::Let,
        Token::Ident("five".to_string()),
        Token::Assign,
        Token::Int(5),
        Token::Semicolon,
        Token::Let,
        Token::Ident("ten".to_string()),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,
        Token::Let,
        Token::Ident("add".to_string()),
        Token::Assign,
        Token::Func,
        Token::Lparen,
        Token::Ident("x".to_string()),
        Token::Comma,
        Token::Ident("y".to_string()),
        Token::Rparen,
        Token::Lbrace,
        Token::Ident("x".to_string()),
        Token::Plus,
        Token::Ident("y".to_string()),
        Token::Semicolon,
        Token::Rbrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident("result".to_string()),
        Token::Assign,
        Token::Ident("add".to_string()),
        Token::Lparen,
        Token::Ident("five".to_string()),
        Token::Comma,
        Token::Ident("ten".to_string()),
        Token::Rparen,
        Token::Semicolon,
        Token::Eof,
    ];

    let mut l = Lexer::new(input);

    for exp in expected.iter() {
        let tok = l.next_token();

        assert_eq!(&tok, exp);
    }
}

#[test]
fn operators() {
    let input = "
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
            +-*/!
            1 < 3 > 2
            if (5 != 3) {
                return true;
            }
            else {
                return false;
            }
            1 == 1
        ";

    let expected: Answer = vec![
        Token::Let,
        Token::Ident("five".to_string()),
        Token::Assign,
        Token::Int(5),
        Token::Semicolon,
        Token::Let,
        Token::Ident("ten".to_string()),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,
        Token::Let,
        Token::Ident("add".to_string()),
        Token::Assign,
        Token::Func,
        Token::Lparen,
        Token::Ident("x".to_string()),
        Token::Comma,
        Token::Ident("y".to_string()),
        Token::Rparen,
        Token::Lbrace,
        Token::Ident("x".to_string()),
        Token::Plus,
        Token::Ident("y".to_string()),
        Token::Semicolon,
        Token::Rbrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident("result".to_string()),
        Token::Assign,
        Token::Ident("add".to_string()),
        Token::Lparen,
        Token::Ident("five".to_string()),
        Token::Comma,
        Token::Ident("ten".to_string()),
        Token::Rparen,
        Token::Semicolon,
        Token::Plus,
        Token::Minus,
        Token::Asterisk,
        Token::Slash,
        Token::Bang,
        Token::Int(1),
        Token::Lt,
        Token::Int(3),
        Token::Gt,
        Token::Int(2),
        Token::If,
        Token::Lparen,
        Token::Int(5),
        Token::Neq,
        Token::Int(3),
        Token::Rparen,
        Token::Lbrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::Rbrace,
        Token::Else,
        Token::Lbrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::Rbrace,
        Token::Int(1),
        Token::Eq,
        Token::Int(1),
        Token::Eof,
    ];

    let mut l = Lexer::new(input);

    for exp in expected.iter() {
        let tok = l.next_token();

        assert_eq!(&tok, exp);
    }
}
