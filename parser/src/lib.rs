mod expression;
mod parser;
mod parsersettings;
mod statement;

pub use crate::parser::Parser;
use crate::parsersettings::Priority;

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
macro_rules! check_curr {
    ($self:ident, $code:expr, $exp:expr) => {
        if !$self.check_curr(&[$exp]) {
            $self.raise_err($code, &format!("expected {}", $exp));
            return Err(());
        }
    };
    ($self:ident, $code:expr, $exp_arr:expr, $err:expr) => {
        if !$self.check_curr($exp_arr) {
            $self.raise_err($code, $err);
            return Err(());
        }
    };
}

#[macro_export]
macro_rules! ensure_curr {
    ($self:ident, $code:expr, $exp:expr) => {
        if !$self.check_curr(&[$exp]) {
            $self.raise_err($code, &format!("expected {}", $exp));
            return Err(());
        }
        $self.next_token();
    };
    ($self:ident, $code:expr, $exp_arr:expr, $err:expr) => {
        if !$self.check_curr($exp_arr) {
            $self.raise_err($code, $err);
            return Err(());
        }
        $self.next_token();
    };
}
