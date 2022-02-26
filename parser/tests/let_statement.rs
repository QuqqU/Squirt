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
            let stmt = program.statements[i]
                .as_any()
                .downcast_ref::<ast::LetStatement>();

            let stmt = match stmt {
                Some(s) => s,
                None => panic!("Let stmt!!"),
            };

            assert_eq!(stmt.token, token::LET);
            assert_eq!(stmt.name.token, token::IDENT);
            assert_eq!(stmt.name.value, *exp);
        }
    }
}
