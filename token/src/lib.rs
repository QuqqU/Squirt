mod keyword;
mod util;

pub use keyword::*;
pub use util::*;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal:    String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}
