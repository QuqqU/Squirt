use ast::{Args, BlockStmts, InfixType, Location, Params, PrefixType};
use lexer::TokenType;

use super::Parser;
use crate::{check_curr, consume_curr, Priority};
use crate::{try_parse, PartParsingResult};

impl<'a> Parser<'a> {
    pub(super) fn parse_expr(&mut self, priority: Priority) -> PartParsingResult<ast::Expr> {
        // prefix & left
        let prefix = self
            .settings
            .prefix_parse_funcs
            .get(&self.curr_token.token_type);

        let prefix = match prefix {
            Some(p) => p,
            None => {
                self.raise_err("PAR:3001", "Cannot parse prefix");
                return Err(());
            }
        };

        let mut left_exp = prefix(self)?;

        // infix
        while priority < self.curr_precedence() {
            let infix = self
                .settings
                .infix_parse_funcs
                .get(&self.curr_token.token_type);

            let infix = match infix {
                Some(i) => i,
                None => {
                    self.raise_err("PAR:3002", "Cannot parse infix");
                    return Err(());
                }
            };

            left_exp = infix(self, left_exp)?;
        }

        Ok(left_exp)
    }
}

impl<'a> Parser<'a> {
    #[inline]
    fn curr_precedence(&self) -> Priority {
        self.settings.precedence_of(&self.curr_token.token_type)
    }

    #[inline]
    fn token_2_prefix(token: &TokenType) -> PrefixType {
        match token {
            TokenType::Minus => PrefixType::Minus,
            TokenType::Bang => PrefixType::Bang,
            _ => unreachable!("PAR:3003 - Cannot transform TokenType to PrefixType"),
        }
    }

    #[inline]
    fn token_2_infix(token: &TokenType) -> InfixType {
        match token {
            TokenType::Assign => InfixType::Assign,
            TokenType::Plus => InfixType::Plus,
            TokenType::Minus => InfixType::Minus,
            TokenType::Asterisk => InfixType::Asterisk,
            TokenType::Slash => InfixType::Slash,
            TokenType::Lt => InfixType::Lt,
            TokenType::Gt => InfixType::Gt,
            TokenType::Eq => InfixType::Eq,
            TokenType::Neq => InfixType::Neq,
            _ => unreachable!("PAR:3004 - Cannot transform TokenType to InfixType"),
        }
    }
}

impl<'a> Parser<'a> {
    pub(super) fn parse_ident(&mut self) -> PartParsingResult<ast::Expr> {
        check_curr!(self, "PAR:3011", TokenType::Ident);

        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let name = std::mem::take(&mut self.curr_token.literal);
        self.next_token();

        Ok(ast::Expr::Ident { loc, name })
    }

    pub(super) fn parse_int(&mut self) -> PartParsingResult<ast::Expr> {
        check_curr!(self, "PAR:3012", TokenType::Int);

        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let value = self.curr_token.literal.parse().unwrap();
        self.next_token();

        Ok(ast::Expr::Int { loc, value })
    }

    pub(super) fn parse_bool(&mut self) -> PartParsingResult<ast::Expr> {
        check_curr!(
            self,
            "PAR:3013",
            &[TokenType::True, TokenType::False],
            "expected Boolean"
        );

        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let value = self.curr_token.token_type == TokenType::True;
        self.next_token();

        Ok(ast::Expr::Bool { loc, value })
    }

    pub(super) fn parse_prefix_expr(&mut self) -> PartParsingResult<ast::Expr> {
        check_curr!(
            self,
            "PAR:3021",
            &[TokenType::Minus, TokenType::Bang],
            "expected Prefix Operator"
        );

        // prefix operator
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let operator = Parser::token_2_prefix(&self.curr_token.token_type);
        self.next_token();

        // expr
        let right = Box::new(try_parse!(self, parse_expr, Priority::Prefix));

        Ok(ast::Expr::Prefix {
            loc,
            operator,
            right,
        })
    }

