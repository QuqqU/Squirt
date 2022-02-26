use lexer::Lexer;
use token::Token;

pub struct Parser {
    lexer:      Lexer,
    curr_token: Token,
    next_token: Token,
    // todo errors
    // errors:     Vec<String>,
}

impl Parser {
    pub fn new(mut l: Lexer) -> Self {
        let ctoken = l.next_token();
        let ntoken = l.next_token();

        Self {
            lexer:      l,
            curr_token: ctoken,
            next_token: ntoken,
            // errors:     vec![],
        }
    }

    fn next_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn expect_next(&mut self, expected_type: token::TokenType) -> bool {
        if self.next_token.token_type == expected_type {
            self.next_token();
            true
        }
        else {
            // if error append errros
            false
        }
    }
}

impl Parser {
    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program { statements: vec![] };
        while self.curr_token.token_type != token::EOF {
            let stmt = self.parse_statement();
            program.statements.push(stmt);
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> ast::Statement {
        match self.curr_token.token_type {
            token::LET => self.parse_let_statement(),
            token::RETURN => self.parse_return_statement(),
            _ => ast::Statement::Undefined {},
        }
    }

    fn parse_let_statement(&mut self) -> ast::Statement {
        let token_type = self.curr_token.token_type;

        if !self.expect_next(token::IDENT) {
            panic!("PAR0001: No ident after LET");
            // if error return null
        }

        let var_name = ast::Identifier {
            token: self.curr_token.token_type,
            value: self.curr_token.literal.clone(),
        };

        if !self.expect_next(token::ASSIGN) {
            panic!("PAR0002: No ASSIGN sign after ident");
            // if error return null
        }

        //todo expression
        while self.curr_token.token_type != token::SEMICOLON {
            self.next_token();
        }

        let stmt = ast::Statement::Let {
            token: token_type,
            name:  var_name,
            value: ast::Expression {},
        };

        stmt
    }

    fn parse_return_statement(&mut self) -> ast::Statement {
        let token_type = self.curr_token.token_type;
        self.next_token();

        //todo expression
        while self.curr_token.token_type != token::SEMICOLON {
            self.next_token();
        }

        let stmt = ast::Statement::Return {
            token: token_type,
            value: ast::Expression {},
        };

        stmt
    }
}
