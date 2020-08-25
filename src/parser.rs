
use crate::lexer::{Lexer, Token, TokenType, Keyword, Symbol};
use crate::ast::{AST, ASTKind};

macro_rules! match_tok {
    ($e:expr, $p:pat) => {
        match $e {
            $p => true,
            _ => false
        }
    }
}

pub enum Error {
    error,
    EOF,
}

pub type Node<T> = Result<T, Error>;

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
       self.match_tok(TokenType::Symbol(Symbol::OpenParen));
       self.expr();      
       self.match_tok(TokenType::Symbol(Symbol::CloseParen));

    }

    pub fn factor_(&mut self) -> Node<AST>{
        println!("-> factor()");

        match self.cur_tok.unwrap().token_type {

            TokenType::Identifier => {
                self.match_tok(TokenType::Identifier);
                return Ok(AST{kind:ASTKind::Var});
            },
            TokenType::Symbol(Symbol::OpenParen) => {
                self.match_tok(TokenType::Symbol(Symbol::OpenParen)); 
                return self.expr_();
                self.match_tok(TokenType::Symbol(Symbol::CloseParen)); 
            },
            TokenType::Symbol(Symbol::Not) => {
                self.match_tok(TokenType::Symbol(Symbol::Not)); 
                return self.factor_();
            },
            _ => Err(Error::error),
        }
    }
    pub fn term_(&mut self) -> Node<AST>{
        println!("-> term()");
        let mut lhs = self.factor_()?;

        loop {
            let symbol = match self.cur_tok.unwrap().token_type {
                TokenType::Symbol(Symbol::Star) => {self.match_tok(TokenType::Symbol(Symbol::Star)); Symbol::Star},
                TokenType::Symbol(Symbol::Bar) => {self.match_tok(TokenType::Symbol(Symbol::Bar)); Symbol::Bar},
                TokenType::Symbol(Symbol::And) => {self.match_tok(TokenType::Symbol(Symbol::And)); Symbol::And},

                _ => break,
            };
            lhs = AST{kind:ASTKind::BinOps(Box::new(lhs), Box::new(self.factor_()?), symbol)}
        }
        return Ok(lhs);
    }

    pub fn factor(&mut self){
        println!("-> factor()");
        match self.cur_tok.unwrap().token_type {
            TokenType::Identifier => {self.match_tok(TokenType::Identifier); ()},
            TokenType::Symbol(Symbol::OpenParen) => {
                self.match_tok(TokenType::Symbol(Symbol::OpenParen)); self.expr()
            },
            TokenType::Symbol(Symbol::Not) => {
                self.match_tok(TokenType::Symbol(Symbol::Not)); self.factor()
            },
            _ => (),
        }
    }

    pub fn term(&mut self){
        println!("-> term()");
        self.factor();

        loop {
            match self.cur_tok.unwrap().token_type {
                TokenType::Symbol(Symbol::Star) => {self.match_tok(TokenType::Symbol(Symbol::Star)); ()},
                TokenType::Symbol(Symbol::Bar) => {self.match_tok(TokenType::Symbol(Symbol::Bar)); ()},
                TokenType::Symbol(Symbol::And) => {self.match_tok(TokenType::Symbol(Symbol::And)); ()},

                _ => break,
            }
            self.factor();
        }
    }
    pub fn expr_s(&mut self) {
        println!("-> expr_s()");
        match self.cur_tok.unwrap().token_type {
            TokenType::Symbol(Symbol::Plus) => {self.match_tok(TokenType::Symbol(Symbol::Plus)); ()},
            TokenType::Symbol(Symbol::Minus) => {self.match_tok(TokenType::Symbol(Symbol::Minus)); ()},
            _ => (),
        }

        self.term();

        loop {
            match self.cur_tok.unwrap().token_type {
                TokenType::Symbol(Symbol::Plus) => {self.match_tok(TokenType::Symbol(Symbol::Plus)); ()},
                TokenType::Symbol(Symbol::Minus) => {self.match_tok(TokenType::Symbol(Symbol::Minus)); ()},
                TokenType::Symbol(Symbol::Or) => {self.match_tok(TokenType::Symbol(Symbol::Or)); ()},

                _ => break,
            }
            self.term();
        }
    }
    pub fn expr_s_(&mut self) -> Node<AST>{
        println!("-> expr_s()");
        match self.cur_tok.unwrap().token_type {
            TokenType::Symbol(Symbol::Plus) => {self.match_tok(TokenType::Symbol(Symbol::Plus)); ()},
            TokenType::Symbol(Symbol::Minus) => {self.match_tok(TokenType::Symbol(Symbol::Minus)); ()},
            _ => (),
        }

        let mut lhs = self.term_()?;

        loop {
            let symbol = match self.cur_tok.unwrap().token_type {
                TokenType::Symbol(Symbol::Plus) => {self.match_tok(TokenType::Symbol(Symbol::Plus)); Symbol::Plus},
                TokenType::Symbol(Symbol::Minus) => {self.match_tok(TokenType::Symbol(Symbol::Minus)); Symbol::Minus},
                TokenType::Symbol(Symbol::Or) => {self.match_tok(TokenType::Symbol(Symbol::Or)); Symbol::Or},

                _ => break,
            };
            lhs = AST{kind:ASTKind::BinOps(Box::new(lhs), Box::new(self.term_()?), symbol)};
        }
        return Ok(lhs);
    }
    pub fn expr_(&mut self) -> Node<AST> {
        println!("-> expr()");

        let mut lhs = self.expr_s_()?;

        loop {
            let symbol = match self.cur_tok.unwrap().token_type {
                TokenType::Symbol(Symbol::Gr) => {self.match_tok(TokenType::Symbol(Symbol::Gr)); Symbol::Gr},
                TokenType::Symbol(Symbol::Ls) => {self.match_tok(TokenType::Symbol(Symbol::Ls)); Symbol::Ls},
                TokenType::Symbol(Symbol::Eq) => {self.match_tok(TokenType::Symbol(Symbol::Eq)); Symbol::Eq},
                TokenType::Symbol(Symbol::Ge) => {self.match_tok(TokenType::Symbol(Symbol::Ge)); Symbol::Ge},
                TokenType::Symbol(Symbol::Le) => {self.match_tok(TokenType::Symbol(Symbol::Le)); Symbol::Le},
                
                _ => break,
            };
            lhs = AST{kind:ASTKind::BinOps(Box::new(lhs), Box::new(self.expr_s_()?), symbol)};
        }
        return Ok(lhs);
    }

    pub fn expr(&mut self) {
        println!("-> expr()");

        self.expr_s();

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
        while self.cur_tok.unwrap().token_type == TokenType::Keyword(Keyword::If) {
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
    // pub fn if_stmt_(&mut self) -> Node<AST>{
    //     println!("-> if stmt()");
    //     self.match_tok(TokenType::Keyword(Keyword::If));
    //     self.match_tok(TokenType::Symbol(Symbol::OpenParen));
    //     let cond = self.expr_();
    //     self.match_tok(TokenType::Symbol(Symbol::CloseParen));
    //     //body
    //     self.match_tok(TokenType::Symbol(Symbol::OpenBrace));
    //     let body = self.stmt();
    //     self.match_tok(TokenType::Symbol(Symbol::CloseBrace));


    //     if self.cur_tok.unwrap().token_type == TokenType::Keyword(Keyword::Else) {
    //         self.match_tok(TokenType::Symbol(Symbol::OpenBrace));
    //         let else_stmt = self.stmt_();
    //         self.match_tok(TokenType::Symbol(Symbol::CloseBrace));
    //     }else {
    //         let else_stmt = None;
    //     }

    //     return Node(AST{kind:ASTKind::If(Box::new(cond), Box::new(body), else_stmt)});
        //optional list of if else
        // loop {
        //     if self.cur_tok.unwrap().token_type == TokenType::Keyword(Keyword::Else) {
        //         self.match_tok(TokenType::Keyword(Keyword::Else));
        //         if self.cur_tok.unwrap().token_type == TokenType::Keyword(Keyword::If) {
        //             self.match_tok(TokenType::Keyword(Keyword::If));
        //             self.match_tok(TokenType::Symbol(Symbol::OpenParen));
        //             self.expr_();
        //             self.match_tok(TokenType::Symbol(Symbol::CloseParen));
        //             self.match_tok(TokenType::Symbol(Symbol::OpenBrace));
        //             self.stmt();
        //             self.match_tok(TokenType::Symbol(Symbol::CloseBrace));
        //         //only else's body
        //         }else {
        //             self.match_tok(TokenType::Symbol(Symbol::OpenBrace));
        //             self.stmt();
        //             self.match_tok(TokenType::Symbol(Symbol::CloseBrace));
        //             break;
        //         }
        //     }else {
        //         break;
        //     }
    
        // }
    // }





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

