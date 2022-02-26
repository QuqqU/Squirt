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
}
