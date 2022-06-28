use ast::Location;

use super::Parser;
use crate::{ensure_curr, Priority};
use crate::{ensure_next, try_parse, PartParsingResult};
use lexer::token::TokenType;

// for
impl<'a> Parser<'a> {
    pub fn parse_expression(&mut self, priority: Priority) -> PartParsingResult<ast::Expr> {
        // prefix & left
        let prefix = self
            .settings
            .prefix_parse_funcs
            .get(&self.curr_token.token_type)
            .unwrap();

        let mut left_exp = prefix(self)?;

        // infix
        while self.curr_token.token_type != TokenType::Semicolon
            && priority < self.curr_precedence()
        {
            let infix = self
                .settings
                .infix_parse_funcs
                .get(&self.curr_token.token_type)
                .cloned()
                .unwrap();

            left_exp = infix(self, left_exp)?;
        }

        Ok(left_exp)
    }

    // { stmt1; stmt2; }
    pub fn parse_block_statements(&mut self) -> PartParsingResult<Vec<ast::Stmt>> {
        // {
        self.next_token();

        // stmts
        let mut block_stmts = vec![];
        while self.curr_token.token_type != TokenType::Rbrace
            && self.curr_token.token_type != TokenType::Eof
        {
            let stmt = try_parse!(self, parse_statement);
            block_stmts.push(stmt);
        }

        // }
        ensure_curr!(self, TokenType::Rbrace, "PAR9999");

        Ok(block_stmts)
    }
}

impl<'a> Parser<'a> {
    pub fn curr_precedence(&self) -> Priority {
        let p = self.settings.precedences.get(&self.curr_token.token_type);
        if let Some(p) = p {
            p.clone()
        }
        else {
            Priority::Lowest
        }
    }

    // pub fn next_precedence(&self) -> Priority {
    //     let p = self.settings.precedences.get(&self.next_token.token_type);
    //     if let Some(p) = p {
    //         p.clone()
    //     }
    //     else {
    //         Priority::Lowest
    //     }
    // }
}

impl<'a> Parser<'a> {
    pub fn parse_ident(&mut self) -> PartParsingResult<ast::Expr> {
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let name = self.curr_token.literal.clone();
        self.next_token();

        Ok(ast::Expr::Ident { loc, name })
    }

    pub fn parse_int(&mut self) -> PartParsingResult<ast::Expr> {
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let value = self.curr_token.literal.parse().unwrap();
        self.next_token();

        Ok(ast::Expr::Int { loc, value })
    }

    pub fn parse_bool(&mut self) -> PartParsingResult<ast::Expr> {
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let value = self.curr_token.token_type == TokenType::True;
        self.next_token();

        Ok(ast::Expr::Bool { loc, value })
    }

    pub fn parse_prefix_expression(&mut self) -> PartParsingResult<ast::Expr> {
        // prefix operator
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let operator = Parser::token_2_prefix(self.curr_token.token_type);
        self.next_token();

        // expression
        let right = Box::new(try_parse!(self, parse_expression, Priority::Prefix));

        Ok(ast::Expr::Prefix {
            loc,
            operator,
            right,
        })
    }

    pub fn parse_infix_express(&mut self, left: ast::Expr) -> PartParsingResult<ast::Expr> {
        // left expression
        let left = Box::new(left);

        // infix operator
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let operator = Parser::token_2_infix(self.curr_token.token_type);
        let precedence = self.curr_precedence();
        self.next_token();

        // right expression
        let right = Box::new(try_parse!(self, parse_expression, precedence));

        Ok(ast::Expr::Infix {
            loc,
            left,
            operator,
            right,
        })
    }

    pub fn parse_grouped_expression(&mut self) -> PartParsingResult<ast::Expr> {
        // (
        self.next_token();

        // expressions
        let expression = try_parse!(self, parse_expression, Priority::Lowest);

        // )
        ensure_curr!(self, TokenType::Rparen, "PAR9999");

        Ok(expression)
    }

    pub fn parse_if_expression(&mut self) -> PartParsingResult<ast::Expr> {
        // if
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        self.next_token();

        // (condition)
        ensure_curr!(self, TokenType::Lparen, "PAR9999");
        let condition = Box::new(try_parse!(self, parse_expression, Priority::Lowest));
        ensure_curr!(self, TokenType::Rparen, "PAR9999");

        // { consequence }
        ensure_curr!(self, TokenType::Lbrace, "PAR9999");
        let consequence = try_parse!(self, parse_block_statements);
        ensure_curr!(self, TokenType::Rbrace, "PAR9999");

        // else { alternative }
        let mut alternative = vec![];
        if self.next_if(TokenType::Else) {
            ensure_curr!(self, TokenType::Lbrace, "PAR9999");
            alternative = try_parse!(self, parse_block_statements);
            ensure_curr!(self, TokenType::Rbrace, "PAR9999");
        }

        Ok(ast::Expr::If {
            loc,
            condition,
            consequence,
            alternative,
        })
    }

    pub fn parse_function_literal(&mut self) -> PartParsingResult<ast::Expr> {
        // fn
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        // let token = self.curr_token;

        // ( params )
        if !self.expect_next(TokenType::Lparen, "PAR9999") {
            return Err(());
        }
        self.next_token();

        let parameters = try_parse!(self, parse_function_parameters);

        // { body }
        let body = try_parse!(self, parse_block_statements);

        Ok(ast::Expr::FunctionLiteral {
            loc,
            parameters,
            body,
        })
    }

    pub fn parse_function_call_expression(
        &mut self,
        func: ast::Expr,
    ) -> PartParsingResult<ast::Expr> {
        // let token = self.curr_token;
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let args = try_parse!(self, parse_function_arguments);

        Ok(ast::Expr::FunctionCall {
            loc,
            func: Box::new(func),
            args,
        })
    }

    //////////////////

    pub fn parse_function_parameters(&mut self) -> PartParsingResult<Vec<ast::Expr>> {
        // Expr::Ident
        let mut params = vec![];

        if self.curr_token.token_type == TokenType::Rparen {
            return Ok(params);
        }

        params.push(ast::Expr::Ident {
            loc:  Location::new(self.curr_token.row, self.curr_token.column),
            name: self.curr_token.literal.clone(),
        });

        while self.next_token.token_type == TokenType::Comma {
            self.next_token();
            self.next_token();
            params.push(ast::Expr::Ident {
                loc:  Location::new(self.curr_token.row, self.curr_token.column),
                name: self.curr_token.literal.clone(),
            });
        }

        if !self.expect_next(TokenType::Rparen, "PAR:9999") {
            // error
            return Err(());
        }

        Ok(params)
    }

    pub fn parse_function_arguments(&mut self) -> PartParsingResult<Vec<ast::Expr>> {
        // (
        // let token = self.curr_token.token_type;
        self.next_token();

        let mut args = vec![];

        if self.curr_token.token_type == TokenType::Rparen {
            self.next_token();
            return Ok(args);
        }

        // args.push(self.parse_expression(Priority::Lowest));
        args.push(try_parse!(self, parse_expression, Priority::Lowest));

        while self.next_token.token_type == TokenType::Comma {
            self.next_token();
            self.next_token();
            // args.push(self.parse_expression(Priority::Lowest));
            args.push(try_parse!(self, parse_expression, Priority::Lowest));
        }

        if !self.expect_next(TokenType::Rparen, "PAR9999") {
            return Err(());
        }
        self.next_token();

        Ok(args)
    }
}
