use ast::Location;

use super::Parser;
use crate::Priority;
use crate::{try_parse, PartParsingResult};
use lexer::token::TokenType;

// for
impl<'a> Parser<'a> {
    pub fn parse_expression(&mut self, priority: Priority) -> PartParsingResult<ast::Expr> {
        let prefix = self
            .settings
            .prefix_parse_funcs
            .get(&self.curr_token.token_type);
        if let Some(prefix) = prefix {
            // let mut left_exp = try_parse!(self, prefix);
            let mut left_exp = prefix(self)?;

            while self.next_token.token_type != TokenType::Semicolon
                && priority < self.next_precedence()
            {
                let infix = self
                    .settings
                    .infix_parse_funcs
                    .get(&self.next_token.token_type)
                    .cloned();
                if let Some(infix) = infix {
                    self.next_token();
                    left_exp = infix(self, left_exp)?;
                }
                else {
                    break;
                }
            }

            Ok(left_exp)
        }
        else {
            unreachable!("PSR0010 : Prefix_Operator, Identifier, or Int Parsing Error")
        }
    }

    pub fn parse_block_statement(&mut self) -> PartParsingResult<Vec<ast::Stmt>> {
        if !self.expect_next(TokenType::Lbrace, "PAR9999") {
            return Err(());
        }
        self.next_token();

        let mut block_stmt = vec![];
        while self.curr_token.token_type != TokenType::Rbrace
        // && self.curr_token.token_type != TokenType::Eof
        {
            // let stmt = self.parse_statement();
            let stmt = try_parse!(self, parse_statement);
            block_stmt.push(stmt);
            self.next_token();
        }
        Ok(block_stmt)
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

    pub fn next_precedence(&self) -> Priority {
        let p = self.settings.precedences.get(&self.next_token.token_type);
        if let Some(p) = p {
            p.clone()
        }
        else {
            Priority::Lowest
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse_ident(&mut self) -> PartParsingResult<ast::Expr> {
        Ok(ast::Expr::Ident {
            loc:  Location::new(self.curr_token.row, self.curr_token.column),
            name: self.curr_token.literal.clone(),
        })
    }

    pub fn parse_integer_literal(&mut self) -> PartParsingResult<ast::Expr> {
        Ok(ast::Expr::Int {
            loc:   Location::new(self.curr_token.row, self.curr_token.column),
            value: self.curr_token.literal.parse().unwrap(),
        })
    }

    pub fn parse_prefix_expression(&mut self) -> PartParsingResult<ast::Expr> {
        // let token = self.curr_token;
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let operator = Parser::token_2_prefix(self.curr_token.token_type);
        // let operator = self.curr_token.literal.clone();
        self.next_token();
        // let right = Box::new(self.parse_expression(Priority::Prefix));
        let right = Box::new(try_parse!(self, parse_expression, Priority::Prefix));

        Ok(ast::Expr::Prefix {
            loc,
            // token,
            operator,
            right,
        })
    }

    pub fn parse_infix_expression(&mut self, left: ast::Expr) -> PartParsingResult<ast::Expr> {
        let loc = Location::new(self.curr_token.row, self.curr_token.column);
        let operator = Parser::token_2_infix(self.curr_token.token_type);
        // let token = self.curr_token; //.token_type;
        // let operator = self.curr_token.literal.clone();
        let precedence = self.curr_precedence();
        self.next_token();

        Ok(ast::Expr::Infix {
            loc,
            left: Box::new(left),
            operator,
            // right:    Box::new(self.parse_expression(precedence)),
            right: Box::new(try_parse!(self, parse_expression, precedence)),
        })
    }

    pub fn parse_boolean(&mut self) -> PartParsingResult<ast::Expr> {
        // let token = self.curr_token.token_type;
        Ok(ast::Expr::Bool {
            loc:   Location::new(self.curr_token.row, self.curr_token.column),
            value: self.curr_token.token_type == TokenType::True,
        })
    }

    pub fn parse_grouped_expression(&mut self) -> PartParsingResult<ast::Expr> {
        self.next_token();
        // let expression = self.parse_expression(Priority::Lowest);
        let expression = try_parse!(self, parse_expression, Priority::Lowest);
        self.expect_next(TokenType::Rparen, "PAR9999");

        Ok(expression)
    }

    pub fn parse_if_expression(&mut self) -> PartParsingResult<ast::Expr> {
        // if
        // let token = self.curr_token;
        let loc = Location::new(self.curr_token.row, self.curr_token.column);

        // (condition)
        if !self.expect_next(TokenType::Lparen, "PAR9999") {
            return Err(());
        }
        self.next_token();

        let condition = Box::new(try_parse!(self, parse_expression, Priority::Lowest));

        if !self.expect_next(TokenType::Rparen, "PAR9999") {
            return Err(());
        }

        // { consequence }
        if !self.expect_next(TokenType::Lbrace, "PAR9999") {
            return Err(());
        }
        self.next_token();

        // else { alternative }
        let consequence = try_parse!(self, parse_block_statement);
        let mut alternative = vec![];
        if self.expect_next(TokenType::Else, "PAR9999") {
            alternative = try_parse!(self, parse_block_statement);
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
        let body = try_parse!(self, parse_block_statement);

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
