use crate::lexer::{Span, Token};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct TopLevel {
    pub ident: Ident,
    pub kind: TopLevelKind,
}

#[derive(Debug)]
pub enum TopLevelKind {
    MacroDecl(MacroDecl),
    MacroInvoc(MacroInvoc),
    FunctionDecl(FunctionDecl),
}

#[derive(Debug, Clone)]
pub struct MacroDecl {
    pub name: Ident,
    pub entries: Vec<MacroEntry>,
}

#[derive(Debug, Clone)]
pub struct MacroEntry {
    pub defs: Vec<MacroFragment>,
    pub body: Vec<MacroFragment>,
}

#[derive(Debug, Clone)]
pub enum MacroFragment {
    Ident(Ident),
    Token(Token),
    Repetition(Vec<MacroFragment>),
}

#[derive(Debug)]
pub struct MacroInvoc {
    pub name: Ident,
    pub args: Vec<Token>,
}

#[derive(Debug)]
pub struct FunctionDecl {
    pub name: Ident,
    pub parameters: Vec<Ident>,
    pub body: Block,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
}

#[derive(Debug)]
pub enum Expression {
    BinopExpr(UnaryExpr, Operator, Box<Expression>),
    UnaryExpr(UnaryExpr),
}

#[derive(Debug)]
pub enum UnaryExpr {
    PrimaryExpr(PrimaryExpr),
    UnaryExpr(Operator, Box<UnaryExpr>),
}

#[derive(Debug)]
pub enum PrimaryExpr {
    Ident(Ident),
    Literal(Literal),
    MacroInvoc(MacroInvoc),
}

#[derive(Debug)]
pub enum Literal {
    Number(Number),
}

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

#[derive(Debug)]
pub struct Number {
    pub value: String,
    pub span: Span,
}

#[derive(Debug)]
pub struct Operator {
    pub value: String,
    pub span: Span,
}
