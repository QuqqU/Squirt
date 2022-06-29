use ast::Location;
use lexer::token::TokenType;

use crate::parser::Parser;
use crate::{ensure_curr, verify_curr, Priority};
use crate::{try_parse, PartParsingResult};

impl<'a> Parser<'a> {
    pub fn parse_expr(&mut self, priority: Priority) -> PartParsingResult<ast::Expr> {
        // prefix & left
        let prefix = self
            .settings
            .prefix_parse_funcs
            .get(&self.curr_token.token_type);

        let prefix = match prefix {
            Some(p) => p,
            None => {
                self.errors.push(format!(
                    "Line {}:{} PAR:3001 - Cannot parse prefix, found {}",
                    self.curr_token.row, self.curr_token.column, self.curr_token,
                ));
                return Err(());
            }
        };

        let mut left_exp = prefix(self)?;

        // infix
        while priority < self.curr_precedence() {
            let infix = self
                .settings
                .infix_parse_funcs
                .get(&self.curr_token.token_type)
                .cloned();

            let infix = match infix {
                Some(i) => i,
                None => {
                    self.errors.push(format!(
                        "Line {}:{} PAR:3002 - Cannot parse infix, found {}",
                        self.curr_token.row, self.curr_token.column, self.curr_token,
                    ));
                    return Err(());
                }
            };

            left_exp = infix(self, left_exp)?;
        }

        Ok(left_exp)
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
        verify_curr!(self, "PAR:3011", TokenType::Ident);

        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let name = self.curr_token.literal.clone();
        self.next_token();

        Ok(ast::Expr::Ident { loc, name })
    }

    pub fn parse_int(&mut self) -> PartParsingResult<ast::Expr> {
        verify_curr!(self, "PAR:3012", TokenType::Int);

        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let value = self.curr_token.literal.parse().unwrap();
        self.next_token();

        Ok(ast::Expr::Int { loc, value })
    }

    pub fn parse_bool(&mut self) -> PartParsingResult<ast::Expr> {
        verify_curr!(self, "PAR:3013", TokenType::True, TokenType::False);

        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let value = self.curr_token.token_type == TokenType::True;
        self.next_token();

        Ok(ast::Expr::Bool { loc, value })
    }

    pub fn parse_prefix_expr(&mut self) -> PartParsingResult<ast::Expr> {
        verify_curr!(self, "PAR:3021", TokenType::Minus, TokenType::Bang);

        // prefix operator
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let operator = Parser::token_2_prefix(self.curr_token.token_type);
        self.next_token();

        // expr
        let right = Box::new(try_parse!(self, parse_expr, Priority::Prefix));

        Ok(ast::Expr::Prefix {
            loc,
            operator,
            right,
        })
    }

    pub fn parse_infix_expr(&mut self, left: ast::Expr) -> PartParsingResult<ast::Expr> {
        verify_curr!(
            self,
            "PAR:3022",
            TokenType::Assign,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Asterisk,
            TokenType::Slash,
            TokenType::Lt,
            TokenType::Gt,
            TokenType::Eq,
            TokenType::Neq
        );

        // left expr
        let left = Box::new(left);

        // infix operator
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let operator = Parser::token_2_infix(self.curr_token.token_type);
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

    pub fn parse_grouped_expr(&mut self) -> PartParsingResult<ast::Expr> {
        // (
        ensure_curr!(self, "PAR:3031", TokenType::Lparen);

        // exprs
        let expr = try_parse!(self, parse_expr, Priority::Lowest);

        // )
        ensure_curr!(self, "PAR:3032", TokenType::Rparen);

        Ok(expr)
    }

    pub fn parse_if_expr(&mut self) -> PartParsingResult<ast::Expr> {
        // if
        verify_curr!(self, "PAR:3041", TokenType::If);
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        self.next_token();

        // (condition)
        ensure_curr!(self, "PAR:3042", TokenType::Lparen);
        let condition = Box::new(try_parse!(self, parse_expr, Priority::Lowest));
        ensure_curr!(self, "PAR:3043", TokenType::Rparen);

        // { consequence }
        ensure_curr!(self, "PAR:3044", TokenType::Lbrace);
        let consequence = try_parse!(self, parse_block_stmts);
        ensure_curr!(self, "PAR:3045", TokenType::Rbrace);

        // else { alternative }
        let mut alternative = vec![];
        if self.next_if(TokenType::Else) {
            ensure_curr!(self, "PAR:3046", TokenType::Lbrace);
            alternative = try_parse!(self, parse_block_stmts);
            ensure_curr!(self, "PAR:3047", TokenType::Rbrace);
        }

        Ok(ast::Expr::If {
            loc,
            condition,
            consequence,
            alternative,
        })
    }

    /*     pub fn parse_function_literal(&mut self) -> PartParsingResult<ast::Expr> {
        // fn
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        // let token = self.curr_token;

        // ( params )
        if !self.expect_next("PAR9999"),TokenType::Lparen {
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

    pub fn parse_function_call_expr(
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

        if !self.expect_next("PAR:9999"),TokenType::Rparen {
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

        // args.push(self.parse_expr(Priority::Lowest));
        args.push(try_parse!(self, parse_expr, Priority::Lowest));

        while self.next_token.token_type == TokenType::Comma {
            self.next_token();
            self.next_token();
            // args.push(self.parse_expr(Priority::Lowest));
            args.push(try_parse!(self, parse_expr, Priority::Lowest));
        }

        if !self.expect_next("PAR9999"),TokenType::Rparen {
            return Err(());
        }
        self.next_token();

        Ok(args)
    }
    */
}
