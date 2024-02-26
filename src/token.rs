use crate::span::Span;

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
    Equal,
    OpenParen,
    CloseParen,
    Arrow,
    Coma,
    Indent(u8),
    Eol,
    Eof,
}
