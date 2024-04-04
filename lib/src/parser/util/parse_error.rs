use crate::lexer::{LexerError, Span, Token, TokenType};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedKeyword(Token, Vec<String>),
    UnexpectedToken(Token, Vec<TokenType>),
    UnexpectedEof(TokenType),
    LeftoverTokens(Vec<Token>),
    UnknownFile(String),
    Lexer(LexerError),
    MacroNoCorrespondance {
        macro_name: Span,
        invoc_name: Span,
        invoc_arg: Option<Span>,
    }, // macro name span,  macro entry argument span, macro invoc name span
    IndentMismatch(u8, u8),
    InvalidPrecedence(u8, Token),
    InternalError(String),
}
