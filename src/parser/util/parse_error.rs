use crate::lexer::{LexerError, Token, TokenType};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedKeyword(Token, Vec<String>),
    UnexpectedToken(Token, Vec<TokenType>),
    UnexpectedEof,
    LeftoverTokens(Vec<Token>),
    UnknownFile(String),
    Lexer(LexerError),
}
