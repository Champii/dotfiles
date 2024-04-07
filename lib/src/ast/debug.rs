use super::{visit::*, *};
use crate::walk_list;
use paste::paste;

pub fn debug_ast(ast: &Program) {
    ast.visit(&mut AstPrinter::default());
}

#[derive(Default)]
struct AstPrinter {
    indent_level: usize,
}

impl AstPrinter {
    fn indent(&self) -> String {
        "  ".repeat(self.indent_level)
    }
}

macro_rules! ast_printer {
    ($(
        $name:ty
    )+) => {
        impl<'a> Visitor<'a> for AstPrinter {
            fn visit_name(&mut self, _name: &str) {}

            fn visit_primitive<T>(&mut self, _val: T)
            where
                T: std::fmt::Debug,
            {}

            fn visit_ident(&mut self, ident: &'a Ident) {
                println!("{}{} = {}", self.indent(), ident.name(), ident.name);
            }

            fn visit_parse_type(&mut self, parse_type: &'a ParseType) {
                println!("{}{} = {}", self.indent(), parse_type.name(), parse_type.to_string());
            }

            fn visit_parse_type_inner(&mut self, parse_type: &'a ParseTypeInner) {
                println!("{}{} = {}", self.indent(), parse_type.name(), parse_type.to_string());
            }

            fn visit_operator(&mut self, operator: &'a Operator) {
                println!("{}{} = {}", self.indent(), operator.name(), operator.to_string());
            }

            fn visit_literal(&mut self, literal: &'a Literal) {
                match &literal.kind {
                    LiteralKind::Number(n) => {
                        println!("{}{} = {}", self.indent(), "Number", n);
                    }
                    LiteralKind::String(s) => {
                        println!("{}{} = {:?}", self.indent(), "String", s);
                    }
                    LiteralKind::Char(c) => {
                        println!("{}{} = {}", self.indent(), "Char", c);
                    }
                    LiteralKind::Float(f) => {
                        println!("{}{} = {}", self.indent(), "Float", f);
                    }
                    LiteralKind::Bool(b) => {
                        println!("{}{} = {}", self.indent(), "Bool", b);
                    }
                    LiteralKind::Array(array) => {
                        println!("{}{}", self.indent(), "Array");
                        self.indent_level += 1;
                        walk_list!(self, visit_expression, &array.elements);
                        self.indent_level -= 1;
                    }
                }
            }

            fn visit_secondary_expr(&mut self, secondary_expr: &'a SecondaryExpr) {
                match secondary_expr {
                    SecondaryExpr::Dot(name) => {
                        println!("{}{} = .{}", self.indent(), "Dot", name);
                    }
                    SecondaryExpr::DoubleDot(name) => {
                        println!("{}{} = ..{}", self.indent(), "DoubleDot", name);
                    }
                    SecondaryExpr::Arguments(args) => {
                        println!("{}{}", self.indent(), "Arguments");
                        self.indent_level += 1;
                        walk_list!(self, visit_argument, args);
                        self.indent_level -= 1;
                    }
                    SecondaryExpr::Indice(expr) => {
                        println!("{}{}", self.indent(), "Indice");
                        self.indent_level += 1;
                        expr.visit(self);
                        self.indent_level -= 1;
                    }
                    SecondaryExpr::Interogation => {
                        println!("{}{}", self.indent(), "Interogation");
                    }
                }
            }

            fn visit_pattern(&mut self, pattern: &'a Pattern) {
                match &pattern.kind {
                    PatternKind::Instance(instance) => {
                        println!("{}{}", self.indent(), "InstancePattern");
                        self.indent_level += 1;
                        instance.visit(self);
                        self.indent_level -= 1;
                    }
                    PatternKind::Ident(field) => {
                        println!("{}{} = {}", self.indent(), "IdentPattern", field);
                    }
                    PatternKind::Array(array) => {
                        println!("{}{}", self.indent(), "ArrayPattern");
                        self.indent_level += 1;
                        walk_list!(self, visit_pattern, array);
                        self.indent_level -= 1;
                    }
                    PatternKind::Tuple(tuple) => {
                        println!("{}{}", self.indent(), "TuplePattern");
                        self.indent_level += 1;
                        walk_list!(self, visit_pattern, tuple);
                        self.indent_level -= 1;
                    }
                    PatternKind::Literal(literal) => {
                        println!("{}{}", self.indent(), "LiteralPattern");
                        self.indent_level += 1;
                        literal.visit(self);
                        self.indent_level -= 1;
                    }
                    PatternKind::Wildcard => {
                        println!("{}{}", self.indent(), "WildcardPattern");
                    }
                    PatternKind::Nested(nested) => {
                        println!("{}{}", self.indent(), "NestedPattern");
                        self.indent_level += 1;
                        nested.visit(self);
                        self.indent_level -= 1;
                    }
                }
            }


            paste! {
                $(
                    fn [<visit_ $name:snake>](&mut self, node: &'a$name) {
                        println!("{}{}", self.indent(), node.name());
                        self.indent_level += 1;
                        [<walk_ $name:snake>](self, node);
                        self.indent_level -= 1;
                    }
                )+
            }
        }

    };
}

ast_printer!(
    Program
    ModuleDecl
    Module
    TopLevel
    MacroDecl
    MacroInvoc
    TraitDecl
    Impl
    EnumDecl
    EnumVariant
    // NamedFieldsOrTypesList
    FunctionDecl
    FunctionSig
    LambdaDecl
    // Block
    StructDecl
    StructDeclField
    // Ident
    // IdentOrNumber
    Assignment
    // AssignmentLHS
    // IdentifierPath
    // Statement
    Loop
    // Expression
    If
    Else
    Match
    MatchArm
    // Pattern
    // PatternKind
    // InstancePattern
    // FieldPattern
    // ArrayPattern
    // FieldsPatternOrArgumentsPattern
    // UnaryExpr
    // Operator
    /* PrimaryExpr
    SecondaryExpr */
    // Operand
    Argument
    // Literal
    Instance
    NativeOperator
    Tuple
    Array
    // ParseType
    // ParseTypeInner
    // IdentOrType
);
