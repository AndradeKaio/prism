
/*
    http://www.iro.umontreal.ca/~felipe/IFT2030-Automne2002/Complements/tinyc.c
*//*
 *  <program> ::= <statement>
 *  <statement> ::= "if" <paren_expr> <statement> |
 *                  "if" <paren_expr> <statement> "else" <statement> |
 *                  "while" <paren_expr> <statement> |
 *                  "do" <statement> "while" <paren_expr> ";" |
 *                  "{" { <statement> } "}" |
 *                  <expr> ";" |
 *                  ";"
                    <type> <var_decl> ";"
                    <declaration_list>

    <declaration_list> ::= <var_decl> |
                           <fn_decl>


    <var_decl> ::= <type> <id> [["="] <expr> {, id  ["="] <expr> } ] ";"
    int um

 *  <paren_expr> ::= "(" <expr> ")"
 *  <expr> ::= <test> | <id> "=" <expr>
 *  <test> ::= <sum> | <sum> "<" <sum>
 *  <sum> ::= <term> | <sum> "+" <term> | <sum> "-" <term>
 *  <term> ::= <id> | <int> | <paren_expr>
 *  <id> ::= "a" | "b" | "c" | "d" | ... | "z"
 *  <int> ::= <an_unsigned_decimal_integer>
*/
use crate::lexer::{Symbol};

pub struct AST {
    pub kind: ASTKind,
}

pub enum ASTKind {

    Int(i32),
    Var,
    BinOps(Box<AST>, Box<AST>, Symbol),
    While(Box<AST>, Box<AST>),
    If(Box<AST>, Box<AST>, Box<AST>),

}



impl AST {

    pub fn print(&self) {

        match self.kind {
            ASTKind::Int(n) => print!("{}", n),
            ASTKind::BinOps(ref lhs, ref rhs, c) => {
                lhs.print();
                print!(" {:?} ", c);
                rhs.print();
            },
            _ => (),
            
        }
    }
}

