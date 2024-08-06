use crate::lexer::Token;

#[derive(Debug)]
pub enum ParseError {
    ExpectedIdent(Token),
    UnexpectedToken(Token),
    ExpectedType(Token),
    ExpectedOperator(Token),
    ExpectedBool(Token),
    ExpectedNumber(Token),
    UnexpectedEOF,
    UnknownFile(String),
    Lexer(crate::lexer::LexerError),
    UnexpectedIndent(u8),
    ExpectedOneOrMore,
}
