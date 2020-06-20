
//mod lexer;

use crate::lexer::{Lexer, Token, TokenType};

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
}

impl <'a> Parser <'a>{
    
    pub fn new(lexer: &'a mut Lexer <'a>) -> Self {
        Parser {
            lexer,
        }

    }

    pub fn match_tok(&mut self, token_type: TokenType) -> bool {
        
        let token = self.lexer.next().unwrap();

        assert_eq!(token.token_type, token_type, "Expected token '{:?}' find '{:?}' at line {}.", 
        token_type, token.token_type, token.line);
        true
    }
    pub fn parse(&mut self){
        println!("Parser starts...");
        self.statement(); 
    }

    pub fn paren_expr(&mut self) {
       println!("-> paren_expr()");
       self.match_tok(TokenType::OpenParen);
       self.expr();      
       self.match_tok(TokenType::CloseParen);

    }


    pub fn expr(&mut self) {
        println!("-> expr()");
    }


    pub fn if_statement (&mut self) {
        println!("-> if statement()");
        self.paren_expr();
        let t = self.lexer.next().unwrap();
        if match_tok!(t.token_type, TokenType::Else) {
            self.statement();
        }

    
    }


    pub fn while_statement(&mut self){
        println!("-> while statement()");

    }

    pub fn do_while_statement(&mut self) {
        println!("-> do while statement()");

    }

    pub fn statement(&mut self) {
        println!("-> statement()");
        let tok = self.lexer.next().unwrap();
        
        match tok.token_type {

            TokenType::If => self.if_statement(),

            TokenType::While => self.while_statement(),

            TokenType::Do => self.do_while_statement(),

            TokenType::OpenBrace => { 
                self.statement();
                let t = self.lexer.next().unwrap();
                match_tok!(t.token_type, TokenType::CloseBrace);
            }
            TokenType::SemiColon => (),

            _ => {
                self.expr(); 
                let t = self.lexer.next().unwrap();
                match_tok!(t.token_type, TokenType::SemiColon);
                ()
            },
            
        }
    }
   

}

