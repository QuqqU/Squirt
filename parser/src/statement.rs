use token::Token;

use super::Parser;
use super::Priority;

impl<'a> Parser<'a> {
    pub(super) fn parse_statement(&mut self) -> ParsingResult<ast::Statement> {
        match self.curr_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expr_statement(),
        }
    }

    pub(super) fn parse_let_statement(&mut self) -> ast::Statement {
        // let token_type = self.curr_token.token_type;

        if !self.expect_next(Token::Ident) {
            panic!("PAR0001: No ident after LET");
            // if error return null
        }

        let var_name = ast::Identifier {
            token: self.curr_token.token_type,
            value: self.curr_token.literal.clone(),
        };

        if !self.expect_next(token::ASSIGN) {
            panic!("PAR0002: No ASSIGN sign after ident");
            // if error return null
        }
        self.next_token();

        let value = self.parse_expression(Priority::Lowest);
        self.expect_next(token::SEMICOLON);

        let stmt = ast::Statement::Let {
            name: var_name,
            value,
        };

        stmt
    }

    pub(super) fn parse_return_statement(&mut self) -> ast::Statement {
        let token_type = self.curr_token.token_type;
        self.next_token();

        let value = self.parse_expression(Priority::Lowest);
        self.expect_next(token::SEMICOLON);

        let stmt = ast::Statement::Return { value };

        stmt
    }

    pub(super) fn parse_expr_statement(&mut self) -> ast::Statement {
        let token_type = self.curr_token.token_type;
        let expr = self.parse_expression(Priority::Lowest);

        self.expect_next(token::SEMICOLON);

        let stmt = ast::Statement::Expr { expression: expr };

        stmt
    }
}
