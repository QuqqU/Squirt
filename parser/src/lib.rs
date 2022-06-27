mod expression;
mod parser;

mod statement;

pub use parser::Parser;
use parser::Priority;

pub type ParsingResult<T> = Result<T, Vec<String>>;
type PartParsingResult<T> = Result<T, ()>;
type PrefixParseFn = fn(&mut Parser) -> PartParsingResult<ast::Expr>;
type InfixParseFn = fn(&mut Parser, ast::Expr) -> PartParsingResult<ast::Expr>;

#[macro_export]
macro_rules! try_parse {
    ($self:ident, $func:ident) => {
        try_parse!($self, $func, )
    };
    ($self:ident, $func:ident, $($arg:expr),*) => {
        match $self.$func($($arg),*) {
            Ok(ast) => ast,
            Err(err) => return Err(err)
        }
    };
}

#[macro_export]
macro_rules! expect_token {
    ($self:ident, $exp:ident, $code:ident) => {
        // if $self.$func($())
    };
}
