use std::collections::BTreeMap;

use crate::lexer::{Span, Token};

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct TopLevel {
    pub ident: Ident,
    pub kind: TopLevelKind,
}

#[derive(Debug, PartialEq)]
pub enum TopLevelKind {
    MacroDecl(MacroDecl),
    MacroInvoc(MacroInvoc),
    FunctionDecl(FunctionDecl),
    StructDecl(StructDecl),
}

#[derive(Debug, PartialEq)]
pub struct StructDecl {
    pub name: ParseType,
    pub fields: BTreeMap<Ident, ParseType>,
    pub methods: BTreeMap<Ident, FunctionDecl>,
}

#[derive(Debug, PartialEq)]
pub struct ParseType {
    pub name: String,
    pub generics: Vec<ParseType>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MacroDecl {
    pub name: Ident,
    pub entries: Vec<MacroEntry>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MacroEntry {
    pub defs: Vec<MacroFragment>,
    pub body: Vec<MacroFragment>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MacroFragment {
    Ident(Ident),
    Token(Token),
    Repetition(Vec<MacroFragment>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MacroInvoc {
    pub name: Ident,
    pub args: Vec<Token>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDecl {
    pub name: Ident,
    pub lambda: LambdaDecl,
    pub inject_self: bool,
}

#[derive(Debug, PartialEq)]
pub struct LambdaDecl {
    pub parameters: Vec<Ident>,
    pub body: Block,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinopExpr(UnaryExpr, Operator, Box<Expression>),
    UnaryExpr(UnaryExpr),
}

#[derive(Debug, PartialEq)]
pub enum UnaryExpr {
    PrimaryExpr(PrimaryExpr),
    UnaryExpr(Operator, Box<UnaryExpr>),
}

#[derive(Debug, PartialEq)]
pub struct PrimaryExpr {
    pub operand: Operand,
    pub secondaries: Option<Vec<SecondaryExpr>>,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Literal(Literal),
    Ident(IdentifierPath),
    /// Ident prefixed with a @ are desugared to self.ident
    SelfIdent(Ident),
    StructInstance(StructInstance),
    LambdaDecl(LambdaDecl),
    Expression(Box<Expression>), // parenthesis
}

#[derive(Debug, PartialEq)]
pub struct StructInstance {
    pub name: ParseType,
    pub fields: BTreeMap<Ident, Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierPath {
    pub path: Vec<Ident>,
}

#[derive(Debug, PartialEq)]
pub struct Literal {
    pub kind: LiteralKind,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Bool(bool),
    Number(u64),
    Float(f64),
    Array(Array),
    String(String),
    Char(char),
}

#[derive(Debug, PartialEq)]
pub struct Array {
    pub elements: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum SecondaryExpr {
    Arguments(Vec<Argument>),
    Indice(Box<Expression>), // Boxing here to keep the enum size low
    Dot(Ident),
}

#[derive(Debug, PartialEq)]
pub struct Argument {
    pub arg: Expression,
}

#[derive(Debug, Clone, Default)]
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

impl PartialOrd for Ident {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for Ident {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

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
