use std::collections::HashMap;

use lexer::TokenType;

use crate::{InfixParseFn, PrefixParseFn};

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
        let mut p = Self {
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
        };

        p.register_prefix(TokenType::Ident, |p| p.parse_ident());
        p.register_prefix(TokenType::Int, |p| p.parse_int());
        p.register_prefix(TokenType::Bang, |p| p.parse_prefix_expr());
        p.register_prefix(TokenType::Minus, |p| p.parse_prefix_expr());
        p.register_prefix(TokenType::True, |p| p.parse_bool());
        p.register_prefix(TokenType::False, |p| p.parse_bool());
        p.register_prefix(TokenType::Lparen, |p| p.parse_grouped_expr());
        p.register_prefix(TokenType::If, |p| p.parse_if_expr());

        p.register_infix(TokenType::Plus, |p, a| p.parse_infix_expr(a));
        p.register_infix(TokenType::Minus, |p, a| p.parse_infix_expr(a));
        p.register_infix(TokenType::Asterisk, |p, a| p.parse_infix_expr(a));
        p.register_infix(TokenType::Slash, |p, a| p.parse_infix_expr(a));
        p.register_infix(TokenType::Lt, |p, a| p.parse_infix_expr(a));
        p.register_infix(TokenType::Gt, |p, a| p.parse_infix_expr(a));
        p.register_infix(TokenType::Eq, |p, a| p.parse_infix_expr(a));
        p.register_infix(TokenType::Neq, |p, a| p.parse_infix_expr(a));
        p.register_infix(TokenType::Assign, |p, a| p.parse_infix_expr(a));

        p.register_prefix(TokenType::Func, |p| p.parse_func_literal_expr());
        p.register_infix(TokenType::Lparen, |p, a| p.parse_func_call_expr(a));

        p
    }

    pub(super) fn precedence_of(&self, token_type: &TokenType) -> Priority {
        let p = self.precedences.get(token_type);
        if let Some(&p) = p {
            p
        }
        else {
            Priority::Lowest
        }
    }

    fn register_prefix(&mut self, token_type: TokenType, prefix_parse_fn: PrefixParseFn) {
        self.prefix_parse_funcs.insert(token_type, prefix_parse_fn);
    }

    fn register_infix(&mut self, token_type: TokenType, infix_parse_fn: InfixParseFn) {
        self.infix_parse_funcs.insert(token_type, infix_parse_fn);
    }
}

#[test]
fn priority() {
    assert!(
        Priority::Lowest < Priority::Assign
            && Priority::Assign < Priority::Equal
            && Priority::Equal < Priority::Compare
            && Priority::Compare < Priority::Sum
            && Priority::Sum < Priority::Product
            && Priority::Product < Priority::Prefix
            && Priority::Prefix < Priority::Call
    );
}
