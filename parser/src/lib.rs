use std::collections::HashMap;

use lexer::Lexer;
use token::{Token, TokenType};

type PrefixParseFn = fn(&Parser) -> ast::Expression;
type InfixParseFn = fn(ast::Expression) -> ast::Expression;

enum Priority {
    Lowest,
    Equals,
    LessGreater,
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
            prefix_parse_funcs: HashMap::new(),
            infix_parse_funcs:  HashMap::new(),
        };

        p.register_prefix(token::IDENT, Parser::parse_ident);

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

        //todo expression
        while self.curr_token.token_type != token::SEMICOLON {
            self.next_token();
        }

        let stmt = ast::Statement::Let {
            token: token_type,
            name:  var_name,
            value: ast::Expression::Undefined,
        };

        stmt
    }

    fn parse_return_statement(&mut self) -> ast::Statement {
        let token_type = self.curr_token.token_type;
        self.next_token();

        //todo expression
        while self.curr_token.token_type != token::SEMICOLON {
            self.next_token();
        }

        let stmt = ast::Statement::Return {
            token: token_type,
            value: ast::Expression::Undefined,
        };

        stmt
    }

    fn parse_expr_statement(&mut self) -> ast::Statement {
        let token_type = self.curr_token.token_type;
        let expr = self.parse_expression(Priority::Lowest);

        if self.expect_next(token::SEMICOLON) {
            self.next_token();
        }

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

    fn parse_ident(&self) -> ast::Expression {
        ast::Expression::Ident(ast::Identifier {
            token: self.curr_token.token_type,
            value: self.curr_token.literal.clone(),
        })
    }

    fn parse_expression(&self, priority: Priority) -> ast::Expression {
        let prefix = self.prefix_parse_funcs.get(self.curr_token.token_type);
        if let Some(prefix) = prefix {
            let left_exp = prefix(self);
            left_exp
        }
        else {
            ast::Expression::Undefined
        }
    }
}
