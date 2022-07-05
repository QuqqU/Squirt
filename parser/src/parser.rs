use std::mem;

use lexer::token::{Token, TokenType};
use lexer::Lexer;

use crate::parsersettings::ParserSettings;
use crate::ParsingResult;

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

        Self {
            lexer: l,
            curr_token: ctoken,
            next_token: ntoken,
            errors: vec![],
            settings,
        }
    }

    pub fn reset(&mut self, input: &'a str) {
        self.lexer.reset(input);
        self.curr_token = self.lexer.next_token();
        self.next_token = self.lexer.next_token();
        self.errors = vec![];
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

    pub(super) fn skip_stmt(&mut self) {
        while self.curr_token.token_type != TokenType::Semicolon
            && self.curr_token.token_type != TokenType::Eof
        {
            self.next_token();
        }
        self.next_token();
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
}

impl<'a> Parser<'a> {
    #[inline]
    pub(super) fn check_curr(&mut self, expected: &[TokenType]) -> bool {
        expected.contains(&self.curr_token.token_type)
    }

    pub(crate) fn raise_err(&mut self, code: &str, err_message: &str) {
        let found = &self.curr_token;
        let err = format!(
            "Line {}:{} {} - {}, found {}",
            found.row, found.column, code, err_message, found
        );
        self.errors.push(err);
    }
}
