use std::fmt;

#[derive(PartialEq, Eq, Hash)]
pub enum TokenType {
    Poison,
    Eof,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Lt,
    Gt,
    Eq,
    Neq,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Let,
    Func,
    True,
    False,
    If,
    Else,
    Return,
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token = match *self {
            TokenType::Poison => "__poison__",
            TokenType::Eof => "__EOF__",
            TokenType::Ident => "__ident__",
            TokenType::Int => "__int__",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",
            TokenType::Bang => "!",
            TokenType::Lt => "<",
            TokenType::Gt => ">",
            TokenType::Eq => "==",
            TokenType::Neq => "!=",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Lparen => "(",
            TokenType::Rparen => ")",
            TokenType::Lbrace => "{",
            TokenType::Rbrace => "}",
            TokenType::Let => "let",
            TokenType::Func => "fn",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Return => "return",
        };
        write!(f, "{}", token)
    }
}

pub fn look_up_ident(s: &str) -> TokenType {
    match s {
        "let" => TokenType::Let,
        "fn" => TokenType::Func,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Ident,
    }
}

#[test]
fn look_up_identifier() {
    assert_eq!(
        TokenType::Let,
        look_up_ident(&format!("{:?}", TokenType::Let))
    );
    assert_eq!(
        TokenType::Func,
        look_up_ident(&format!("{:?}", TokenType::Func))
    );
    assert_eq!(
        TokenType::True,
        look_up_ident(&format!("{:?}", TokenType::True))
    );
    assert_eq!(
        TokenType::False,
        look_up_ident(&format!("{:?}", TokenType::False))
    );
    assert_eq!(
        TokenType::If,
        look_up_ident(&format!("{:?}", TokenType::If))
    );
    assert_eq!(
        TokenType::Else,
        look_up_ident(&format!("{:?}", TokenType::Else))
    );
    assert_eq!(
        TokenType::Return,
        look_up_ident(&format!("{:?}", TokenType::Return))
    );
    assert_eq!(TokenType::Ident, look_up_ident("abc"));
}
