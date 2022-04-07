use super::Parser;
use super::Priority;
use token::TokenType;

pub(super) type PrefixParseFn = fn(&mut Parser) -> ast::Expression;
pub(super) type InfixParseFn = fn(&mut Parser, ast::Expression) -> ast::Expression;

// for
impl Parser {
    pub(super) fn register_prefix(
        &mut self,
        token_type: TokenType,
        prefix_parse_fn: PrefixParseFn,
    ) {
        self.prefix_parse_funcs.insert(token_type, prefix_parse_fn);
    }

    pub(super) fn register_infix(&mut self, token_type: TokenType, infix_parse_fn: InfixParseFn) {
        self.infix_parse_funcs.insert(token_type, infix_parse_fn);
    }

    pub(super) fn parse_expression(&mut self, priority: Priority) -> ast::Expression {
        let prefix = self.prefix_parse_funcs.get(self.curr_token.token_type);
        if let Some(prefix) = prefix {
            let mut left_exp = prefix(self);

            while self.next_token.token_type != token::SEMICOLON
                && priority < self.next_precedence()
            {
                let infix = self
                    .infix_parse_funcs
                    .get(self.next_token.token_type)
                    .cloned();
                if let Some(infix) = infix {
                    self.next_token();
                    left_exp = infix(self, left_exp);
                }
                else {
                    break;
                }
            }

            left_exp
        }
        else {
            ast::Expression::Undefined
        }
    }

    pub(super) fn parse_block_statement(&mut self) -> Vec<ast::Statement> {
        let mut block_stmt = vec![];
        while self.curr_token.token_type != token::RBRACE
            && self.curr_token.token_type != token::EOF
        {
            let stmt = self.parse_statement();
            block_stmt.push(stmt);
            self.next_token();
        }
        block_stmt
    }
}

impl Parser {
    pub(super) fn curr_precedence(&self) -> Priority {
        let p = self.precedences.get(self.curr_token.token_type);
        if let Some(p) = p {
            p.clone()
        }
        else {
            Priority::Lowest
        }
    }

    pub(super) fn next_precedence(&self) -> Priority {
        let p = self.precedences.get(self.next_token.token_type);
        if let Some(p) = p {
            p.clone()
        }
        else {
            Priority::Lowest
        }
    }
}

impl Parser {
    pub(super) fn parse_ident(&mut self) -> ast::Expression {
        ast::Expression::Ident(ast::Identifier {
            token: self.curr_token.token_type,
            value: self.curr_token.literal.clone(),
        })
    }

    pub(super) fn parse_integer_literal(&mut self) -> ast::Expression {
        ast::Expression::IntegerLiteral {
            token: self.curr_token.token_type,
            value: self.curr_token.literal.parse().unwrap(),
        }
    }

    pub(super) fn parse_prefix_expression(&mut self) -> ast::Expression {
        let token = self.curr_token.token_type;
        let operator = self.curr_token.literal.clone();
        self.next_token();
        let right = Box::new(self.parse_expression(Priority::Prefix));

        ast::Expression::Prefix {
            token,
            operator,
            right,
        }
    }

    pub(super) fn parse_infix_expression(&mut self, left: ast::Expression) -> ast::Expression {
        let token = self.curr_token.token_type;
        let operator = self.curr_token.literal.clone();
        let precedence = self.curr_precedence();
        self.next_token();

        ast::Expression::Infix {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(self.parse_expression(precedence)),
        }
    }

    pub(super) fn parse_boolean(&mut self) -> ast::Expression {
        let token = self.curr_token.token_type;

        ast::Expression::Bool {
            token,
            value: self.curr_token.token_type == token::TRUE,
        }
    }

    pub(super) fn parse_grouped_expression(&mut self) -> ast::Expression {
        self.next_token();
        let expression = self.parse_expression(Priority::Lowest);
        if !self.expect_next(token::RPAREN) {
            ast::Expression::Undefined
        }
        else {
            expression
        }
    }

    pub(super) fn parse_if_expression(&mut self) -> ast::Expression {
        // if
        let token = self.curr_token.token_type;

        // (condition)
        if !self.expect_next(token::LPAREN) {
            return ast::Expression::Undefined;
        }
        self.next_token();

        let condition = Box::new(self.parse_expression(Priority::Lowest));

        if !self.expect_next(token::RPAREN) {
            return ast::Expression::Undefined;
        }

        // { consequence }
        if !self.expect_next(token::LBRACE) {
            return ast::Expression::Undefined;
        }
        self.next_token();

        // else { alternative }
        let consequence = self.parse_block_statement();
        let mut alternative = vec![];
        if self.expect_next(token::ELSE) {
            if !self.expect_next(token::LBRACE) {
                return ast::Expression::Undefined;
            }
            self.next_token();
            alternative = self.parse_block_statement();
        }

        ast::Expression::If {
            token,
            condition,
            consequence,
            alternative,
        }
    }

    pub(super) fn parse_function_literal(&mut self) -> ast::Expression {
        // fn
        let token = self.curr_token.token_type;

        // ( params )
        if !self.expect_next(token::LPAREN) {
            return ast::Expression::Undefined;
        }
        self.next_token();

        let parameters = self.parse_function_parameters();

        // { body }
        if !self.expect_next(token::LBRACE) {
            return ast::Expression::Undefined;
        }
        self.next_token();
        let body = self.parse_block_statement();

        ast::Expression::FunctionLiteral {
            token,
            parameters,
            body,
        }
    }

    pub(super) fn parse_function_call_expression(
        &mut self,
        func: ast::Expression,
    ) -> ast::Expression {
        // (
        let token = self.curr_token.token_type;
        self.next_token();

        let args = self.parse_function_arguments();

        ast::Expression::FunctionCall {
            token,
            func: Box::new(func),
            args,
        }
    }

    //////////////////

    pub(super) fn parse_function_parameters(&mut self) -> Vec<ast::Identifier> {
        let mut params = vec![];

        if self.curr_token.token_type == token::RPAREN {
            return params;
        }

        params.push(ast::Identifier {
            token: token::IDENT,
            value: self.curr_token.literal.clone(),
        });

        while self.next_token.token_type == token::COMMA {
            self.next_token();
            self.next_token();
            params.push(ast::Identifier {
                token: token::IDENT,
                value: self.curr_token.literal.clone(),
            });
        }

        if !self.expect_next(token::RPAREN) {
            // error
            return vec![];
        }

        params
    }

    pub(super) fn parse_function_arguments(&mut self) -> Vec<ast::Expression> {
        let mut args = vec![];

        if self.curr_token.token_type == token::RPAREN {
            self.next_token();
            return args;
        }

        args.push(self.parse_expression(Priority::Lowest));

        while self.next_token.token_type == token::COMMA {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Priority::Lowest));
        }

        if !self.expect_next(token::RPAREN) {
            // error
            return vec![];
        }

        args
    }
}
