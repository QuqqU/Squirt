#[cfg(test)]
mod eval_tests {

    #[test]
    fn test_let() {
        let input = "
            5;
            123;
            0;
        "
        .to_string();

        let expected: Vec<object::Integer> = vec![
            object::Integer { value: 5 },
            object::Integer { value: 123 },
            object::Integer { value: 0 },
        ];

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), expected.len());

        for (i, stmt) in program.statements.iter().enumerate() {
            assert_eq!(eval::eval(stmt).inspect(), expected[i].value.to_string());
        }
        // panic!();
    }
}
