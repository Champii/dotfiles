use crate::lexer::{LexerError, Token};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEof,
    LeftoverTokens(Vec<Token>),
    UnknownFile(String),
    Lexer(LexerError),
}
