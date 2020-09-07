
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
        
        let token = self.cur_tok.clone().unwrap();
        self.cur_tok = Some(self.lexer.next().unwrap());

        assert_eq!(token.token_type, token_type, "Expected token '{:?}' find '{:?}' {:?} at line {}.", 
        token_type, token.token_type, self.lexer.get_lexem(token.lexem.start..token.lexem.end), token.line);
        true
    }
    
    pub fn cur_tok(&self) -> Option<Token> {
        self.cur_tok.clone()
    }

    pub fn cur_type(&self) -> TokenType {
        self.cur_tok.clone().unwrap().token_type
    }

    pub fn get_lexem(&self, tok: Token) -> String {
        self.lexer.get_tok_lexem(tok).to_string()
    }

    pub fn parse(&mut self) -> Node<AST>{
        self.cur_tok = Some(self.lexer.next().unwrap());
            let start = self.stmt()?; 
            start.print_ast();
        //}
        Ok(start)
    }

    pub fn factor(&mut self) -> Node<AST>{
        println!("-> factor()");

        match self.cur_type() {

            TokenType::Identifier(i) => {
                self.match_tok(TokenType::Identifier(i.clone()));
                return Ok(AST{kind:ASTKind::Var(i)});
            },
            TokenType::LitInt(u) => {
                self.match_tok(TokenType::LitInt(u));
                Ok(AST{kind:ASTKind::LitInt(u)})
            },
            TokenType::LitFloat(f) => {
                self.match_tok(TokenType::LitFloat(f));
                Ok(AST{kind:ASTKind::LitFloat(f)})
            },
            TokenType::LitByte(b) => {
                self.match_tok(TokenType::LitByte(b));
                Ok(AST{kind:ASTKind::LitByte(b)})
            },
            TokenType::LitString(s) => {
                self.match_tok(TokenType::LitString(s.clone()));
                Ok(AST{kind:ASTKind::LitString(s)})
            },
            TokenType::LitBool(b) => {
                self.match_tok(TokenType::LitBool(b));
                Ok(AST{kind:ASTKind::LitBoolean(b)})
            },
            TokenType::Symbol(Symbol::OpenParen) => {
                self.match_tok(TokenType::Symbol(Symbol::OpenParen)); 
                return self.expr();
                self.match_tok(TokenType::Symbol(Symbol::CloseParen)); 
            },
            TokenType::Symbol(Symbol::Not) => {
                self.match_tok(TokenType::Symbol(Symbol::Not)); 
                return self.factor();
            },
            _ => Err(Error::error),
        }
    }
    pub fn term(&mut self) -> Node<AST>{
        println!("-> term()");
        let mut lhs = self.factor()?;

        loop {
            let symbol = match self.cur_type() {
                TokenType::Symbol(Symbol::Star) => {self.match_tok(TokenType::Symbol(Symbol::Star)); Symbol::Star},
                TokenType::Symbol(Symbol::Bar) => {self.match_tok(TokenType::Symbol(Symbol::Bar)); Symbol::Bar},
                TokenType::Symbol(Symbol::And) => {self.match_tok(TokenType::Symbol(Symbol::And)); Symbol::And},

                _ => break,
            };
            lhs = AST{kind:ASTKind::BinOps(Box::new(lhs), Box::new(self.factor()?), symbol)}
        }
        return Ok(lhs);
    }


    pub fn exprs(&mut self) -> Node<AST>{
        println!("-> exprs()");
        match self.cur_type() {
            TokenType::Symbol(Symbol::Plus) => {self.match_tok(TokenType::Symbol(Symbol::Plus)); ()},
            TokenType::Symbol(Symbol::Minus) => {self.match_tok(TokenType::Symbol(Symbol::Minus)); ()},
            _ => (),
        }

        let mut lhs = self.term()?;

        loop {
            let symbol = match self.cur_type() {
                TokenType::Symbol(Symbol::Plus) => {self.match_tok(TokenType::Symbol(Symbol::Plus)); Symbol::Plus},
                TokenType::Symbol(Symbol::Minus) => {self.match_tok(TokenType::Symbol(Symbol::Minus)); Symbol::Minus},
                TokenType::Symbol(Symbol::Or) => {self.match_tok(TokenType::Symbol(Symbol::Or)); Symbol::Or},

                _ => break,
            };
            lhs = AST{kind:ASTKind::BinOps(Box::new(lhs), Box::new(self.term()?), symbol)};
        }
        return Ok(lhs);
    }
    pub fn expr(&mut self) -> Node<AST> {
        println!("-> expr()");

        let mut lhs = self.exprs()?;

        loop {
            let symbol = match self.cur_type() {
                TokenType::Symbol(Symbol::Gr) => {self.match_tok(TokenType::Symbol(Symbol::Gr)); Symbol::Gr},
                TokenType::Symbol(Symbol::Ls) => {self.match_tok(TokenType::Symbol(Symbol::Ls)); Symbol::Ls},
                TokenType::Symbol(Symbol::Eq) => {self.match_tok(TokenType::Symbol(Symbol::Eq)); Symbol::Eq},
                TokenType::Symbol(Symbol::Ge) => {self.match_tok(TokenType::Symbol(Symbol::Ge)); Symbol::Ge},
                TokenType::Symbol(Symbol::Le) => {self.match_tok(TokenType::Symbol(Symbol::Le)); Symbol::Le},
                
                _ => break,
            };
            lhs = AST{kind:ASTKind::BinOps(Box::new(lhs), Box::new(self.exprs()?), symbol)};
        }
        return Ok(lhs);
    }



    pub fn if_stmt(&mut self) -> Node<AST>{
        println!("-> if stmt()");
        self.match_tok(TokenType::Keyword(Keyword::If));
        self.match_tok(TokenType::Symbol(Symbol::OpenParen));
        let cond = self.expr();
        self.match_tok(TokenType::Symbol(Symbol::CloseParen));
        //body
        self.match_tok(TokenType::Symbol(Symbol::OpenBrace));
        let body = self.stmt();
        self.match_tok(TokenType::Symbol(Symbol::CloseBrace));

        if self.cur_type() == TokenType::Keyword(Keyword::Else) {
            self.match_tok(TokenType::Symbol(Symbol::OpenBrace));
            let else_stmt = self.stmt()?;
            self.match_tok(TokenType::Symbol(Symbol::CloseBrace));
            return Ok(AST{kind:ASTKind::IfElse(Box::new(cond?), Box::new(body?), Box::new(else_stmt))});
        }
        return Ok(AST{kind:ASTKind::If(Box::new(cond?), Box::new(body?))});
    }


    pub fn while_stmt(&mut self) -> Node<AST>{
        println!("-> while stmt()");
        Ok(
            AST{kind:ASTKind::Var("teste".to_string())}
        )
    }

    pub fn func_def(&mut self) -> Node<AST> {
        self.match_tok(TokenType::Keyword(Keyword::Fn));

        let func_name = self.cur_tok.clone().unwrap();

        self.match_tok(TokenType::Identifier("".to_string()));//panicate if is not a id

        //type spec
        if self.cur_type() == TokenType::Symbol(Symbol::Colon) {
            self.match_tok(TokenType::Symbol(Symbol::Colon));
            //match type self.match_tok(TokenType::Symbol(Symbol::Colon));
        }



        self.match_tok(TokenType::Symbol(Symbol::OpenParen));
        //paren_list
        self.match_tok(TokenType::Symbol(Symbol::CloseParen));
        self.match_tok(TokenType::Symbol(Symbol::OpenBrace));
        let body = self.stmt()?;
        self.match_tok(TokenType::Symbol(Symbol::CloseBrace));

        Ok(
            AST{kind:ASTKind::FuncDef(Box::new(AST{kind:ASTKind::LitInt(1)}), 
            self.get_lexem(func_name), 
            Box::new(body))}
        )
    }

    pub fn compound_stmt(&mut self) -> Node<AST>{
        println!("-> compound_stmt()");

        match self.cur_type() {
            TokenType::Keyword(Keyword::If) => self.if_stmt(),
            TokenType::Keyword(Keyword::While) => self.while_stmt(),
            TokenType::Keyword(Keyword::Fn) => self.func_def(),
            _ => Err(Error::error),

        }
    }
    pub fn simple_stmt(&mut self) -> Node<AST>{
        println!("-> simple_stmt()");
        Ok(AST{kind:ASTKind::Var("teste".to_string())})
    }

    pub fn stmt(&mut self) -> Node<AST>{
        println!("-> stmt()");
        match self.cur_type() {
            TokenType::Keyword(_) => self.compound_stmt(),
            _ =>  self.simple_stmt(),
            
        }
    }

}

