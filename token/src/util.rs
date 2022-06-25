use super::*;

pub fn look_up_ident(s: String) -> Token {
    match s.as_str() {
        "let" => Token::Let,
        "fn" => Token::Func,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(s),
    }
}
