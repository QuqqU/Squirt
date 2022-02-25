pub type TokenType = &'static str;
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

pub fn is_letter(c: char) -> bool {
    'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_'
}

pub fn is_digit(c: char) -> bool {
    '0' <= c && c <= '9'
}

pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";

pub const IDENT: TokenType = "IDENT";
pub const INT: TokenType = "INT";

pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";
pub const MINUS: TokenType = "-";
pub const ASTERISK: TokenType = "*";
pub const SLASH: TokenType = "/";
pub const BANG: TokenType = "!";

pub const LT: TokenType = "<";
pub const GT: TokenType = ">";
pub const EQ: TokenType = "==";
pub const NEQ: TokenType = "!=";


pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";

pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";

pub const LET: TokenType = "LET";
pub const FUNC: TokenType = "FUNCTION";
pub const TRUE: TokenType = "TRUE";
pub const FALSE: TokenType = "FALSE";
pub const IF: TokenType = "IF";
pub const ELSE: TokenType = "ELSE";
pub const RETURN: TokenType = "RETURN";
