use core::fmt;
use std::{
    fmt::{Display, Formatter},
    sync::Mutex,
};

use crate::{ast::*, lexer::TokenType};

// TODO: Implement a custom Display trait to give the indent context
static INDENT: Mutex<u8> = Mutex::new(0);

fn indent() -> String {
    let indent = INDENT.lock().unwrap();

    let mut s = String::new();

    for _ in 0..*indent {
        s.push_str(" ");
    }

    s
}

fn increase_indent() {
    let mut indent = INDENT.lock().unwrap();

    *indent += 4;
}

fn decrease_indent() {
    let mut indent = INDENT.lock().unwrap();

    *indent -= 4;
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.module)
    }
}

impl Display for ModuleDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.0.name {
            write!(f, "mod {}", name)?;

            if let Some(comment) = &self.0.comment {
                write!(f, " //{}", comment)?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.is_inline {
            increase_indent();
        }

        for (i, top_level) in self.top_levels.iter().enumerate() {
            write!(f, "{}", indent())?;
            write!(f, "{}", top_level)?;

            if i < self.top_levels.len() - 1 {
                // if the top level is a functiondecl or a comment, check if the last statement of the block is
                // an empty line and if so, don't add an extra newline
                if let TopLevel::FunctionDecl(decl) = &top_level {
                    if let Some(last) = &decl.lambda.body.statements.last() {
                        if let Statement::EmptyLine = last {
                            continue;
                        }
                    }
                }

                if let TopLevel::Comment(_) = &top_level {
                    continue;
                }

                write!(f, "\n")?;
            }
        }

        if !self.is_inline {
            decrease_indent();
        }

        Ok(())
    }
}

impl Display for TopLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            TopLevel::Module(module) => write!(f, "{}", module),
            TopLevel::InfixOperator(precedence, decl) => {
                write!(f, "infix {} {}\n", precedence, decl)
            }
            TopLevel::Import(path) => write!(f, "> {}\n", path),
            TopLevel::Export(path) => write!(f, "< {}\n", path),
            TopLevel::MacroDecl(decl) => write!(f, "{}", decl),
            TopLevel::MacroInvoc(invoc) => write!(f, "{}\n", invoc),
            TopLevel::Extern(sig) => write!(f, "extern {}", sig),
            TopLevel::FunctionSig(sig) => write!(f, "{}", sig),
            TopLevel::FunctionDecl(decl) => write!(f, "{}", decl),
            TopLevel::StructDecl(decl) => write!(f, "{}", decl),
            TopLevel::TraitDecl(decl) => write!(f, "{}", decl),
            TopLevel::EnumDecl(decl) => write!(f, "{}", decl),
            TopLevel::Impl(impl_) => write!(f, "{}", impl_),
            TopLevel::Comment(comment) => write!(f, "//{}\n", comment),
            TopLevel::NewType(inner, ty) => write!(f, "type {} = {}\n", inner, ty),
        }
    }
}

impl Display for StructDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "struct {}\n", self.name)?;

        increase_indent();

        for field in &self.fields {
            write!(f, "{}", indent())?;
            write!(f, "{}\n", field)?;
        }

        decrease_indent();

        Ok(())
    }
}

impl Display for StructDeclField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let public_str = if self.public { "< " } else { "" };

        write!(f, "{}{} : {}", public_str, self.name, self.ty)
    }
}

impl Display for EnumDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "enum {}\n", self.name)?;

        increase_indent();

        for variant in &self.variants {
            write!(f, "{}", indent())?;
            write!(f, "{}\n", variant)?;
        }

        decrease_indent();

        Ok(())
    }
}

impl Display for EnumVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        write!(f, "{}", self.fields)
    }
}

impl Display for NamedFieldsOrTypesList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NamedFieldsOrTypesList::NamedFields(fields) => {
                if fields.is_empty() {
                    return Ok(());
                }

                write!(f, "\n")?;

                increase_indent();

                for field in fields {
                    write!(f, "{}", indent())?;
                    write!(f, "{}\n", field)?;
                }

                decrease_indent();

                Ok(())
            }
            NamedFieldsOrTypesList::TypesList(types) => {
                for (i, ty) in types.iter().enumerate() {
                    write!(f, " {}", ty)?;

                    if i < types.len() - 1 {
                        write!(f, ",")?;
                    }
                }

                Ok(())
            }
        }
    }
}

impl Display for MacroDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "macro {}\n", self.name)?;

        for entry in &self.entries {
            write!(f, "{}", entry)?;
        }

        Ok(())
    }
}

