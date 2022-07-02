use std::collections::HashMap;
use std::mem;

use ast::{InfixType, PrefixType};
use lexer::token::{Token, TokenType};
use lexer::Lexer;

use crate::{InfixParseFn, ParsingResult, PrefixParseFn};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Priority {
    Lowest,
    Assign,
    Equal,
    Compare,
    Sum,
    Product,
    Prefix,
    Call,
}

pub(super) struct ParserSettings {
    pub precedences:        HashMap<TokenType, Priority>,
    pub prefix_parse_funcs: HashMap<TokenType, PrefixParseFn>,
    pub infix_parse_funcs:  HashMap<TokenType, InfixParseFn>,
}
impl ParserSettings {
    pub(super) fn new() -> Self {
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
    pub(super) lexer:      Lexer<'a>,
    pub(super) curr_token: Token,
    pub(super) next_token: Token,
    pub(super) errors:     Vec<String>,
    pub(super) settings:   ParserSettings,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Lexer::new(input);

        let ctoken = l.next_token();
        let ntoken = l.next_token();
        let settings = ParserSettings::new();
        let mut parser = Self {
            lexer: l,
            curr_token: ctoken,
            next_token: ntoken,
            errors: vec![],
            settings,
        };

        parser.register_prefix(TokenType::Ident, |p| p.parse_ident());
        parser.register_prefix(TokenType::Int, |p| p.parse_int());
        parser.register_prefix(TokenType::Bang, |p| p.parse_prefix_expr());
        parser.register_prefix(TokenType::Minus, |p| p.parse_prefix_expr());
        parser.register_prefix(TokenType::True, |p| p.parse_bool());
        parser.register_prefix(TokenType::False, |p| p.parse_bool());
        parser.register_prefix(TokenType::Lparen, |p| p.parse_grouped_expr());
        parser.register_prefix(TokenType::If, |p| p.parse_if_expr());

        parser.register_infix(TokenType::Plus, |p, a| p.parse_infix_expr(a));
        parser.register_infix(TokenType::Minus, |p, a| p.parse_infix_expr(a));
        parser.register_infix(TokenType::Asterisk, |p, a| p.parse_infix_expr(a));
        parser.register_infix(TokenType::Slash, |p, a| p.parse_infix_expr(a));
        parser.register_infix(TokenType::Lt, |p, a| p.parse_infix_expr(a));
        parser.register_infix(TokenType::Gt, |p, a| p.parse_infix_expr(a));
        parser.register_infix(TokenType::Eq, |p, a| p.parse_infix_expr(a));
        parser.register_infix(TokenType::Neq, |p, a| p.parse_infix_expr(a));
        parser.register_infix(TokenType::Assign, |p, a| p.parse_infix_expr(a));

        parser
    }

    pub fn parse(&mut self) -> ParsingResult<ast::Program> {
        let mut program = ast::Program { stmts: vec![] };
        while self.curr_token.token_type != TokenType::Eof {
            while self.curr_token.token_type == TokenType::Semicolon {
                self.next_token();
            }

            if let Some(stmt) = self.parse_stmt().ok() {
                program.stmts.push(stmt);
            }
            else {
                self.skip_stmt();
            }
        }

        if self.errors.is_empty() {
            Ok(program)
        }
        else {
            Err(self.errors.clone())
        }
    }

    pub fn reset(&mut self, input: &'a str) {
        self.lexer.reset(input);
        self.curr_token = self.lexer.next_token();
        self.next_token = self.lexer.next_token();
        self.errors = vec![];
    }

    pub(super) fn skip_stmt(&mut self) {
        while self.curr_token.token_type != TokenType::Semicolon
            && self.curr_token.token_type != TokenType::Eof
        {
            self.next_token();
        }
        self.next_token();
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

    #[inline]
    pub(super) fn next_token(&mut self) {
        self.curr_token = mem::replace(&mut self.next_token, self.lexer.next_token())
    }

    pub(super) fn next_if(&mut self, curr_token_type: TokenType) -> bool {
        if self.curr_token.token_type == curr_token_type {
            self.next_token();
            true
        }
        else {
            false
        }
    }

    #[inline]
    pub(super) fn verify_curr(&mut self, code: &str, curr_token_type: TokenType) -> bool {
        if self.curr_token.token_type == curr_token_type {
            true
        }
        else {
            self.raise_error(code, curr_token_type);
            false
        }
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn raise_error(&mut self, code: &str, expected: TokenType) {
        let found = &self.curr_token;
        let err = format!(
            "Line {}:{} {} - expected {}, found {}",
            found.row, found.column, code, expected, found
        );
        self.errors.push(err);
    }

    pub(crate) fn token_2_prefix(token: TokenType) -> PrefixType {
        match token {
            TokenType::Minus => PrefixType::Minus,
            TokenType::Bang => PrefixType::Bang,
            _ => unreachable!("123"),
        }
    }

    pub(crate) fn token_2_infix(token: TokenType) -> InfixType {
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
