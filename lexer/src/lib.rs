use token::Token;

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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();

        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(token::EQ, "==".to_string())
                }
                else {
                    Token::new(token::ASSIGN, self.ch.to_string())
                }
            }
            '+' => Token::new(token::PLUS, self.ch.to_string()),
            '-' => Token::new(token::MINUS, self.ch.to_string()),
            '*' => Token::new(token::ASTERISK, self.ch.to_string()),
            '/' => Token::new(token::SLASH, self.ch.to_string()),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(token::NEQ, "!=".to_string())
                }
                else {
                    Token::new(token::BANG, self.ch.to_string())
                }
            }
            '<' => Token::new(token::LT, self.ch.to_string()),
            '>' => Token::new(token::GT, self.ch.to_string()),
            ',' => Token::new(token::COMMA, self.ch.to_string()),
            ';' => Token::new(token::SEMICOLON, self.ch.to_string()),
            '(' => Token::new(token::LPAREN, self.ch.to_string()),
            ')' => Token::new(token::RPAREN, self.ch.to_string()),
            '{' => Token::new(token::LBRACE, self.ch.to_string()),
            '}' => Token::new(token::RBRACE, self.ch.to_string()),
            '\0' => Token::new(token::EOF, self.ch.to_string()),
            _ => {
                if self.is_letter() {
                    let s = self.read_ident();
                    return Token::new(token::look_up_ident(&s), s);
                }
                else if self.is_digit() {
                    return Token::new(token::INT, self.read_number());
                }
                else {
                    // panic!("LXR0001: Unexpected Character \"{}\"", self.ch);
                    return Token::new(token::ILLEGAL, self.ch.to_string());
                }
            }
        };

        self.read_char();
        return token;
    }
}

impl Lexer {
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

    fn peek_char(&mut self) -> char {
        if self.position >= self.input.len() {
            '\0'
        }
        else {
            self.input[self.position]
        }
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
