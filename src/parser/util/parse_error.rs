use crate::{
    ast::Ident,
    lexer::{LexerError, Token, TokenType},
};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedKeyword(Token, Vec<String>),
    UnexpectedToken(Token, Vec<TokenType>),
    UnexpectedEof(TokenType),
    LeftoverTokens(Vec<Token>),
    UnknownFile(String),
    Lexer(LexerError),
    MacroNoCorrespondance(Ident),
}
