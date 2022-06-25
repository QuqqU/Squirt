use std::{iter::Peekable, str::Chars};

use token::{look_up_ident, Token, TokenType};

pub struct Lexer<'a> {
    pub input:   Peekable<Chars<'a>>,
    pub poisons: Vec<char>,
    ch:          char,
    row:         i64,
    column:      i64,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input:   input.chars().peekable(),
            poisons: vec![],
            ch:      ' ',
            row:     1,
            column:  0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();

        let row = self.row;
        let column = self.column;

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    Token {
                        token_type: TokenType::Eq,
                        literal: token::EQ.to_string(),
                        row,
                        column,
                    }
                }
                else {
                    Token {
                        token_type: TokenType::Assign,
                        literal: token::ASSIGN.to_string(),
                        row,
                        column,
                    }
                }
            }
            '+' => Token {
                token_type: TokenType::Plus,
                literal: token::PLUS.to_string(),
                row,
                column,
            },
            '-' => Token {
                token_type: TokenType::Minus,
                literal: token::MINUS.to_string(),
                row,
                column,
            },
            '*' => Token {
                token_type: TokenType::Asterisk,
                literal: token::ASTERISK.to_string(),
                row,
                column,
            },
            '/' => Token {
                token_type: TokenType::Slash,
                literal: token::SLASH.to_string(),
                row,
                column,
            },
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token {
                        token_type: TokenType::Neq,
                        literal: token::NEQ.to_string(),
                        row,
                        column,
                    }
                }
                else {
                    Token {
                        token_type: TokenType::Bang,
                        literal: token::BANG.to_string(),
                        row,
                        column,
                    }
                }
            }
            '<' => Token {
                token_type: TokenType::Lt,
                literal: token::LT.to_string(),
                row,
                column,
            },
            '>' => Token {
                token_type: TokenType::Gt,
                literal: token::GT.to_string(),
                row,
                column,
            },
            ',' => Token {
                token_type: TokenType::Comma,
                literal: token::COMMA.to_string(),
                row,
                column,
            },
            ';' => Token {
                token_type: TokenType::Semicolon,
                literal: token::SEMICOLON.to_string(),
                row,
                column,
            },
            '(' => Token {
                token_type: TokenType::Lparen,
                literal: token::LPAREN.to_string(),
                row,
                column,
            },
            ')' => Token {
                token_type: TokenType::Rparen,
                literal: token::RPAREN.to_string(),
                row,
                column,
            },
            '{' => Token {
                token_type: TokenType::Lbrace,
                literal: token::LBRACE.to_string(),
                row,
                column,
            },
            '}' => Token {
                token_type: TokenType::Rbrace,
                literal: token::RBRACE.to_string(),
                row,
                column,
            },
            '\0' => Token {
                token_type: TokenType::Eof,
                literal: token::EOF.to_string(),
                row,
                column,
            },
            _ => {
                if self.is_letter() {
                    let s = self.read_ident();
                    return Token {
                        token_type: look_up_ident(&s),
                        literal: s,
                        row,
                        column,
                    };
                }
                else if self.is_digit() {
                    return Token {
                        token_type: TokenType::Int,
                        literal: self.read_number(),
                        row,
                        column,
                    };
                }
                else {
                    self.poisons.push(self.ch.clone());
                    return Token {
                        token_type: TokenType::Poison,
                        literal: self.ch.to_string(),
                        row,
                        column,
                    };
                }
            }
        };

        self.read_char();
        return token;
    }
}

impl<'a> Lexer<'a> {
    fn read_char(&mut self) {
        self.ch = self.input.next().unwrap_or('\0');
        self.column += 1;

        if self.ch == '\n' {
            self.row += 1;
            self.column = 0;
        }
    }

    fn peek_char(&mut self) -> char {
        *self.input.peek().unwrap_or(&'\0')
    }

    fn skip_whitespaces(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\r' || self.ch == '\n' {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        while self.is_letter() {
            s.push(self.ch);
            self.read_char();
        }
        s
    }

    fn read_number(&mut self) -> String {
        let mut s = String::new();
        while self.is_digit() {
            s.push(self.ch);
            self.read_char();
        }
        s
    }

    fn is_letter(&self) -> bool {
        let c = self.ch;
        'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_'
    }

    fn is_digit(&self) -> bool {
        let c = self.ch;
        '0' <= c && c <= '9'
    }
}
