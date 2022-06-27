use super::token::{look_up_ident, Token, TokenType};
use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    pub input: Peekable<Chars<'a>>,
    ch:        char,
    row:       i64,
    column:    i64,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input:  input.chars().peekable(),
            ch:     ' ',
            row:    1,
            column: 0,
        }
    }

    pub fn reset(&mut self, input: &'a str) {
        self.input = input.chars().peekable();
        self.ch = ' ';
        self.row = 1;
        self.column = 0;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();

        let row = self.row;
        let column = self.column;

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::Eq, "==", row, column)
                }
                else {
                    Token::new(TokenType::Assign, "=", row, column)
                }
            }
            '+' => Token::new(TokenType::Plus, "+", row, column),
            '-' => Token::new(TokenType::Minus, "-", row, column),
            '*' => Token::new(TokenType::Asterisk, "*", row, column),
            '/' => Token::new(TokenType::Slash, "/", row, column),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::Neq, "!=", row, column)
                }
                else {
                    Token::new(TokenType::Bang, "!", row, column)
                }
            }
            '<' => Token::new(TokenType::Lt, "<", row, column),
            '>' => Token::new(TokenType::Gt, ">", row, column),
            ',' => Token::new(TokenType::Comma, ",", row, column),
            ';' => Token::new(TokenType::Semicolon, ";", row, column),
            '(' => Token::new(TokenType::Lparen, "(", row, column),
            ')' => Token::new(TokenType::Rparen, ")", row, column),
            '{' => Token::new(TokenType::Lbrace, "{", row, column),
            '}' => Token::new(TokenType::Rbrace, "}", row, column),
            '\0' => Token::new(TokenType::Eof, "\0", row, column),
            _ => {
                if self.is_letter() {
                    let s = &self.read_ident();
                    return Token::new(look_up_ident(s), s, row, column);
                }
                else if self.is_digit() {
                    return Token::new(TokenType::Int, &self.read_number(), row, column);
                }
                else {
                    return Token::new(
                        TokenType::Poison,
                        self.ch.to_string().as_str(),
                        row,
                        column,
                    );
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
