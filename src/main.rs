
use std::fs;



mod ast;
mod parser;
mod lexer;

use lexer::{Lexer};
use ast::{AST, ASTKind};
use parser::{Parser};
//use crate::parser::Parser;








fn main() {
    let content = fs::read_to_string("file.txt").unwrap();
    let mut lexer = Lexer::new(&content);
    //while let Some(t) = lexer.next() {
    //    println!("{:?}, {}", t, lexer.get_lexem(t.lexem.start..t.lexem.end));
    //};
    //println!("{:?} printing", lexer.next());
    //println!("{:?} printing", lexer.next());
    //println!("{}", lexer.get_lexem(24..26));
    //println!("{}", lexer.get_lexem(45..49));
    //println!("{}", lexer.content[0]);
    //println!("lines = {}", lexer.line);

    let val1 = ast::AST{kind:ASTKind::Int(1)};
    let val2 = ast::AST{kind:ASTKind::Int(2)};
    let binop = ast::AST{kind:ASTKind::BinOps(Box::new(val1), Box::new(val2), '+')};
    
    let mut parser = Parser::new(&mut lexer); 
    parser.parse();
    binop.print();
}
