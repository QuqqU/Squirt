use lexer::token::TokenType;

use crate::ensure_curr;
use crate::try_parse;
use crate::Parser;
use crate::PartParsingResult;
use crate::Priority;

impl<'a> Parser<'a> {
    pub(super) fn parse_stmt(&mut self) -> PartParsingResult<ast::Stmt> {
        let stmt = match self.curr_token.token_type {
            TokenType::Let => try_parse!(self, parse_let_stmt),
            TokenType::Return => try_parse!(self, parse_return_stmt),
            _ => try_parse!(self, parse_expr_stmt),
        };

        Ok(stmt)
    }

    // { stmt1; stmt2; }
    pub fn parse_block_stmts(&mut self) -> PartParsingResult<Vec<ast::Stmt>> {
        // {
        ensure_curr!(self, "PAR:2001", TokenType::Lbrace);

        // stmts
        let mut block_stmts = vec![];
        while self.curr_token.token_type != TokenType::Rbrace
            && self.curr_token.token_type != TokenType::Eof
        {
            while self.curr_token.token_type == TokenType::Semicolon {
                self.next_token();
            }

            if let Some(stmt) = self.parse_stmt().ok() {
                block_stmts.push(stmt);
            }
            else {
                self.skip_stmt();
            }
        }

        // }
        ensure_curr!(self, "PAR:2002", TokenType::Rbrace);

        Ok(block_stmts)
    }

    pub(super) fn parse_let_stmt(&mut self) -> PartParsingResult<ast::Stmt> {
        // let
        ensure_curr!(self, "PAR:2011", TokenType::Let);

        // ident
        let name = try_parse!(self, parse_ident);

        // =
        ensure_curr!(self, "PAR:2012", TokenType::Assign);

        // expr
        let expr = try_parse!(self, parse_expr, Priority::Lowest);

        // ;(optional)
        self.next_if(TokenType::Semicolon);

        Ok(ast::Stmt::Let { name, expr })
    }

    pub(super) fn parse_return_stmt(&mut self) -> PartParsingResult<ast::Stmt> {
        // return
        ensure_curr!(self, "PAR:2021", TokenType::Return);

        // expr
        let expr = try_parse!(self, parse_expr, Priority::Lowest);

        // ;(optional)
        self.next_if(TokenType::Semicolon);

        Ok(ast::Stmt::Return { expr })
    }

    pub(super) fn parse_expr_stmt(&mut self) -> PartParsingResult<ast::Stmt> {
        // expr
        let expr = try_parse!(self, parse_expr, Priority::Lowest);

        // ;(optional)
        self.next_if(TokenType::Semicolon);

        Ok(ast::Stmt::Expr { expr })
    }
}
