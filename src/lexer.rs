use std::ops::Range;
use std::fmt;


// this is
pub struct Lexer<'a> {
    pub content: &'a str,
    pub pos: usize,
    pub line: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Self {
        Lexer {
            content,
            pos: 0,
            line: 1,
        }
    }

    pub fn get_line(&self) -> u32 {
        self.line
    }

    pub fn get_pos(&self) -> usize {
        self.pos
    }

    pub fn get_char(&mut self) -> Option<char> {
        if self.content[self.pos..].chars().next() == Some('\n') {
            self.line +=1;
        }
        self.pos += 1;
        self.content[(self.pos - 1)..].chars().next()
    }
    pub fn current_char(&self) -> Option<char> {
        self.content[self.pos..].chars().next()
    }
    pub fn advance(&mut self) {
        self.pos += 1;
        if self.current_char() == Some('\n') {
            self.line += 1;
        }
    }

    pub fn get_lexem(&self, range: Range<usize>) -> &str {
        &self.content[range.start..range.end + 1]
    }
    
    pub fn peek(&self) -> Option<char> {
        self.content[self.pos..].chars().next()

    }
    pub fn token(&self, token_type: TokenType, start: usize) -> Token {
        Token {
            token_type,
            lexem: Lexem::new(start, self.pos - 1),
            line: self.line,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
    
        let mut c = self.get_char().unwrap();

        if c.is_whitespace() {
            c = self.get_char()?;

            while c.is_whitespace() {
                c = self.get_char()?;
            }
        }


        if c.is_alphabetic() {
            
            let start = self.pos - 1;

            while self.current_char()?.is_alphabetic() || self.current_char()?.is_numeric() {
                self.advance();
            }
            
            let lexem = self.get_lexem(start.. self.pos - 1);

            let token_type = match lexem {
                "if" => TokenType::If,
                "else" => TokenType::Else,
                "while" => TokenType::While,
                "do" => TokenType::Do,
                _ => TokenType::Identifier,
            };
            return Some(self.token(token_type, start));

       } else if c.is_numeric() {

           let start = self.pos - 1;

            while self.current_char()?.is_numeric() {
                self.advance();
            }

            return Some(self.token(TokenType::Number, start));

       } else if c == '=' {
           let start = self.pos-1;
            let token_type = match self.peek() {
                Some('=') => {self.advance(); TokenType::Eq},
                _   => TokenType::Assign,
            };
            return Some(self.token(token_type, start));

       } else if c == '>' {
           let start = self.pos-1;
 
           let token_type = match self.peek() {
                Some('=') => {self.advance(); TokenType::Ge},
                _ => TokenType::Gr,
           };
 
           return Some(self.token(token_type, start));
       } else if c == '<' {
           let start = self.pos-1;
 
           let token_type = match self.peek() {
                Some('=') => {self.advance(); TokenType::Le},
                _ => TokenType::Ls,
           };
 
           return Some(self.token(token_type, start));

       } else if c == '(' {
           return Some(self.token(TokenType::OpenParen, self.pos-1));
       } else if c == ')' {
           return Some(self.token(TokenType::CloseParen, self.pos-1));
       }

        None
    }    


}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    If,
    Else,
    Do,
    While,
    Number,
    
    Ge,
    Gr,
    Le,
    Ls,
    Eq,
    Assign,
    
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,

    SemiColon,
    Comma,

    Eof,
}
#[derive(Debug)]
pub struct Lexem {
    pub start: usize,
    pub end: usize,
}
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexem: Lexem,
    pub line: u32,
}
impl Lexem {

    pub fn new(start: usize, end: usize) -> Lexem {

        Lexem {
            start,
            end,
        }

    }
}

impl Token {
    pub fn new(token_type: TokenType, lexem: Lexem, line: u32) -> Token {
        Token {
            line,
            token_type,
            lexem,
        }
    }

    pub fn get_line(&self) -> u32 {
        self.line
    }  
}










