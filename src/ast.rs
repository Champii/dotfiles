use crate::lexer::{Span, Token};

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub top_levels: Vec<TopLevel>,
}

impl Program {
    pub fn top_level_from_ident(&self, ident: &str) -> Option<&TopLevel> {
        self.top_levels.iter().find(|tl| tl.ident.name == ident)
    }

    pub fn has_macro_invoc(&self) -> bool {
        self.top_levels.iter().any(|tl| match &tl.kind {
            TopLevelKind::MacroInvoc(_) => true,
            _ => false,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TopLevel {
    pub ident: Ident,
    pub kind: TopLevelKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TopLevelKind {
    MacroDecl(MacroDecl),
    MacroInvoc(MacroInvoc),
    FunctionDecl(FunctionDecl),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MacroDecl {
    pub name: Ident,
    pub entries: Vec<MacroEntry>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MacroEntry {
    pub defs: Vec<MacroFragment>,
    pub body: Vec<MacroFragment>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MacroFragment {
    Ident(Ident),
    Token(Token),
    Repetition(Vec<MacroFragment>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MacroInvoc {
    pub name: Ident,
    pub args: Vec<Token>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionDecl {
    pub name: Ident,
    pub parameters: Vec<Ident>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Expression(Expression),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    BinopExpr(UnaryExpr, Operator, Box<Expression>),
    UnaryExpr(UnaryExpr),
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryExpr {
    PrimaryExpr(PrimaryExpr),
    UnaryExpr(Operator, Box<UnaryExpr>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrimaryExpr {
    Ident(Ident),
    Literal(Literal),
    MacroInvoc(MacroInvoc),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    Number(Number),
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Ident {}

#[derive(Debug)]
pub struct Number {
    pub value: String,
    pub span: Span,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Number {}

#[derive(Debug)]
pub struct Operator {
    pub value: String,
    pub span: Span,
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Operator {}
