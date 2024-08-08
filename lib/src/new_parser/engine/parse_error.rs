use crate::lexer::{Span, Token};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String, Token), // expected, got
    UnexpectedEOF,
    UnknownFile(String),
    Lexer(crate::lexer::LexerError),
    UnexpectedIndent(u8),
    ExpectedOneOrMore,
    MacroNoCorrespondance {
        macro_name: Span,
        invoc_name: Span,
        invoc_arg: Option<Span>,
    },
}

impl ParseError {
    pub fn discriminant(&self) -> &'static str {
        match self {
            ParseError::UnexpectedToken(_, _) => "UnexpectedToken",
            ParseError::UnexpectedEOF => "UnexpectedEOF",
            ParseError::UnknownFile(_) => "UnknownFile",
            ParseError::Lexer(_) => "Lexer",
            ParseError::UnexpectedIndent(_) => "UnexpectedIndent",
            ParseError::ExpectedOneOrMore => "ExpectedOneOrMore",
            ParseError::MacroNoCorrespondance { .. } => "MacroNoCorrespondance",
        }
    }
}
