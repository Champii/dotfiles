use crate::lexer::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEof,
    LeftoverTokens(Vec<Token>),
}