    pub(super) fn parse_infix_expr(&mut self, left: ast::Expr) -> PartParsingResult<ast::Expr> {
        check_curr!(
            self,
            "PAR:3022",
            &[
                TokenType::Assign,
                TokenType::Plus,
                TokenType::Minus,
                TokenType::Asterisk,
                TokenType::Slash,
                TokenType::Lt,
                TokenType::Gt,
                TokenType::Eq,
                TokenType::Neq
            ],
            "expected Infix Operator"
        );

        // left expr
        let left = Box::new(left);

        // infix operator
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let operator = Parser::token_2_infix(&self.curr_token.token_type);
        let precedence = self.curr_precedence();
        self.next_token();

        // right expr
        let right = Box::new(try_parse!(self, parse_expr, precedence));

        Ok(ast::Expr::Infix {
            loc,
            left,
            operator,
            right,
        })
    }

    pub(super) fn parse_grouped_expr(&mut self) -> PartParsingResult<ast::Expr> {
        // (
        consume_curr!(self, "PAR:3031", TokenType::Lparen);

        // exprs
        let expr = try_parse!(self, parse_expr, Priority::Lowest);

        // )
        consume_curr!(self, "PAR:3032", TokenType::Rparen);

        Ok(expr)
    }

    pub(super) fn parse_if_expr(&mut self) -> PartParsingResult<ast::Expr> {
        // if
        check_curr!(self, "PAR:3041", TokenType::If);
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        self.next_token();

        // (condition)
        consume_curr!(self, "PAR:3042", TokenType::Lparen);
        let condition = Box::new(try_parse!(self, parse_expr, Priority::Lowest));
        consume_curr!(self, "PAR:3043", TokenType::Rparen);

        // { consequence }
        let consequence = try_parse!(self, parse_block_stmts);

        // else { alternative }
        let mut alternative = BlockStmts(vec![]);
        if self.next_if(TokenType::Else) {
            alternative = try_parse!(self, parse_block_stmts);
        }

        Ok(ast::Expr::If {
            loc,
            condition,
            consequence,
            alternative,
        })
    }

    pub(super) fn parse_func_literal_expr(&mut self) -> PartParsingResult<ast::Expr> {
        // fn
        check_curr!(self, "PAR:3081", TokenType::Func);
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        self.next_token();

        // ( params )
        let parameters = try_parse!(self, parse_func_params);

        // { body }
        let body = try_parse!(self, parse_block_stmts);

        Ok(ast::Expr::FuncLiteral {
            loc,
            parameters,
            body,
        })
    }

    pub(super) fn parse_func_call_expr(
        &mut self,
        func_ident: ast::Expr,
    ) -> PartParsingResult<ast::Expr> {
        // (
        check_curr!(self, "PAR:3091", TokenType::Lparen);
        let loc = Location::new(self.curr_token.row, self.curr_token.column);

        // ( params )
        let args = try_parse!(self, parse_func_args);

        Ok(ast::Expr::FuncCall {
            loc,
            ident: Box::new(func_ident),
            args,
        })
    }

    fn parse_func_params(&mut self) -> PartParsingResult<Params> {
        // (
        consume_curr!(self, "PAR:3085", TokenType::Lparen);

        let mut params = vec![];

        if self.next_if(TokenType::Rparen) {
            return Ok(Params(params));
        }

        params.push(try_parse!(self, parse_ident));
        while !self.next_if(TokenType::Rparen) {
            consume_curr!(self, "PAR:3086", TokenType::Comma);
            params.push(try_parse!(self, parse_ident));
        }

        Ok(Params(params))
    }

    pub(super) fn parse_func_args(&mut self) -> PartParsingResult<Args> {
        // (
        consume_curr!(self, "PAR:3095", TokenType::Lparen);

        let mut args = vec![];

        if self.next_if(TokenType::Rparen) {
            return Ok(Args(args));
        }

        args.push(try_parse!(self, parse_expr, Priority::Lowest));
        while !self.next_if(TokenType::Rparen) {
            consume_curr!(self, "PAR:3096", TokenType::Comma);
            args.push(try_parse!(self, parse_expr, Priority::Lowest));
        }

        Ok(Args(args))
    }
}
