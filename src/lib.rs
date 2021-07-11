//pub mod lexer;
//pub mod parser;

mod lexer;
use lexer::*;

#[macro_export]
macro_rules! match_tok{
    ($r:expr, $l: expr) => {
        match $r {
            $l => true,
            _ => false,
        }
    };

}

#[test]
fn test_lexer() {
    let code = "int a = b; int b; while(true){int c = 10;}";
    let mut lexer = Lexer::new(code);
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Keyword(Keyword::Int));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier("a".to_string()));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Symbol(Symbol::Assign));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier("b".to_string()));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Symbol(Symbol::Semicolon));

    assert_eq!(lexer.next().unwrap().token_type, TokenType::Keyword(Keyword::Int));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier("b".to_string()));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Symbol(Symbol::Semicolon));

    assert_eq!(lexer.next().unwrap().token_type, TokenType::Keyword(Keyword::While));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Symbol(Symbol::OpenParen));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::LitBool(true));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Symbol(Symbol::CloseParen));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Symbol(Symbol::OpenBrace));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Keyword(Keyword::Int));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Identifier("c".to_string()));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Symbol(Symbol::Assign));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::LitInt(10));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Symbol(Symbol::Semicolon));
    assert_eq!(lexer.next().unwrap().token_type, TokenType::Symbol(Symbol::CloseBrace));


}
