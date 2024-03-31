use std::fmt::{Display, Formatter};

use crate::lexer::Span;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Ident(String),
    Type(String),
    Number(String),
    Float(String),
    Operator(String),
    Comment(String),
    //Operator that is not followed by a space
    StuckOperator(String),
    NativeOperator(String),
    Keyword(String),
    MacroVar(String),
    MacroInvoc(String),
    MacroRepeatOpen,
    MacroRepeatClose,
    Equal,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    SimpleQuote,
    DoubleQuote,
    Arrow,
    FatArrow,
    Coma,
    Colon,
    DoubleColon,
    Dot,
    SpacedDot,
    Arobase,
    Indent(u8),
    Eol,
    Eof,
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::Ident(s) => s.clone(),
            TokenType::Type(s) => s.clone(),
            TokenType::Number(s) => s.clone(),
            TokenType::Float(s) => s.clone(),
            TokenType::Operator(s) => s.clone(),
            TokenType::Comment(s) => s.clone(),
            TokenType::StuckOperator(s) => s.clone(),
            TokenType::NativeOperator(s) => s.clone(),
            TokenType::Keyword(s) => s.clone(),
            TokenType::MacroVar(s) => s.clone(),
            TokenType::MacroInvoc(s) => s.clone(),
            TokenType::MacroRepeatOpen => "$(".to_string(),
            TokenType::MacroRepeatClose => ")".to_string(),
            TokenType::Equal => "=".to_string(),
            TokenType::OpenParen => "(".to_string(),
            TokenType::CloseParen => ")".to_string(),
            TokenType::OpenBracket => "[".to_string(),
            TokenType::CloseBracket => "]".to_string(),
            TokenType::SimpleQuote => "'".to_string(),
            TokenType::DoubleQuote => "\"".to_string(),
            TokenType::Arrow => "->".to_string(),
            TokenType::FatArrow => "=>".to_string(),
            TokenType::Coma => ",".to_string(),
            TokenType::Colon => ":".to_string(),
            TokenType::DoubleColon => "::".to_string(),
            TokenType::Dot => ".".to_string(),
            TokenType::SpacedDot => " .".to_string(),
            TokenType::Arobase => "@".to_string(),
            TokenType::Indent(i) => " ".repeat(*i as usize),
            TokenType::Eol => "\n".to_string(),
            TokenType::Eof => "".to_string(),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}

impl Eq for Token {}

impl From<TokenType> for Token {
    fn from(token_type: TokenType) -> Self {
        Self {
            token_type,
            span: Span::default(),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_type.to_string())
    }
}
