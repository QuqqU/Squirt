use std::collections::HashMap;

use ast;
use lexer::Lexer;
use token::{Token, TokenLiteral};

mod expression;
mod statement;
use expression::{InfixParseFn, PrefixParseFn};

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

pub type ParsingResult<T> = Result<T, &'static str>;

pub struct Parser<'a> {
    lexer:      Lexer<'a>,
    curr_token: Token,
    next_token: Token,

    // todo errors
    // errors:     Vec<String>,
    precedences:        HashMap<TokenLiteral, Priority>,
    prefix_parse_funcs: HashMap<TokenLiteral, PrefixParseFn>,
    infix_parse_funcs:  HashMap<TokenLiteral, InfixParseFn>,
}

impl<'a> Parser<'a> {
    pub fn parse(input: &'a str) -> ParsingResult<ast::Program> {
        let mut l = Lexer::new(input);

        let ctoken = l.next_token();
        let ntoken = l.next_token();

        let mut p = Self {
            lexer:              l,
            curr_token:         ctoken,
            next_token:         ntoken,
            // errors:     vec![],
            precedences:        HashMap::from([
                (token::ASSIGN, Priority::Assign),
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
        p.register_infix(token::ASSIGN, Parser::parse_infix_expression);

        p.register_prefix(token::TRUE, Parser::parse_boolean);
        p.register_prefix(token::FALSE, Parser::parse_boolean);

        p.register_prefix(token::LPAREN, Parser::parse_grouped_expression);
        p.register_prefix(token::IF, Parser::parse_if_expression);
        p.register_prefix(token::FUNC, Parser::parse_function_literal);
        p.register_infix(token::LPAREN, Parser::parse_function_call_expression);

        p.parse_program()
    }

    fn next_token(&mut self) {
        self.curr_token = self.next_token;
        self.next_token = self.lexer.next_token();
    }

    fn expect_next(&mut self, expected_type: Token) -> bool {
        if self.next_token == expected_type {
            self.next_token();
            true
        }
        else {
            // if error append errros
            false
        }
    }
}

impl<'a> Parser<'a> {
    fn parse_program(&mut self) -> ParsingResult<ast::Program> {
        let mut program = ast::Program { statements: vec![] };
        while self.curr_token != Token::Eof {
            let stmt = self.parse_statement();
            program.statements.push(stmt);
            self.next_token();
        }
        program
    }
}
