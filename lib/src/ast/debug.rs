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

    fn name_with_indent<F>(&mut self, name: &str, f: F)
    where
        F: FnOnce(&mut Self),
    {
        self.name(name);

        self.indent_level += 1;

        f(self);

        self.indent_level -= 1;
    }

    fn name(&mut self, name: &str) {
        println!("{}{}", self.indent(), name);
    }

    fn name_value<T>(&mut self, name: &str, value: T)
    where
        T: std::fmt::Display,
    {
        println!("{}{} = {}", self.indent(), name, value);
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
                self.name_value(ident.name(), &ident.name);
            }

            fn visit_parse_type(&mut self, parse_type: &'a ParseType) {
                self.name_value(parse_type.name(), parse_type.to_string());
            }

            fn visit_parse_type_inner(&mut self, parse_type: &'a ParseTypeInner) {
                self.name_value(parse_type.name(), parse_type.to_string());
            }

            fn visit_operator(&mut self, operator: &'a Operator) {
                self.name_value(operator.name(), operator.to_string());
            }

            fn visit_literal(&mut self, literal: &'a Literal) {
                match &literal.kind {
                    LiteralKind::Number(n) => {
                        self.name_value("Number", n);
                    }
                    LiteralKind::String(s) => {
                        self.name_value("String", s);
                    }
                    LiteralKind::Char(c) => {
                        self.name_value("Char", c);
                    }
                    LiteralKind::Float(f) => {
                        self.name_value("Float", f);
                    }
                    LiteralKind::Bool(b) => {
                        self.name_value("Bool", b);
                    }
                    LiteralKind::Array(array) => {
                        self.name_with_indent("Array", |printer| {
                            walk_list!(printer, visit_expression, &array.elements);
                        });
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
                        self.name_with_indent("Arguments", |printer| {
                            walk_list!(printer, visit_argument, args);
                        });
                    }
                    SecondaryExpr::Indice(expr) => {
                        self.name_with_indent("Indice", |printer| {
                            expr.visit(printer);
                        });
                    }
                    SecondaryExpr::Interogation => {
                        self.name("Interogation");
                    }
                }
            }

            fn visit_pattern(&mut self, pattern: &'a Pattern) {
                match &pattern.kind {
                    PatternKind::Instance(instance) => {
                        self.name_with_indent("InstancePattern", |printer| {
                            instance.visit(printer);
                        });
                    }
                    PatternKind::Ident(field) => {
                        self.name_value("IdentPattern", field);
                    }
                    PatternKind::Array(array) => {
                        self.name_with_indent("ArrayPattern", |printer| {
                            walk_list!(printer, visit_pattern, array);
                        });
                    }
                    PatternKind::Tuple(tuple) => {
                        self.name_with_indent("TuplePattern", |printer| {
                            walk_list!(printer, visit_pattern, tuple);
                        });
                    }
                    PatternKind::Literal(literal) => {
                        self.name_with_indent("LiteralPattern", |printer| {
                            literal.visit(printer);
                        });
                    }
                    PatternKind::Wildcard => {
                        self.name("WildcardPattern");
                    }
                    PatternKind::Nested(nested) => {
                        self.name_with_indent("NestedPattern", |printer| {
                            nested.visit(printer);
                        });
                    }
                }
            }

            fn visit_assignment(&mut self, assignment: &'a Assignment) {
                self.name_with_indent("Assignment", |printer| {
                    printer.name_with_indent("LHS", |printer| {
                        assignment.lhs.visit(printer);
                    });

                    printer.name_with_indent("RHS", |printer| {
                        assignment.rhs.visit(printer);
                    });
                });

            }

            paste! {
                $(
                    fn [<visit_ $name:snake>](&mut self, node: &'a$name) {
                        self.name_with_indent(&node.name(), |printer| {
                            [<walk_ $name:snake>](printer, node);
                        });
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
    // Assignment
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