impl Display for MacroEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        increase_indent();

        write!(f, "{}", indent())?;
        for def in &self.defs {
            write!(f, "{}", HeadMacroFragment(def.clone()))?;
            write!(f, " ")?;
        }

        write!(f, "=>\n")?;

        increase_indent();
        for (i, fragment) in self.body.iter().enumerate() {
            let next_is_eol = self.body.get(i + 1).map_or(true, |f| {
                if let MacroFragment::Token(token) = f {
                    token.token_type == TokenType::Eol
                } else {
                    false
                }
            });

            let write_space = !next_is_eol && i < self.body.len() - 1;

            if let MacroFragment::Token(token) = fragment {
                if let TokenType::Indent(_) = token.token_type {
                    write!(f, "{}", fragment)?;
                } else if let TokenType::StuckOperator(_) = token.token_type {
                    write!(f, "{}", fragment)?;
                } else {
                    write!(f, "{}", fragment)?;

                    if write_space && token.token_type != TokenType::Eol {
                        write!(f, " ")?;
                    }
                }
            } else {
                write!(f, "{}", fragment)?;

                if write_space {
                    write!(f, " ")?;
                }
            }
        }

        decrease_indent();
        decrease_indent();

        Ok(())
    }
}

impl Display for MacroFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MacroFragment::Ident(ident) => write!(f, "${}", ident),
            MacroFragment::Expr(ident) => write!(f, "${}", ident),
            MacroFragment::Type(ident) => write!(f, "${}", ident),
            MacroFragment::Token(token) => {
                if let TokenType::Indent(_) = token.token_type {
                    write!(f, "{}", indent())?;
                }
                write!(f, "{}", token)
            }
            MacroFragment::Repetition(fragments) => {
                write!(f, "$(")?;

                for fragment in fragments {
                    write!(f, "{}", fragment)?;
                }

                write!(f, ")*")
            }
        }
    }
}

struct HeadMacroFragment(MacroFragment);

impl Display for HeadMacroFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            MacroFragment::Ident(ident) => write!(f, "${}:ident", ident),
            MacroFragment::Expr(ident) => write!(f, "${}:expr", ident),
            MacroFragment::Type(ident) => write!(f, "${}:ty", ident),
            MacroFragment::Token(token) => write!(f, "{}", token),
            MacroFragment::Repetition(fragments) => {
                write!(f, "$(")?;

                for fragment in fragments {
                    write!(f, "{}", fragment)?;

                    if let MacroFragment::Ident(_) = fragment {
                        write!(f, ":ident")?;
                    }
                }

                write!(f, ")*")
            }
        }
    }
}

impl Display for MacroInvoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.name)?;

        for arg in &self.args {
            write!(f, " {}", arg)?;
        }

        Ok(())
    }
}

impl Display for TraitDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "trait {}\n", self.name)?;

        increase_indent();

        for (name, signature) in &self.signatures {
            write!(f, "{}", indent())?;
            write!(f, "{}", signature)?;
        }

        for (_, method) in &self.methods {
            write!(f, "{}", indent())?;
            write!(f, "{}", method)?;
        }

        decrease_indent();

        Ok(())
    }
}

impl Display for Impl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "impl {}\n", self.name)?;

        increase_indent();

        for (name, signature) in &self.signatures {
            write!(f, "{}", indent())?;
            write!(f, "{}", signature)?;
        }

        for (_, method) in &self.methods {
            write!(f, "{}", indent())?;
            write!(f, "{}", method)?;
        }

        decrease_indent();

        Ok(())
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Path::Ident(ident) => write!(f, "{}", ident),
            Path::Type(ty) => write!(f, "{}", ty),
        }
    }
}

impl Display for IdentifierPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, ident) in self.path.iter().enumerate() {
            write!(f, "{}", ident)?;

            if i < self.path.len() - 1 {
                write!(f, "::")?;
            }
        }

        Ok(())
    }
}

impl Display for TypePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, ident) in self.path.iter().enumerate() {
            write!(f, "{}", ident)?;

            if i < self.path.len() - 1 {
                write!(f, "::")?;
            }
        }

        Ok(())
    }
}

impl Display for IdentOrType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IdentOrType::Ident(ident) => write!(f, "{}", ident),
            IdentOrType::Type(parse_type) => write!(f, "{}", parse_type),
        }
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

static IS_INSIDE_FN_DECL: Mutex<bool> = Mutex::new(false);

