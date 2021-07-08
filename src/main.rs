
use std::fs;



mod ast;
mod parser;
mod lexer;

use lexer::{Lexer, Symbol};
use ast::{AST, ASTKind};
use parser::{Parser};
//use crate::parser::Parser;
fn main() {
    let content = fs::read_to_string("file.txt").unwrap();
    let mut lexer = Lexer::new(&content);

    // let val1 = ast::AST{kind:ASTKind::LitInt(1)};
    // let val2 = ast::AST{kind:ASTKind::LitInt(2)};
    // let binop = ast::AST{kind:ASTKind::BinOps(Box::new(val1), Box::new(val2), Symbol::Plus)};
    
    let mut parser = Parser::new(&mut lexer); 
    let _ = parser.parse();
}
