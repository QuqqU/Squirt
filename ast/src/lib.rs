
use token::TokenType;

pub struct Identifier {
    pub token: TokenType, // token::IDENT
    pub value: String,
}

pub struct Expression {}

pub enum Statement {
    Undefined,
    Let {
        token: TokenType, // token::LET
        name:  Identifier,
        value: Expression,
    },
}

pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn empty(&self) -> bool {
        self.statements.is_empty()
    }
}

