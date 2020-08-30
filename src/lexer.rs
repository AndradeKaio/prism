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
    pub fn get_tok_lexem(&self, tok: Token) -> &str {
        &self.content[tok.lexem.start..tok.lexem.end + 1]
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
    // reads a identifier
    pub fn read_identifier(&mut self) -> Option<Token> {

        let start = self.pos - 1;

        while self.current_char()?.is_alphabetic() || 
                self.current_char()?.is_numeric() || 
                self.current_char().unwrap() == '_' {
            self.advance();
        }
        
        let lexem = self.get_lexem(start.. self.pos - 1);

        let token_type = match lexem {
            "if" => TokenType::Keyword(Keyword::If),
            "else" => TokenType::Keyword(Keyword::Else),
            "while" => TokenType::Keyword(Keyword::While),
            "fn" => TokenType::Keyword(Keyword::Fn),
            "byte" => TokenType::Keyword(Keyword::Byte),
            "string" => TokenType::Keyword(Keyword::String),
            "float" => TokenType::Keyword(Keyword::Float),
            "int" => TokenType::Keyword(Keyword::Int),
            "boolean" => TokenType::Keyword(Keyword::Boolean),
            _ => TokenType::Identifier(lexem.to_string()),
        };

        Some(self.token(token_type, start))
    }

    pub fn read_number_literal(&mut self) -> Option<Token> {
        let start = self.pos - 1;

        while self.current_char()?.is_numeric() {
            self.advance();
        }
        let lexem = self.get_lexem(start.. self.pos - 1);        
        Some(self.token(TokenType::IntNumber(lexem.parse::<u32>().unwrap()), start))
    }

    pub fn read_symbol(&mut self, c: char) -> Option<Token> {

        let token_type = match c {

            '(' => TokenType::Symbol(Symbol::OpenParen),
            ')' => TokenType::Symbol(Symbol::CloseParen),
            '{' => TokenType::Symbol(Symbol::OpenBrace),
            '}' => TokenType::Symbol(Symbol::CloseBrace),
            '|' => TokenType::Symbol(Symbol::Or),
            '&' => TokenType::Symbol(Symbol::And),
            ',' => TokenType::Symbol(Symbol::Comma),
            '.' => TokenType::Symbol(Symbol::Dot),
            ';' => TokenType::Symbol(Symbol::Semicolon),
            ':' => TokenType::Symbol(Symbol::Colon),
            '+' => TokenType::Symbol(Symbol::Plus),
            '-' => TokenType::Symbol(Symbol::Minus),
            '*' => TokenType::Symbol(Symbol::Star),
            '/' => TokenType::Symbol(Symbol::Bar),
            '>' => {
                if self.peek() == Some('=') {
                    TokenType::Symbol(Symbol::Ge)
                } else {
                    TokenType::Symbol(Symbol::Gr)
                }
            },
            '<' => {
                if self.peek() == Some('=') {
                    TokenType::Symbol(Symbol::Le)
                } else {
                    TokenType::Symbol(Symbol::Ls)
                }
            },
            '=' => {
                if self.peek() == Some('=') {
                    TokenType::Symbol(Symbol::Eq)
                } else {
                    TokenType::Symbol(Symbol::Assign)
                }
            },
            '!' => {
                if self.peek() == Some('=') {
                    TokenType::Symbol(Symbol::Ne)
                } else {
                    TokenType::Symbol(Symbol::Eq)
                }
            },
            '\'' => {
                let start = self.pos;
                while self.current_char()? != '\'' {
                    self.advance();
                }
                let lexem = self.get_lexem(start.. self.pos - 1);
                TokenType::StringValue(lexem.to_string())
            }
            _ => return None,
        };

        Some(self.token(token_type, self.pos - 1))
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

        let token = match c {
            'a'...'z' | 'A'...'Z' | '_' => self.read_identifier(),
            '0'...'9' => self.read_number_literal(),
            _ => self.read_symbol(c),
        };
        println!("{:?}", token);
        token
    } 
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    If,
    Else,
    While,
    Fn,
    
    Int,
    Float,
    Byte,
    Boolean,
    String,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Symbol {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Comma,
    Semicolon,
    Colon,
    Dot,

    Ge,
    Gr,
    Le,
    Ls,
    Eq,
    Ne,
    Assign,

    Plus,
    Minus,
    Star,
    Bar,


    Not,
    Or,
    And,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier(String),
    Symbol(Symbol),
    Keyword(Keyword),

    IntNumber(u32),
    FloatNumber(f32),
    ByteNumber(u8),

    BoolValue(bool),
    StringValue(String),

    Eof,
}
#[derive(Debug, Clone, Copy)]
pub struct Lexem {
    pub start: usize,
    pub end: usize,
}
#[derive(Debug, Clone)]
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