impl Display for ParseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut has_toggled_inside_fn_type_decl = false;

        match self {
            ParseType::Function(types) => {
                if IS_INSIDE_FN_DECL.lock().unwrap().clone() {
                    write!(f, "(")?;
                } else {
                    has_toggled_inside_fn_type_decl = true;

                    *IS_INSIDE_FN_DECL.lock().unwrap() = true;
                }

                for (i, inner) in types.iter().enumerate() {
                    write!(f, "{}", inner)?;

                    if i < types.len() - 1 {
                        write!(f, " -> ")?;
                    }
                }

                if has_toggled_inside_fn_type_decl {
                    *IS_INSIDE_FN_DECL.lock().unwrap() = false;
                }

                if IS_INSIDE_FN_DECL.lock().unwrap().clone() {
                    write!(f, ")")?;
                }

                Ok(())
            }
            ParseType::Array(inner) => write!(f, "[{}]", inner),
            ParseType::Tuple(types) => {
                write!(f, "(")?;

                for (i, inner) in types.iter().enumerate() {
                    write!(f, "{}", inner)?;

                    if i < types.len() - 1 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, ")")
            }
            ParseType::Type(inner) => write!(f, "{}", inner),
            ParseType::Unit => write!(f, "()"),
        }
    }
}

impl Display for ParseTypeInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;

        for (i, generic) in self.generics.iter().enumerate() {
            write!(f, " {}", generic)?;

            if i < self.generics.len() - 1 {
                write!(f, ",")?;
            }
        }

        Ok(())
    }
}

impl Display for FunctionSig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let inject_self = if self.inject_self { "@" } else { "" };
        write!(f, "{}{} : {}\n", inject_self, self.name, self.sig)
    }
}

impl Display for FunctionDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let inject_self = if self.inject_self { "@" } else { "" };
        write!(f, "{}{} = {}\n", inject_self, self.name, self.lambda)
    }
}

impl Display for LambdaDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(tokens) = &self.shorthand_tokens {
            write!(f, "(")?;

            for token in tokens {
                write!(f, "{}", token)?;
            }

            write!(f, ")")?;

            return Ok(());
        }

        for (i, param) in self.parameters.iter().enumerate() {
            write!(f, "{}", param)?;

            if i < self.parameters.len() - 1 {
                write!(f, ", ")?;
            }
        }

        if !self.parameters.is_empty() {
            write!(f, " ")?;
        }

        write!(f, "->")?;

        if self.body.statements.len() <= 1 {
            write!(f, " ")?;
        }

        display_block(&self.body, false, f)
    }
}

fn display_block(block: &Block, force_multiline: bool, f: &mut Formatter<'_>) -> fmt::Result {
    let mono_statement = !force_multiline && block.statements.len() <= 1;

    if !mono_statement {
        increase_indent();
        write!(f, "\n")?;
    }

    let mut skip_next_empty_lines = false;

    'main: for (i, stmt) in block.statements.iter().enumerate() {
        while skip_next_empty_lines && Statement::EmptyLine == *stmt {
            continue 'main;
        }

        skip_next_empty_lines = false;

        if Statement::EmptyLine == *stmt {
            skip_next_empty_lines = true;
        }

        if !mono_statement && Statement::EmptyLine != *stmt {
            write!(f, "{}", indent())?;
        }

        write!(f, "{}", stmt)?;

        if !mono_statement && i < block.statements.len() - 1 {
            write!(f, "\n")?;
        }
    }

    if !mono_statement {
        decrease_indent();
    }

    Ok(())
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Assignment(assign) => write!(f, "{}", assign),
            Statement::Expression(expr) => write!(f, "{}", expr),
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    write!(f, "return {}", expr)
                } else {
                    write!(f, "return")
                }
            }
            Statement::Continue(expr) => {
                if let Some(expr) = expr {
                    write!(f, "continue {}", expr)
                } else {
                    write!(f, "continue")
                }
            }
            Statement::Break(expr) => {
                if let Some(expr) = expr {
                    write!(f, "break {}", expr)
                } else {
                    write!(f, "break")
                }
            }
            Statement::EmptyLine => write!(f, ""),
            Statement::Comment(comment) => write!(f, "//{}", comment),
        }
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.lhs, self.rhs)
    }
}

impl Display for AssignmentLHS {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AssignmentLHS::Expression(expr) => write!(f, "{}", expr),
            AssignmentLHS::Pattern(pattern) => write!(f, "{}", pattern),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expression::BinopExpr(lhs, op, rhs) => write!(f, "{} {} {}", lhs, op, rhs),
            Expression::UnaryExpr(expr) => write!(f, "{}", expr),
        }
    }
}

