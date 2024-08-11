use std::{collections::BTreeMap, path::PathBuf};

use crate::lexer::{Span, Token};

#[derive(Debug, PartialEq)]
pub struct Program {
    pub module: Module,
}

#[derive(Debug, PartialEq)]
pub struct ModuleDecl(pub Module);

#[derive(Debug, PartialEq)]
pub struct Module {
    pub name: Option<Ident>,
    pub top_levels: Vec<TopLevel>,
    pub is_inline: bool,
    pub filepath: Option<PathBuf>,
    pub comment: Option<String>,
}

impl Module {
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

// Used to parse the first module without a name
pub struct ModuleInner {
    pub top_levels: Vec<TopLevel>,
}

#[derive(Debug, PartialEq)]
pub struct TopLevel {
    pub ident: Ident,
    pub kind: TopLevelKind,
}

#[derive(Debug, PartialEq)]
pub enum TopLevelKind {
    Module(ModuleDecl),
    Import(IdentifierPath),
    Export(IdentifierPath),
    Extern(FunctionSig),
    InfixOperator(u8, String),
    MacroDecl(MacroDecl),
    MacroInvoc(MacroInvoc),
    FunctionSig(FunctionSig),
    FunctionDecl(FunctionDecl),
    StructDecl(StructDecl),
    TraitDecl(TraitDecl),
    EnumDecl(EnumDecl),
    Impl(Impl),
    Comment(String),
    NewType(ParseTypeInner, ParseType),
}

#[derive(Debug, PartialEq)]
pub struct StructDecl {
    pub name: ParseTypeInner,
    pub fields: Vec<StructDeclField>,
}

#[derive(Debug, PartialEq)]
pub struct StructDeclField {
    pub name: Ident,
    pub ty: ParseType,
    pub public: bool,
    pub default: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct TraitDecl {
    pub name: ParseTypeInner,
    pub methods: BTreeMap<Ident, FunctionDecl>,
    pub signatures: BTreeMap<(Ident, bool), ParseType>, // bool is for inject_self
}

#[derive(Debug, PartialEq)]
pub struct EnumDecl {
    pub name: ParseTypeInner,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, PartialEq)]
pub struct EnumVariant {
    pub name: ParseTypeInner,
    pub fields: NamedFieldsOrTypesList,
}

#[derive(Debug, PartialEq)]
pub enum NamedFieldsOrTypesList {
    NamedFields(Vec<StructDeclField>),
    TypesList(Vec<ParseType>),
}

#[derive(Debug, PartialEq)]
pub struct Impl {
    pub name: ParseTypeInner,
    pub methods: BTreeMap<Ident, FunctionDecl>,
    pub signatures: BTreeMap<Ident, FunctionSig>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseType {
    Function(Vec<ParseType>),
    Type(ParseTypeInner),
    Array(Box<ParseType>),
    Tuple(Vec<ParseType>),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseTypeInner {
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
    Expr(Ident),
    Type(Ident),
    Token(Token),
    Repetition(Vec<MacroFragment>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MacroInvoc {
    pub name: Ident,
    pub args: Vec<Token>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionSig {
    pub name: Ident,
    pub sig: ParseType,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDecl {
    pub name: Ident,
    pub lambda: LambdaDecl,
    pub inject_self: bool,
}

#[derive(Debug, PartialEq)]
pub struct LambdaDecl {
    pub parameters: Vec<Pattern>,
    pub body: Block,
    pub shorthand_tokens: Option<Vec<Token>>, // for formating
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment(Assignment),
    Expression(Expression),
    Return(Option<Expression>),
    Continue(Option<Expression>),
    Break(Option<Expression>),
    EmptyLine, // Empty line, kept for formating
}

#[derive(Debug, PartialEq)]
pub enum AssignmentLHS {
    Expression(UnaryExpr),
    Pattern(Pattern),
}

#[derive(Debug, PartialEq)]
pub struct Assignment {
    pub lhs: AssignmentLHS,
    pub rhs: Expression,
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
    pub type_annotation: Option<ParseType>,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Literal(Literal),
    Ident(IdentifierPath),
    /// Ident prefixed with a @ are desugared to self.ident
    SelfIdent(Ident),
    Instance(Instance),
    LambdaDecl(LambdaDecl),
    Tuple(Tuple),
    NativeOperator(NativeOperator),
    If(Box<If>),
    Match(Box<Match>),
    Loop(Box<Loop>),
    Unsafe(Block),
    Expression(Box<Expression>), // parenthesis
}

#[derive(Debug, PartialEq)]
pub struct Match {
    pub expr: Expression,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub condition: Option<Expression>,
    pub body: Block,
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    pub binding: Option<Ident>,
    pub kind: PatternKind,
}

#[derive(Debug, PartialEq)]
pub enum PatternKind {
    Ident(Ident),
    Literal(Literal),
    Tuple(Vec<Pattern>),
    Array(Vec<ArrayPattern>),
    Instance(InstancePattern),
    Nested(Box<Pattern>), // parenthesis
    Wildcard,
}

#[derive(Debug, PartialEq)]
pub enum ArrayPattern {
    Pattern(Pattern),
    Rest(Ident),
}

#[derive(Debug, PartialEq)]
pub struct InstancePattern {
    pub name: TypePath,
    pub args: FieldsPatternOrArgumentsPattern,
}

#[derive(Debug, PartialEq)]
pub enum FieldsPatternOrArgumentsPattern {
    Fields(Vec<FieldPattern>),
    Arguments(Vec<Pattern>),
}

#[derive(Debug, PartialEq)]
pub struct FieldPattern {
    pub name: Ident,
    pub pattern: Pattern,
}

#[derive(Debug, PartialEq)]
pub struct Tuple {
    pub elements: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct NativeOperator {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub condition: Expression,
    pub then: Block,
    pub else_: Option<Else>,
}

#[derive(Debug, PartialEq)]
pub enum Else {
    If(Box<If>),
    Block(Block),
}

#[derive(Debug, PartialEq)]
pub enum Loop {
    While(Expression, Block),
    For(Ident, Expression, Block),
    Loop(Block),
}

#[derive(Debug, PartialEq)]
pub struct Instance {
    pub name: TypePath,
    pub fields: BTreeMap<Ident, Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdentOrType {
    Ident(Ident),
    Type(ParseType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierPath {
    pub path: Vec<IdentOrType>,
}

#[derive(Debug, PartialEq)]
pub struct TypePath {
    pub path: Vec<IdentOrType>,
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
    Char(String),
}

#[derive(Debug, PartialEq)]
pub struct Array {
    pub elements: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum IdentOrNumber {
    Ident(Ident),
    Number(u64),
}

#[derive(Debug, PartialEq)]
pub enum SecondaryExpr {
    Arguments(Vec<Argument>),
    Indice(Box<Expression>), // Boxing here to keep the enum size low
    Dot(IdentOrNumber),
    DoubleDot(IdentOrNumber),
    Interogation,
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
