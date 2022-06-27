use ast::Location;
use lexer::token::TokenType;

use crate::{try_parse, PartParsingResult};

use super::Parser;
use crate::Priority;
// use crate::parser::try_parse;

impl<'a> Parser<'a> {
    pub fn parse_statement(&mut self) -> PartParsingResult<ast::Stmt> {
        let stmt = match self.curr_token.token_type {
            TokenType::Let => try_parse!(self, parse_let_statement),
            // self.parse_let_statement(),
            TokenType::Return => try_parse!(self, parse_return_statement),
            // self.parse_return_statement(),
            _ => try_parse!(self, parse_expr_statement),
            //self.parse_expr_statement(),
            // _ => todo!(),
        };

        Ok(stmt)
    }

    pub fn parse_let_statement(&mut self) -> PartParsingResult<ast::Stmt> {
        // let token_type = self.curr_token.token_type;

        if !self.expect_next(TokenType::Ident, "PAR0010") {
            // panic!("PAR0001: No ident after LET");
            return Err(());
            // if error return null
        }

        let var_name = ast::Expr::Ident {
            // token: self.curr_token.token_type,
            // value: self.curr_token.literal.clone(),
            loc:  Location::new(self.curr_token.row, self.curr_token.column),
            name: self.curr_token.literal.clone(),
        };

        if !self.expect_next(TokenType::Assign, "PAR0011") {
            // panic!("PAR0002: No ASSIGN sign after ident");
            return Err(());
            // if error return null
        }
        self.next_token();

        let value = try_parse!(self, parse_expression, Priority::Lowest);
        // self.parse_expression(Priority::Lowest);
        self.next_if(TokenType::Semicolon);

        let stmt = ast::Stmt::Let {
            // loc: Location,
            name: var_name,
            value,
        };

        Ok(stmt)
    }

    pub fn parse_return_statement(&mut self) -> PartParsingResult<ast::Stmt> {
        // let token_type = &self.curr_token.token_type;

        // return
        self.next_token();

        let value = try_parse!(self, parse_expression, Priority::Lowest);
        // let value = self.parse_expression(Priority::Lowest);
        self.next_if(TokenType::Semicolon);

        let stmt = ast::Stmt::Return { value };

        Ok(stmt)
    }

    pub fn parse_expr_statement(&mut self) -> PartParsingResult<ast::Stmt> {
        // let token_type = &self.curr_token.token_type;
        let expr = try_parse!(self, parse_expression, Priority::Lowest);
        self.next_if(TokenType::Semicolon);

        let stmt = ast::Stmt::Expr { expression: expr };

        Ok(stmt)
    }
}