impl Display for UnaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UnaryExpr::PrimaryExpr(expr) => write!(f, "{}", expr),
            UnaryExpr::UnaryExpr(op, expr) => write!(f, "{}{}", op, expr),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for PrimaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.operand)?;

        if let Some(secondaries) = &self.secondaries {
            for (i, secondary) in secondaries.iter().enumerate() {
                write!(f, "{}", secondary)?;

                if let SecondaryExpr::Arguments(_) = secondary {
                    if i < secondaries.len() - 1 {
                        if let SecondaryExpr::Dot(_) = secondaries[i + 1] {
                            write!(f, " ")?;

                            continue;
                        }
                    }
                }
            }
        }

        if let Some(type_annotation) = &self.type_annotation {
            write!(f, " : {}", type_annotation)?;
        }

        Ok(())
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Literal(lit) => write!(f, "{}", lit),
            Operand::Ident(ident) => write!(f, "{}", ident),
            Operand::SelfIdent(ident) => write!(f, "{}", ident),
            Operand::Instance(inst) => write!(f, "{}", inst),
            Operand::NativeOperator(op) => write!(f, "{}", op),
            Operand::LambdaDecl(decl) => write!(f, "{}", decl),
            Operand::Tuple(tuple) => write!(f, "{}", tuple),
            Operand::If(if_) => write!(f, "{}", if_),
            Operand::Match(match_) => write!(f, "{}", match_),
            Operand::Loop(loop_) => write!(f, "{}", loop_),
            Operand::Expression(expr) => write!(f, "({})", expr),
            Operand::Unsafe(block) => {
                write!(f, "unsafe")?;
                display_block(block, true, f)
            }
        }
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "match {}\n", self.expr)?;

        increase_indent();

        for (i, arm) in self.arms.iter().enumerate() {
            write!(f, "{}", indent())?;
            write!(f, "{}", arm)?;

            if i < self.arms.len() - 1 {
                write!(f, "\n")?;
            }
        }

        decrease_indent();

        Ok(())
    }
}

impl Display for MatchArm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> ", self.pattern)?;
        display_block(&self.body, false, f)
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(binding) = &self.binding {
            write!(f, "{} @ ", binding)?;
        }

        write!(f, "{}", self.kind)
    }
}

impl Display for PatternKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PatternKind::Ident(ident) => write!(f, "{}", ident),
            PatternKind::Literal(lit) => write!(f, "{}", lit),
            PatternKind::Tuple(patterns) => {
                write!(f, "(")?;

                for (i, pattern) in patterns.iter().enumerate() {
                    write!(f, "{}", pattern)?;

                    if i < patterns.len() - 1 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, ")")
            }
            PatternKind::Array(patterns) => {
                write!(f, "[")?;

                for (i, pattern) in patterns.iter().enumerate() {
                    write!(f, "{}", pattern)?;

                    if i < patterns.len() - 1 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "]")
            }
            PatternKind::Instance(inst) => write!(f, "{}", inst),
            PatternKind::Nested(pattern) => write!(f, "({})", pattern),
            PatternKind::Wildcard => write!(f, "_"),
        }
    }
}

impl Display for ArrayPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ArrayPattern::Pattern(pattern) => write!(f, "{}", pattern),
            ArrayPattern::Rest(ident) => write!(f, "..{}", ident),
        }
    }
}

impl Display for InstancePattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        write!(f, "{}", self.args)
    }
}

impl Display for FieldsPatternOrArgumentsPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FieldsPatternOrArgumentsPattern::Fields(fields) => {
                if fields.is_empty() {
                    return Ok(());
                }

                for (i, field) in fields.iter().enumerate() {
                    write!(f, " {}", field)?;

                    if i < fields.len() - 1 {
                        write!(f, ",")?;
                    }
                }

                Ok(())
            }
            FieldsPatternOrArgumentsPattern::Arguments(args) => {
                if args.is_empty() {
                    return Ok(());
                }

                for (i, arg) in args.iter().enumerate() {
                    write!(f, " {}", arg)?;

                    if i < args.len() - 1 {
                        write!(f, ",")?;
                    }
                }

                Ok(())
            }
        }
    }
}

impl Display for FieldPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.pattern)
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;

        for (i, expr) in self.elements.iter().enumerate() {
            write!(f, "{}", expr)?;

            if i < self.elements.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")
    }
}

