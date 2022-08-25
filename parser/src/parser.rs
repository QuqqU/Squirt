use token::{Token, TokenType};

use crate::parsersettings::ParserSettings;
use crate::ParsingResult;

pub struct Parser {
    tokens:              Vec<Token>,
    idx:                 usize,
    errors:              Vec<String>,
    pub(super) settings: ParserSettings,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> ParsingResult<ast::Program> {
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }

    fn new(mut tokens: Vec<Token>) -> Self {
        tokens.push(Token::new(TokenType::Eof, "\0", 0, 0));
        let tokens = tokens;
        Self {
            tokens,
            idx: 0,
            errors: vec![],
            settings: ParserSettings::new(),
        }
    }

    fn parse_program(&mut self) -> ParsingResult<ast::Program> {
        let mut program = vec![];
        while self.curr_token().token_type != TokenType::Eof {
            while self.curr_token().token_type == TokenType::Semicolon {
                self.consume_token();
            }

            if let Some(stmt) = self.parse_stmt().ok() {
                program.push(stmt);
            }
            else {
                self.skip_stmt();
            }
        }

        if self.errors.is_empty() {
            Ok(ast::Program { program })
        }
        else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    pub(super) fn skip_stmt(&mut self) {
        while self.curr_token().token_type != TokenType::Semicolon
            && self.curr_token().token_type != TokenType::Eof
        {
            self.consume_token();
        }
        self.consume_token();
    }

    #[inline]
    pub(super) fn consume_token(&mut self) {
        self.idx += 1;
    }

    #[inline]
    pub(super) fn curr_token(&self) -> &Token {
        &self.tokens[self.idx]
    }

    pub(super) fn next_if(&mut self, curr_token_type: TokenType) -> bool {
        if self.curr_token().token_type == curr_token_type {
            self.consume_token();
            true
        }
        else {
            false
        }
    }
}

impl Parser {
    #[inline]
    pub(super) fn check_curr(&mut self, expected: &[TokenType]) -> bool {
        expected.contains(&self.curr_token().token_type)
    }

    pub(crate) fn raise_err(&mut self, code: &str, err_message: &str) {
        let found = &self.curr_token();
        let err = format!(
            "Line {}:{} {} - {}, found {}",
            found.row, found.column, code, err_message, found
        );
        self.errors.push(err);
    }
}
