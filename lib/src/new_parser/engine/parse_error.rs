use crate::lexer::Token;

#[derive(Debug)]
pub enum ParseError {
    ExpectedIdent(Token),
    UnexpectedToken(String, Token), // expected, got
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

impl ParseError {
    pub fn discriminant(&self) -> &'static str {
        match self {
            ParseError::ExpectedIdent(_) => "ExpectedIdent",
            ParseError::UnexpectedToken(_, _) => "UnexpectedToken",
            ParseError::ExpectedType(_) => "ExpectedType",
            ParseError::ExpectedOperator(_) => "ExpectedOperator",
            ParseError::ExpectedBool(_) => "ExpectedBool",
            ParseError::ExpectedNumber(_) => "ExpectedNumber",
            ParseError::UnexpectedEOF => "UnexpectedEOF",
            ParseError::UnknownFile(_) => "UnknownFile",
            ParseError::Lexer(_) => "Lexer",
            ParseError::UnexpectedIndent(_) => "UnexpectedIndent",
            ParseError::ExpectedOneOrMore => "ExpectedOneOrMore",
        }
    }
}
