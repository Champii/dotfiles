use crate::lexer::Span;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Ident(String),
    Number(String),
    Operator(String),
    Keyword(String),
    MacroInvoc(String),
    Equal,
    OpenParen,
    CloseParen,
    Arrow,
    FatArrow,
    Coma,
    Colon,
    Indent(u8),
    Eol,
    Eof,
}
