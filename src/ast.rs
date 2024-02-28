use crate::lexer::{Span, Token};

#[derive(Debug)]
pub struct Program {
    pub top_levels: Vec<TopLevel>,
}

#[derive(Debug)]
pub enum TopLevel {
    MacroDecl(MacroDecl),
    MacroInvoc(MacroInvoc),
    FunctionDecl(FunctionDecl),
}

#[derive(Debug)]
pub struct MacroDecl {
    pub name: Ident,
    pub entries: Vec<MacroEntry>,
}

#[derive(Debug)]
pub struct MacroInvoc {
    pub name: Ident,
    pub args: Vec<Token>,
}

#[derive(Debug)]
pub struct MacroEntry {
    pub defs: Vec<Token>,
    pub block: Vec<Token>,
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

#[derive(Debug)]
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
