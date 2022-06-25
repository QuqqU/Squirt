pub type TokenLiteral = &'static str;

pub const ILLEGAL: TokenLiteral = "ILLEGAL";
pub const EOF: TokenLiteral = "EOF";

pub const IDENT: TokenLiteral = "IDENT";
pub const INT: TokenLiteral = "INT";

pub const ASSIGN: TokenLiteral = "=";
pub const PLUS: TokenLiteral = "+";
pub const MINUS: TokenLiteral = "-";
pub const ASTERISK: TokenLiteral = "*";
pub const SLASH: TokenLiteral = "/";
pub const BANG: TokenLiteral = "!";

pub const LT: TokenLiteral = "<";
pub const GT: TokenLiteral = ">";
pub const EQ: TokenLiteral = "==";
pub const NEQ: TokenLiteral = "!=";

pub const COMMA: TokenLiteral = ",";
pub const SEMICOLON: TokenLiteral = ";";

pub const LPAREN: TokenLiteral = "(";
pub const RPAREN: TokenLiteral = ")";
pub const LBRACE: TokenLiteral = "{";
pub const RBRACE: TokenLiteral = "}";

pub const LET: TokenLiteral = "LET";
pub const FUNC: TokenLiteral = "FUNCTION";
pub const TRUE: TokenLiteral = "TRUE";
pub const FALSE: TokenLiteral = "FALSE";
pub const IF: TokenLiteral = "IF";
pub const ELSE: TokenLiteral = "ELSE";
pub const RETURN: TokenLiteral = "RETURN";
