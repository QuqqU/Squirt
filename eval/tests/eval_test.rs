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
    }

    #[test]
    fn test_prefix_bang_expression() {
        let input = "
            !5;
            !0;
            !!123;
            !true;
            !false;
            !!true;
            !!false;
            
        "
        .to_string();
        // todo
        // to be added when string eval
        // !abcde;
        // !!!abc;
        //
        let expected: Vec<bool> = vec![
            false, true, true, false, true, true, false,
            // false, false
        ];

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), expected.len());

        for (i, stmt) in program.statements.iter().enumerate() {
            let &b = eval::eval(stmt)
                .as_any()
                .downcast_ref::<object::Bool>()
                .unwrap()
                .value;

            assert_eq!(b, expected[i]);
        }
    }

    #[test]
    fn test_prefix_minus_expression() {
        let input = "
            -5;
            10;
            5;
            -10;
        "
        .to_string();

        let expected: Vec<i64> = vec![-5, 10, 5, -10];

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), expected.len());

        for (i, stmt) in program.statements.iter().enumerate() {
            let b = eval::eval(stmt)
                .as_any()
                .downcast_ref::<object::Integer>()
                .unwrap()
                .value;

            assert_eq!(b, expected[i]);
        }
    }

    #[test]
    fn test_interger_infix_expression1() {
        let input = "
            5 + 5 + 5 - 10;
            2 * 2 * 2 * 2 * 2;
            -50 + 100 + -50;
            5 * 2 + 10;
            5 + 2 * 10;
            (1 + 2) * 3 - 2 - (3 + 4) * 1;
        "
        .to_string();

        let expected: Vec<i64> = vec![5, 32, 0, 20, 25, 0];

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), expected.len());

        for (i, stmt) in program.statements.iter().enumerate() {
            let b = eval::eval(stmt)
                .as_any()
                .downcast_ref::<object::Integer>()
                .unwrap()
                .value;

            assert_eq!(b, expected[i]);
        }
    }

    #[test]
    fn test_interger_infix_expression2() {
        let input = "
            1 == 1;
            1 != 1;
            1 < 2;
            1 > 2;
            2 < 1;
            2 > 1;
            2 < 2;
            3 > 3;
        "
        .to_string();

        let expected: Vec<bool> = vec![true, false, true, false, false, true, false, false];

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), expected.len());

        for (i, stmt) in program.statements.iter().enumerate() {
            let &b = eval::eval(stmt)
                .as_any()
                .downcast_ref::<object::Bool>()
                .unwrap()
                .value;

            assert_eq!(b, expected[i]);
        }
    }

    #[test]
    fn test_bool_infix_expression() {
        let input = "
            true == true;
            false == false;
            true == false;
            false != true;
            true == (1 < 2);
            false == (2 != 2);
        "
        .to_string();

        let expected: Vec<bool> = vec![true, true, false, true, true, true];

        let mut p = parser::Parser::new(lexer::Lexer::new(input));
        let program = p.parse_program();

        assert_eq!(program.statements.len(), expected.len());

        for (i, stmt) in program.statements.iter().enumerate() {
            let &b = eval::eval(stmt)
                .as_any()
                .downcast_ref::<object::Bool>()
                .unwrap()
                .value;

            assert_eq!(b, expected[i]);
        }
    }
}
