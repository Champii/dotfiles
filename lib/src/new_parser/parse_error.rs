use crate::lexer::Token;

#[derive(Debug)]
pub enum ParseError {
    ExpectedIdent(Token),
    UnexpectedToken(Token),
    UnexpectedEOF,
}
