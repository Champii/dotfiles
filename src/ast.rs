use crate::span::Span;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<TopLevel>,
}

#[derive(Debug)]
pub enum TopLevel {
    FunctionDecl(FunctionDecl),
}

#[derive(Debug)]
pub struct FunctionDecl {
    pub name: Ident,
    pub parameters: Vec<Ident>,
    pub body: Block,
}

#[derive(Debug)]
pub struct Block {
    pub expressions: Vec<Statement>,
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
