
pub enum Keyword {
    If,
    Else,
    While,
    Do,

pub enum Symbol {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Comma,
    Semicolon,
}

pub enum Lexem {
    pub start: usize,
    pub end: usize,
}

pub enum TokenType{
    Keyword(Keyword),
    Ident : Lexem,
    Symbol(Symbol),
}

pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
}
