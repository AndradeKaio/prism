
//mod lexer;

use crate::lexer::{Lexer, Token, TokenType, Keyword, Symbol};

macro_rules! match_tok {
    ($e:expr, $p:pat) => {
        match $e {
            $p => true,
            _ => false
        }
    }
}

pub struct Parser <'a> {
    pub lexer: &'a mut Lexer<'a>,
    pub cur_tok : Option<Token>,
}

impl <'a> Parser <'a>{
    
    pub fn new(lexer: &'a mut Lexer <'a>) -> Self {
        Parser {
            lexer,
            cur_tok: None,
        }

    }

    pub fn match_tok(&mut self, token_type: TokenType) -> bool {
        
        let token = self.cur_tok.unwrap();
        self.cur_tok = Some(self.lexer.next().unwrap());

        assert_eq!(token.token_type, token_type, "Expected token '{:?}' find '{:?}' {:?} at line {}.", 
        token_type, token.token_type, self.lexer.get_lexem(token.lexem.start..token.lexem.end), token.line);
        true
    }
    pub fn parse(&mut self){
        self.cur_tok = Some(self.lexer.next().unwrap());
        //while !self.cur_tok.is_none() && self.cur_tok.unwrap().token_type != TokenType::Eof {
            println!("{:?}", self.cur_tok);
            self.stmt(); 
        //}
    }

    pub fn paren_expr(&mut self) {
       println!("-> paren_expr()");
       self.match_tok(TokenType::OpenParen);
       self.expr();      
       self.match_tok(TokenType::CloseParen);

    }


    pub fn factor(&mut self){
        println!("-> factor()");
        match self.cur_tok.unwrap().token_type {
            TokenType::Identifier => {self.match_tok(TokenType::Identifier); ()},
            TokenType::OpenParen => {
                self.match_tok(TokenType::OpenParen); self.expr()
            },
            TokenType::Not => {
                self.match_tok(TokenType::Not); self.factor()
            },
            _ => (),
        }
    }

    pub fn term(&mut self){
        println!("-> term()");
        self.factor();

        loop {
            match self.cur_tok.unwrap().token_type {
                TokenType::Star => {self.match_tok(TokenType::Symbol(Symbol::Star)); ()},
                TokenType::Bar => {self.match_tok(TokenType::Symbol(Symbol::Bar)); ()},
                TokenType::And => {self.match_tok(TokenType::Symbol(Symbol::And)); ()},

                _ => break,
            }
            self.factor();
        }
    }
    pub fn expr_s(&mut self) {
        println!("-> expr_s()");
        match self.cur_tok.unwrap().token_type {
            TokenType::Plus => {self.match_tok(TokenType::Symbol(Symbol::Plus)); ()},
            TokenType::Minus => {self.match_tok(TokenType::Symbol(Symbol::Minus)); ()},
            _ => (),
        }

        self.term();

        loop {
            match self.cur_tok.unwrap().token_type {
                TokenType::Plus => {self.match_tok(TokenType::Symbol(Symbol::Plus)); ()},
                TokenType::Minus => {self.match_tok(TokenType::Symbol(Symbol::Minus)); ()},
                TokenType::Or => {self.match_tok(TokenType::Symbol(Symbol::Or)); ()},

                _ => break,
            }
            self.term();
        }
    }

    pub fn expr(&mut self) {
        println!("-> expr()");

        self.expr_s();

        // while self.cur_tok.unwrap().token_type
        loop {
            match self.cur_tok.unwrap().token_type {
                TokenType::Symbol(Symbol::Gr) => {self.match_tok(TokenType::Symbol(Symbol::Gr)); ()},
                TokenType::Symbol(Symbol::Ls) => {self.match_tok(TokenType::Symbol(Symbol::Ls)); ()},
                TokenType::Symbol(Symbol::Eq) => {self.match_tok(TokenType::Symbol(Symbol::Eq)); ()},
                TokenType::Symbol(Symbol::Ge) => {self.match_tok(TokenType::Symbol(Symbol::Ge)); ()},
                TokenType::Symbol(Symbol::Le) => {self.match_tok(TokenType::Symbol(Symbol::Le)); ()},
                
                _ => break,
            }
            
            self.expr_s();
        }
    }


    pub fn if_stmt(&mut self) {
        println!("-> if stmt()");
        self.match_tok(TokenType::Keyword(Keyword::If));
        self.match_tok(TokenType::Symbol(Symbol::OpenParen));
        self.expr();
        self.match_tok(TokenType::Symbol(Symbol::CloseParen));
        //body
        self.match_tok(TokenType::Symbol(Symbol::OpenBrace));
        self.stmt();
        self.match_tok(TokenType::Symbol(Symbol::CloseBrace));

        //optional list of if else
        while self.cur_tok.unwrap().token_type == TokenType::If {
            self.match_tok(TokenType::Keyword(Keyword::If));
            self.match_tok(TokenType::Keyword(Keyword::Else));
            self.match_tok(TokenType::Symbol(Symbol::OpenBrace));
            self.stmt();
            self.match_tok(TokenType::Symbol(Symbol::CloseBrace));
        }
        //optional else
        if self.cur_tok.unwrap().token_type == TokenType::Keyword(Keyword::Else) {
            self.match_tok(TokenType::Keyword(Keyword::Else));
            self.match_tok(TokenType::Symbol(Symbol::OpenBrace));
            self.stmt();
            self.match_tok(TokenType::Symbol(Symbol::CloseBrace));
        }
    

    
    }


    pub fn while_stmt(&mut self){
        println!("-> while stmt()");

    }

    pub fn do_while_stmt(&mut self) {
        println!("-> do while stmt()");

    }

    pub fn compound_stmt(&mut self) {
        println!("-> compound_stmt()");

        match self.cur_tok.unwrap().token_type {
            TokenType::Keyword(Keyword::If) => self.if_stmt(),
            TokenType::Keyword(Keyword::While) => self.while_stmt(),
            _ => (),

        }
    }
    pub fn simple_stmt(&mut self) {
        println!("-> simple_stmt()");

    }

    pub fn stmt(&mut self) {
        println!("-> stmt()");
        //let tok = self.lexer.next().unwrap();
        
        match self.cur_tok.unwrap().token_type {
            TokenType::Keyword(_) => self.compound_stmt(),
            _ =>  self.simple_stmt(),
            
        }
    }
   

}

