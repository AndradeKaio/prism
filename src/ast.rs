
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

    Int(u32),
    Float(f32),
    String(String),
    Boolean(bool),
    Byte(u8),
    Var(String),

    BinOps(Box<AST>, Box<AST>, Symbol),

    While(Box<AST>, Box<AST>),
    If(Box<AST>, Box<AST>),
    IfElse(Box<AST>, Box<AST>, Box<AST>),
    FuncDef(Box<AST>, String, Box<AST>),

}



impl AST {

    pub fn print_ast(&self) {

        match self.kind {
            ASTKind::If(ref cond, ref body) => {
                print!("{}","if( ");
                cond.print_ast();
                println!("{}", ") {");
                body.print_ast();
                println!("{}", "}");
            },
            ASTKind::Int(n) => print!("Integer {}", n),
            ASTKind::Float(n) => print!("Float {}", n),
            ASTKind::Byte(n) => print!("Byte {}", n),
            ASTKind::Boolean(n) => print!("Boolean {}", n),
            ASTKind::String(ref n) => print!("String {}", n.to_string()),

            ASTKind::BinOps(ref lhs, ref rhs, c) => {
                lhs.print_ast();
                print!(" {:?} ", c);
                rhs.print_ast();
            },
            _ => (),
            
        }
    }
}