impl Display for NativeOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "%{}", self.name)?;

        for (i, arg) in self.args.iter().enumerate() {
            write!(f, " {}", arg)?;

            if i < self.args.len() - 1 {
                write!(f, ",")?;
            }
        }

        Ok(())
    }
}

impl Display for SecondaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SecondaryExpr::Arguments(args) => {
                if args.is_empty() {
                    write!(f, "!")?;

                    return Ok(());
                }

                for (i, arg) in args.iter().enumerate() {
                    write!(f, " {}", arg)?;

                    if i < args.len() - 1 {
                        write!(f, ",")?;
                    }
                }

                Ok(())
            }
            SecondaryExpr::Indice(indice) => {
                write!(f, "[{}]", indice)
            }
            SecondaryExpr::Dot(field) => {
                write!(f, ".{}", field)
            }
            SecondaryExpr::DoubleDot(field) => {
                write!(f, "..{}", field)
            }
            SecondaryExpr::Interogation => {
                write!(f, "?")
            }
        }
    }
}

impl Display for IdentOrNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IdentOrNumber::Ident(ident) => write!(f, "{}", ident),
            IdentOrNumber::Number(num) => write!(f, "{}", num),
        }
    }
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.arg)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            LiteralKind::Bool(b) => write!(f, "{}", b),
            LiteralKind::Number(num) => write!(f, "{}", num),
            LiteralKind::Float(num) => write!(f, "{}", num),
            LiteralKind::Array(s) => write!(f, "{}", s),
            LiteralKind::String(s) => write!(f, "\"{}\"", s),
            LiteralKind::Char(c) => write!(f, "'{}'", c),
        }
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;

        increase_indent();

        if !self.fields.is_empty() {
            write!(f, "\n")?;
        }

        for (i, (field, value)) in self.fields.iter().enumerate() {
            write!(f, "{}", indent())?;
            write!(f, "{}: {}", field, value)?;

            if i < self.fields.len() - 1 {
                write!(f, "\n")?;
            }
        }

        decrease_indent();

        Ok(())
    }
}

impl Display for If {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "if {}\n", self.condition)?;
        write!(f, "{}", indent())?;
        write!(f, "then")?;

        if self.then.statements.len() <= 1 {
            write!(f, " ")?;
        }

        display_block(&self.then, false, f)?;

        if let Some(else_) = &self.else_ {
            write!(f, "\n")?;
            write!(f, "{}", indent())?;
            write!(f, "else{}", else_)
        } else {
            Ok(())
        }
    }
}

impl Display for Else {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Else::If(if_) => write!(f, "{}", if_),
            Else::Block(block) => {
                if block.statements.len() <= 1 {
                    write!(f, " ")?;
                }
                display_block(block, false, f)
            }
        }
    }
}

impl Display for Loop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Loop::While(cond, block) => {
                write!(f, "while {}", cond)?;
                display_block(block, true, f)
            }
            Loop::For(ident, cond, block) => {
                write!(f, "for {} in {}", ident, cond)?;
                display_block(block, true, f)
            }
            Loop::Loop(block) => {
                write!(f, "loop")?;
                display_block(block, true, f)
            }
        }
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for (i, expr) in self.elements.iter().enumerate() {
            write!(f, "{}", expr)?;

            if i < self.elements.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod format {
    use crate::ast::Program;

    #[test]
    fn full_program() {
        let input = r#"> foo::Bar

infix 7 |>

macro my_macro
    $name:ident $($arg:ident)* =>
        $name = a -> a
        $($arg)*

%my_macro lol

struct MyStruct
    field : Int

enum MyEnum
    Foo Bar
    Baz

trait MyTrait
    foo : Bar -> Baz
    baz = a -> a

impl MyTrait
    baz = a -> a
    foo = a -> a

lambda = (*5)

main = ->
    foo a, b
    foo[a + b + -c]

    foo.bar.baz

    a = MyStruct
        field: 42

    a + a + c

    Foo::Bar baz

    for i in a
        a

    while a == b
        a = a + b

    loop
        foo bar

    if a
    then foo
    else bar

    if b
    then
        a b
        b c
    else
        c + d
        b c

    a = (.foo)

< MyTrait
"#;

        let config = crate::Config::default();

        let program: Program = crate::new_parser::parse_string(input, &config).unwrap();

        println!("{}", input);
        println!("{}", program);

        assert_eq!(input, program.to_string());
    }
}
