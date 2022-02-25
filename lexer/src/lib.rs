use token::{self, is_digit, is_letter};

pub struct Lexer {
    pub input:    Vec<char>,
    pub position: usize,
    pub ch:       char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input:    input.chars().collect(),
            position: 0,
            ch:       ' ',
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespaces();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token::Token::new(token::EQ, "==".to_string())
                }
                else {
                    token::Token::new(token::ASSIGN, self.ch.to_string())
                }
            }
            '+' => token::Token::new(token::PLUS, self.ch.to_string()),
            '-' => token::Token::new(token::MINUS, self.ch.to_string()),
            '*' => token::Token::new(token::ASTERISK, self.ch.to_string()),
            '/' => token::Token::new(token::SLASH, self.ch.to_string()),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token::Token::new(token::NEQ, "!=".to_string())
                }
                else {
                    token::Token::new(token::BANG, self.ch.to_string())
                }
            }
            '<' => token::Token::new(token::LT, self.ch.to_string()),
            '>' => token::Token::new(token::GT, self.ch.to_string()),
            ',' => token::Token::new(token::COMMA, self.ch.to_string()),
            ';' => token::Token::new(token::SEMICOLON, self.ch.to_string()),
            '(' => token::Token::new(token::LPAREN, self.ch.to_string()),
            ')' => token::Token::new(token::RPAREN, self.ch.to_string()),
            '{' => token::Token::new(token::LBRACE, self.ch.to_string()),
            '}' => token::Token::new(token::RBRACE, self.ch.to_string()),
            '\0' => token::Token::new(token::EOF, self.ch.to_string()),
            _ => {
                if is_letter(self.ch) {
                    let s = self.read_ident();
                    return token::Token::new(token::look_up_ident(&s), s);
                }
                else if is_digit(self.ch) {
                    return token::Token::new(token::INT, self.read_number());
                }
                else {
                    // panic!("LXR0001: Unexpected Character \"{}\"", self.ch);
                    return token::Token::new(token::ILLEGAL, self.ch.to_string());
                }
            }
        };

        self.read_char();
        return token;
    }

    fn skip_whitespaces(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\r' || self.ch == '\n' {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.ch = '\0';
        }
        else {
            self.ch = self.input[self.position];
        }
        self.position += 1;
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        while is_letter(self.ch) {
            s.push(self.ch);
            self.read_char();
        }
        s
    }

    fn read_number(&mut self) -> String {
        let mut s = String::new();
        while is_digit(self.ch) {
            s.push(self.ch);
            self.read_char();
        }
        s
    }

    fn peek_char(&mut self) -> char {
        if self.position >= self.input.len() {
            '\0'
        }
        else {
            self.input[self.position]
        }
    }
}
