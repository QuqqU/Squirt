#[cfg(test)]
mod lexer_tests {
    use token::*;

    #[test]
    fn simple_literal() {
        let input = "=+(){},;".to_string();

        let expected: Vec<(TokenType, &str)> = vec![
            (token::ASSIGN, "="),
            (token::PLUS, "+"),
            (token::LPAREN, "("),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::RBRACE, "}"),
            (token::COMMA, ","),
            (token::SEMICOLON, ";"),
            (token::EOF, "\0"),
        ];

        let mut l = lexer::Lexer::new(input);

        for exp in expected.iter() {
            let tok = l.next_token();

            assert_eq!(tok.token_type, exp.0);
            assert_eq!(tok.literal, exp.1);
        }
    }

    #[test]
    fn code1() {
        let input = "
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        "
        .to_string();

        let expected: Vec<(TokenType, &str)> = vec![
            (token::LET, "let"),
            (token::IDENT, "five"),
            (token::ASSIGN, "="),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "ten"),
            (token::ASSIGN, "="),
            (token::INT, "10"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "add"),
            (token::ASSIGN, "="),
            (token::FUNC, "fn"),
            (token::LPAREN, "("),
            (token::IDENT, "x"),
            (token::COMMA, ","),
            (token::IDENT, "y"),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::IDENT, "x"),
            (token::PLUS, "+"),
            (token::IDENT, "y"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "result"),
            (token::ASSIGN, "="),
            (token::IDENT, "add"),
            (token::LPAREN, "("),
            (token::IDENT, "five"),
            (token::COMMA, ","),
            (token::IDENT, "ten"),
            (token::RPAREN, ")"),
            (token::SEMICOLON, ";"),
            (token::EOF, "\0"),
        ];

        let mut l = lexer::Lexer::new(input);

        for exp in expected.iter() {
            let tok = l.next_token();

            assert_eq!(tok.token_type, exp.0);
            assert_eq!(tok.literal, exp.1);
        }
    }

    #[test]
    fn code2() {
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
        "
        .to_string();

        let expected: Vec<(TokenType, &str)> = vec![
            (token::LET, "let"),
            (token::IDENT, "five"),
            (token::ASSIGN, "="),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "ten"),
            (token::ASSIGN, "="),
            (token::INT, "10"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "add"),
            (token::ASSIGN, "="),
            (token::FUNC, "fn"),
            (token::LPAREN, "("),
            (token::IDENT, "x"),
            (token::COMMA, ","),
            (token::IDENT, "y"),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::IDENT, "x"),
            (token::PLUS, "+"),
            (token::IDENT, "y"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "result"),
            (token::ASSIGN, "="),
            (token::IDENT, "add"),
            (token::LPAREN, "("),
            (token::IDENT, "five"),
            (token::COMMA, ","),
            (token::IDENT, "ten"),
            (token::RPAREN, ")"),
            (token::SEMICOLON, ";"),
            (token::PLUS, "+"),
            (token::MINUS, "-"),
            (token::ASTERISK, "*"),
            (token::SLASH, "/"),
            (token::BANG, "!"),
            (token::INT, "1"),
            (token::LT, "<"),
            (token::INT, "3"),
            (token::GT, ">"),
            (token::INT, "2"),
            (token::IF, "if"),
            (token::LPAREN, "("),
            (token::INT, "5"),
            (token::NEQ, "!="),
            (token::INT, "3"),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::RETURN, "return"),
            (token::TRUE, "true"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            (token::ELSE, "else"),
            (token::LBRACE, "{"),
            (token::RETURN, "return"),
            (token::FALSE, "false"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            (token::INT, "1"),
            (token::EQ, "=="),
            (token::INT, "1"),
            (token::EOF, "\0"),
        ];

        let mut l = lexer::Lexer::new(input);

        for exp in expected.iter() {
            let tok = l.next_token();

            assert_eq!(tok.token_type, exp.0);
            assert_eq!(tok.literal, exp.1);
        }
    }
}
