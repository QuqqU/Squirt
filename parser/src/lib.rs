use std::collections::HashMap;

use lexer::Lexer;
use token::{self, Token, TokenType};

type PrefixParseFn = fn(&mut Parser) -> ast::Expression;
type InfixParseFn = fn(&mut Parser, ast::Expression) -> ast::Expression;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    Lowest,
    Equal,
    Compare,
    Sum,
    Product,
    Prefix,
    Call,
}

pub struct Parser {
    lexer:      Lexer,
    curr_token: Token,
    next_token: Token,

    // todo errors
    // errors:     Vec<String>,
    precedences:        HashMap<TokenType, Priority>,
    prefix_parse_funcs: HashMap<TokenType, PrefixParseFn>,
    infix_parse_funcs:  HashMap<TokenType, InfixParseFn>,
}

impl Parser {
    pub fn new(mut l: Lexer) -> Self {
        let ctoken = l.next_token();
        let ntoken = l.next_token();

        let mut p = Self {
            lexer:              l,
            curr_token:         ctoken,
            next_token:         ntoken,
            // errors:     vec![],
            precedences:        HashMap::from([
                (token::EQ, Priority::Equal),
                (token::NEQ, Priority::Equal),
                (token::LT, Priority::Compare),
                (token::GT, Priority::Compare),
                (token::PLUS, Priority::Sum),
                (token::MINUS, Priority::Sum),
                (token::SLASH, Priority::Product),
                (token::ASTERISK, Priority::Product),
                (token::LPAREN, Priority::Call),
            ]),
            prefix_parse_funcs: HashMap::new(),
            infix_parse_funcs:  HashMap::new(),
        };

        p.register_prefix(token::IDENT, Parser::parse_ident);
        p.register_prefix(token::INT, Parser::parse_integer_literal);
        p.register_prefix(token::BANG, Parser::parse_prefix_expression);
        p.register_prefix(token::MINUS, Parser::parse_prefix_expression);

        p.register_infix(token::PLUS, Parser::parse_infix_expression);
        p.register_infix(token::MINUS, Parser::parse_infix_expression);
        p.register_infix(token::ASTERISK, Parser::parse_infix_expression);
        p.register_infix(token::SLASH, Parser::parse_infix_expression);
        p.register_infix(token::LT, Parser::parse_infix_expression);
        p.register_infix(token::GT, Parser::parse_infix_expression);
        p.register_infix(token::EQ, Parser::parse_infix_expression);
        p.register_infix(token::NEQ, Parser::parse_infix_expression);

        p.register_prefix(token::TRUE, Parser::parse_boolean);
        p.register_prefix(token::FALSE, Parser::parse_boolean);

        p.register_prefix(token::LPAREN, Parser::parse_grouped_expression);
        p.register_prefix(token::IF, Parser::parse_if_expression);
        p.register_prefix(token::FUNC, Parser::parse_function_literal);
        p.register_infix(token::LPAREN, Parser::parse_function_call_expression);
        p
    }

    fn next_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn expect_next(&mut self, expected_type: token::TokenType) -> bool {
        if self.next_token.token_type == expected_type {
            self.next_token();
            true
        }
        else {
            // if error append errros
            false
        }
    }
}

// for statement
impl Parser {
    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program { statements: vec![] };
        while self.curr_token.token_type != token::EOF {
            let stmt = self.parse_statement();
            // println!("=> {}", stmt.to_string());
            program.statements.push(stmt);
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> ast::Statement {
        match self.curr_token.token_type {
            token::LET => self.parse_let_statement(),
            token::RETURN => self.parse_return_statement(),
            _ => self.parse_expr_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> ast::Statement {
        let token_type = self.curr_token.token_type;

        if !self.expect_next(token::IDENT) {
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
            token: token_type,
            name: var_name,
            value,
        };

        stmt
    }

    fn parse_return_statement(&mut self) -> ast::Statement {
        let token_type = self.curr_token.token_type;
        self.next_token();

        let value = self.parse_expression(Priority::Lowest);
        self.expect_next(token::SEMICOLON);

        let stmt = ast::Statement::Return {
            token: token_type,
            value,
        };

        stmt
    }

    fn parse_expr_statement(&mut self) -> ast::Statement {
        let token_type = self.curr_token.token_type;
        let expr = self.parse_expression(Priority::Lowest);

        self.expect_next(token::SEMICOLON);

        let stmt = ast::Statement::Expr {
            token:      token_type,
            expression: expr,
        };

        stmt
    }
}

// for
impl Parser {
    fn register_prefix(&mut self, token_type: TokenType, prefix_parse_fn: PrefixParseFn) {
        self.prefix_parse_funcs.insert(token_type, prefix_parse_fn);
    }

    fn register_infix(&mut self, token_type: TokenType, infix_parse_fn: InfixParseFn) {
        self.infix_parse_funcs.insert(token_type, infix_parse_fn);
    }

    fn curr_precedence(&self) -> Priority {
        let p = self.precedences.get(self.curr_token.token_type);
        if let Some(p) = p {
            p.clone()
        }
        else {
            Priority::Lowest
        }
    }

    fn next_precedence(&self) -> Priority {
        let p = self.precedences.get(self.next_token.token_type);
        if let Some(p) = p {
            p.clone()
        }
        else {
            Priority::Lowest
        }
    }
    //////////////////

    fn parse_ident(&mut self) -> ast::Expression {
        ast::Expression::Ident(ast::Identifier {
            token: self.curr_token.token_type,
            value: self.curr_token.literal.clone(),
        })
    }

    fn parse_integer_literal(&mut self) -> ast::Expression {
        ast::Expression::IntegerLiteral {
            token: self.curr_token.token_type,
            value: self.curr_token.literal.parse().unwrap(),
        }
    }

    fn parse_prefix_expression(&mut self) -> ast::Expression {
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

    fn parse_infix_expression(&mut self, left: ast::Expression) -> ast::Expression {
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

    fn parse_boolean(&mut self) -> ast::Expression {
        let token = self.curr_token.token_type;

        ast::Expression::Bool {
            token,
            value: self.curr_token.token_type == token::TRUE,
        }
    }

    fn parse_grouped_expression(&mut self) -> ast::Expression {
        self.next_token();
        let expression = self.parse_expression(Priority::Lowest);
        if !self.expect_next(token::RPAREN) {
            ast::Expression::Undefined
        }
        else {
            expression
        }
    }

    fn parse_if_expression(&mut self) -> ast::Expression {
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

    fn parse_function_literal(&mut self) -> ast::Expression {
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

    fn parse_function_call_expression(&mut self, func: ast::Expression) -> ast::Expression {
        // (
        let token = self.curr_token.token_type;
        self.next_token();

        let mut args = self.parse_function_arguments();

        ast::Expression::FunctionCall {
            token,
            func: Box::new(func),
            args,
        }
    }

    //////////////////

    fn parse_function_parameters(&mut self) -> Vec<ast::Identifier> {
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

    fn parse_function_arguments(&mut self) -> Vec<ast::Expression> {
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

    fn parse_block_statement(&mut self) -> Vec<ast::Statement> {
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

    fn parse_expression(&mut self, priority: Priority) -> ast::Expression {
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
}
