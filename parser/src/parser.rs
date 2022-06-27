use std::collections::HashMap;
use std::mem;

use crate::ParsingResult;
use crate::{InfixParseFn, PrefixParseFn};
use ast::{self, InfixType, PrefixType};
use lexer::token::{Token, TokenType};
use lexer::Lexer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Lowest,
    Assign,
    Equal,
    Compare,
    Sum,
    Product,
    Prefix,
    Call,
}

pub struct ParserSettings {
    pub precedences:        HashMap<TokenType, Priority>,
    pub prefix_parse_funcs: HashMap<TokenType, PrefixParseFn>,
    pub infix_parse_funcs:  HashMap<TokenType, InfixParseFn>,
}
impl ParserSettings {
    pub fn set() -> Self {
        Self {
            precedences:        HashMap::from([
                (TokenType::Assign, Priority::Assign),
                (TokenType::Eq, Priority::Equal),
                (TokenType::Neq, Priority::Equal),
                (TokenType::Lt, Priority::Compare),
                (TokenType::Gt, Priority::Compare),
                (TokenType::Plus, Priority::Sum),
                (TokenType::Minus, Priority::Sum),
                (TokenType::Slash, Priority::Product),
                (TokenType::Asterisk, Priority::Product),
                (TokenType::Lparen, Priority::Call),
            ]),
            prefix_parse_funcs: HashMap::new(),
            infix_parse_funcs:  HashMap::new(),
        }
    }
}

pub struct Parser<'a> {
    lexer:                 Lexer<'a>,
    pub(crate) curr_token: Token,
    pub(crate) next_token: Token,
    errors:                Vec<String>,
    pub(crate) settings:   ParserSettings,
    // pub precedences:        HashMap<TokenType, Priority>,
    // pub prefix_parse_funcs: HashMap<TokenType, PrefixParseFn>,
    // pub infix_parse_funcs:  HashMap<TokenType, InfixParseFn>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer::new(input);

        let ctoken = l.next_token();
        let ntoken = l.next_token();
        let settings = ParserSettings::set();
        let mut parser = Self {
            lexer: l,
            curr_token: ctoken,
            next_token: ntoken,
            errors: vec![],
            settings,
        };

        parser.register_prefix(TokenType::Ident, |p| p.parse_ident());
        parser.register_prefix(TokenType::Int, |p| p.parse_integer_literal());
        parser.register_prefix(TokenType::Bang, |p| p.parse_prefix_expression());
        parser.register_prefix(TokenType::Minus, |p| p.parse_prefix_expression());
        parser.register_prefix(TokenType::True, |p| p.parse_boolean());
        parser.register_prefix(TokenType::False, |p| p.parse_boolean());
        parser.register_prefix(TokenType::Lparen, |p| p.parse_grouped_expression());
        parser.register_prefix(TokenType::If, |p| p.parse_if_expression());

        parser.register_infix(TokenType::Plus, |p, a| p.parse_infix_expression(a));
        parser.register_infix(TokenType::Minus, |p, a| p.parse_infix_expression(a));
        parser.register_infix(TokenType::Asterisk, |p, a| p.parse_infix_expression(a));
        parser.register_infix(TokenType::Slash, |p, a| p.parse_infix_expression(a));
        parser.register_infix(TokenType::Lt, |p, a| p.parse_infix_expression(a));
        parser.register_infix(TokenType::Gt, |p, a| p.parse_infix_expression(a));
        parser.register_infix(TokenType::Eq, |p, a| p.parse_infix_expression(a));
        parser.register_infix(TokenType::Neq, |p, a| p.parse_infix_expression(a));
        parser.register_infix(TokenType::Assign, |p, a| p.parse_infix_expression(a));

        parser.register_prefix(TokenType::Func, |p| p.parse_function_literal());
        parser.register_infix(TokenType::Lparen, |p, a| {
            p.parse_function_call_expression(a)
        });

        parser
    }

    pub fn parse(&mut self) -> ParsingResult<ast::Program> {
        self.parse_program()
    }

    pub fn reset(&mut self, input: &'a str) {
        self.lexer.reset(input);
        self.next_token();
        self.next_token();
        self.errors = vec![];
    }

    fn register_prefix(&mut self, token_type: TokenType, prefix_parse_fn: PrefixParseFn) {
        self.settings
            .prefix_parse_funcs
            .insert(token_type, prefix_parse_fn);
    }

    fn register_infix(&mut self, token_type: TokenType, infix_parse_fn: InfixParseFn) {
        self.settings
            .infix_parse_funcs
            .insert(token_type, infix_parse_fn);
    }

    pub fn next_token(&mut self) {
        self.curr_token = mem::replace(&mut self.next_token, self.lexer.next_token())
        // self.curr_token = self.next_token;
        // self.next_token = self.lexer.next_token();
    }

    pub fn next_if(&mut self, expected_type: TokenType) {
        if self.next_token.token_type == expected_type {
            self.next_token();
        }
    }

    pub fn expect_next(&mut self, expected_type: TokenType, code: &str) -> bool {
        if self.next_token.token_type == expected_type {
            // self.next_token();
            true
        }
        else {
            // if error append errros
            self.raise_error(code, expected_type);
            false
        }
    }

    pub fn raise_error(&mut self, code: &str, expected: TokenType) {
        let found = &self.next_token;
        let err = format!(
            "Line {}:{} {}: expected {}, found {}",
            found.row, found.column, code, expected, found
        );
        self.errors.push(err);
    }

    pub fn token_2_prefix(token: TokenType) -> PrefixType {
        match token {
            TokenType::Minus => PrefixType::Minus,
            TokenType::Bang => PrefixType::Bang,
            _ => unreachable!("123"),
        }
    }

    pub fn token_2_infix(token: TokenType) -> InfixType {
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
            _ => unreachable!("123"),
        }
    }
}

impl<'a> Parser<'a> {
    fn parse_program(&mut self) -> ParsingResult<ast::Program> {
        let mut program = ast::Program { stmts: vec![] };
        while self.curr_token.token_type != TokenType::Eof {
            if let Some(stmt) = self.parse_statement().ok() {
                program.stmts.push(stmt);
            }
            self.next_token();
        }

        if self.errors.is_empty() {
            Ok(program)
        }
        else {
            Err(self.errors.clone())
        }
    }
}
