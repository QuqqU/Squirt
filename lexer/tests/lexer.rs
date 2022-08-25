use lexer::Lexer;
use token::{Token, TokenType};

type Answer = Vec<TokenType>;

#[test]
fn simple_literal() {
    let input = "=+(){},;";

    let expected: Answer = vec![
        TokenType::Assign,
        TokenType::Plus,
        TokenType::Lparen,
        TokenType::Rparen,
        TokenType::Lbrace,
        TokenType::Rbrace,
        TokenType::Comma,
        TokenType::Semicolon,
        TokenType::Eof,
    ];

    let mut l = Lexer::new(input);

    for exp in expected.iter() {
        let tok = l.next_token();

        assert_eq!(&tok.token_type, exp);
    }
}

#[test]
fn poison() {
    let input = "let a = ";

    let expected: Answer = vec![
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Poison,
    ];

    let mut l = Lexer::new(input);

    for exp in expected.iter() {
        let tok = l.next_token();

        assert_eq!(&tok.token_type, exp);
    }
}

#[test]
fn integer() {
    let input = "let a = 1024";

    let expected: Answer = vec![
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Int,
    ];

    let mut l = Lexer::new(input);

    for exp in expected.iter() {
        let tok = l.next_token();

        assert_eq!(&tok.token_type, exp);

        if *exp == TokenType::Int {
            assert_eq!(&tok.literal, "1024");
        }
    }
}

#[test]
fn token_position1() {
    let input = "let five = 5;
let ten = 10;";

    let expected: Vec<Token> = vec![
        Token {
            token_type: TokenType::Let,
            literal:    "let".to_string(),
            row:        1,
            column:     1,
        },
        Token {
            token_type: TokenType::Ident,
            literal:    "five".to_string(),
            row:        1,
            column:     5,
        },
        Token {
            token_type: TokenType::Assign,
            literal:    "=".to_string(),
            row:        1,
            column:     10,
        },
        Token {
            token_type: TokenType::Int,
            literal:    "5".to_string(),
            row:        1,
            column:     12,
        },
        Token {
            token_type: TokenType::Semicolon,
            literal:    ";".to_string(),
            row:        1,
            column:     13,
        },
        Token {
            token_type: TokenType::Let,
            literal:    "let".to_string(),
            row:        2,
            column:     1,
        },
        Token {
            token_type: TokenType::Ident,
            literal:    "ten".to_string(),
            row:        2,
            column:     5,
        },
        Token {
            token_type: TokenType::Assign,
            literal:    "=".to_string(),
            row:        2,
            column:     9,
        },
        Token {
            token_type: TokenType::Int,
            literal:    "10".to_string(),
            row:        2,
            column:     11,
        },
        Token {
            token_type: TokenType::Semicolon,
            literal:    ";".to_string(),
            row:        2,
            column:     13,
        },
        Token {
            token_type: TokenType::Eof,
            literal:    "\0".to_string(),
            row:        2,
            column:     14,
        },
    ];

    let mut l = Lexer::new(input);

    for exp in expected.iter() {
        let tok = l.next_token();

        assert_eq!(&tok, exp);
    }
}

#[test]
fn token_position2() {
    let input = "
    let five = 5;
let ten = 10;
";

    let expected: Vec<Token> = vec![
        Token {
            token_type: TokenType::Let,
            literal:    "let".to_string(),
            row:        2,
            column:     5,
        },
        Token {
            token_type: TokenType::Ident,
            literal:    "five".to_string(),
            row:        2,
            column:     9,
        },
        Token {
            token_type: TokenType::Assign,
            literal:    "=".to_string(),
            row:        2,
            column:     14,
        },
        Token {
            token_type: TokenType::Int,
            literal:    "5".to_string(),
            row:        2,
            column:     16,
        },
        Token {
            token_type: TokenType::Semicolon,
            literal:    ";".to_string(),
            row:        2,
            column:     17,
        },
        Token {
            token_type: TokenType::Let,
            literal:    "let".to_string(),
            row:        3,
            column:     1,
        },
        Token {
            token_type: TokenType::Ident,
            literal:    "ten".to_string(),
            row:        3,
            column:     5,
        },
        Token {
            token_type: TokenType::Assign,
            literal:    "=".to_string(),
            row:        3,
            column:     9,
        },
        Token {
            token_type: TokenType::Int,
            literal:    "10".to_string(),
            row:        3,
            column:     11,
        },
        Token {
            token_type: TokenType::Semicolon,
            literal:    ";".to_string(),
            row:        3,
            column:     13,
        },
        Token {
            token_type: TokenType::Eof,
            literal:    "\0".to_string(),
            row:        4,
            column:     1,
        },
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
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Func,
        TokenType::Lparen,
        TokenType::Ident,
        TokenType::Comma,
        TokenType::Ident,
        TokenType::Rparen,
        TokenType::Lbrace,
        TokenType::Ident,
        TokenType::Plus,
        TokenType::Ident,
        TokenType::Semicolon,
        TokenType::Rbrace,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Ident,
        TokenType::Lparen,
        TokenType::Ident,
        TokenType::Comma,
        TokenType::Ident,
        TokenType::Rparen,
        TokenType::Semicolon,
        TokenType::Eof,
    ];

    let mut l = Lexer::new(input);

    for exp in expected.iter() {
        let tok = l.next_token();

        assert_eq!(&tok.token_type, exp);
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
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Int,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Func,
        TokenType::Lparen,
        TokenType::Ident,
        TokenType::Comma,
        TokenType::Ident,
        TokenType::Rparen,
        TokenType::Lbrace,
        TokenType::Ident,
        TokenType::Plus,
        TokenType::Ident,
        TokenType::Semicolon,
        TokenType::Rbrace,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Ident,
        TokenType::Assign,
        TokenType::Ident,
        TokenType::Lparen,
        TokenType::Ident,
        TokenType::Comma,
        TokenType::Ident,
        TokenType::Rparen,
        TokenType::Semicolon,
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Asterisk,
        TokenType::Slash,
        TokenType::Bang,
        TokenType::Int,
        TokenType::Lt,
        TokenType::Int,
        TokenType::Gt,
        TokenType::Int,
        TokenType::If,
        TokenType::Lparen,
        TokenType::Int,
        TokenType::Neq,
        TokenType::Int,
        TokenType::Rparen,
        TokenType::Lbrace,
        TokenType::Return,
        TokenType::True,
        TokenType::Semicolon,
        TokenType::Rbrace,
        TokenType::Else,
        TokenType::Lbrace,
        TokenType::Return,
        TokenType::False,
        TokenType::Semicolon,
        TokenType::Rbrace,
        TokenType::Int,
        TokenType::Eq,
        TokenType::Int,
        TokenType::Eof,
    ];

    let mut l = Lexer::new(input);

    for exp in expected.iter() {
        let tok = l.next_token();

        assert_eq!(&tok.token_type, exp);
    }
}
