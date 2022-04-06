use super::keyword::*;

pub fn look_up_ident(s: &str) -> TokenType {
    match s {
        "let" => LET,
        "fn" => FUNC,
        "true" => TRUE,
        "false" => FALSE,
        "if" => IF,
        "else" => ELSE,
        "return" => RETURN,
        _ => IDENT,
    }
}
