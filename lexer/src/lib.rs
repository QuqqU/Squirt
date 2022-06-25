use std::{iter::Peekable, str::Chars};

use token::{look_up_ident, Token};

pub struct Lexer<'a> {
    pub input:   Peekable<Chars<'a>>,
    pub poisons: Vec<char>,
    ch:          char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input:   input.chars().peekable(),
            poisons: vec![],
            ch:      ' ',
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                }
                else {
                    Token::Assign
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Neq
                }
                else {
                    Token::Bang
                }
            }
            '<' => Token::Lt,
            '>' => Token::Gt,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            '{' => Token::Lbrace,
            '}' => Token::Rbrace,
            '\0' => Token::Eof,
            _ => {
                if self.is_letter() {
                    let s = self.read_ident();
                    return look_up_ident(s);
                }
                else if self.is_digit() {
                    return Token::Int(self.read_number());
                }
                else {
                    self.poisons.push(self.ch);
                    return Token::Poison;
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

    fn read_number(&mut self) -> i64 {
        let mut s = String::new();
        while self.is_digit() {
            s.push(self.ch);
            self.read_char();
        }
        s.parse::<i64>().unwrap()
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
