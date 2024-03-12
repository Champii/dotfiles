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
    MacroVar(String),
    MacroInvoc(String),
    MacroRepeatOpen,
    MacroRepeatClose,
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

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}

impl Eq for Token {}
