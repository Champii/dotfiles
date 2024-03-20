use core::fmt;
use std::{
    fmt::{Display, Formatter},
    sync::Mutex,
};

use crate::{ast::*, lexer::Token};

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
    *indent += 2;
}

fn decrease_indent() {
    let mut indent = INDENT.lock().unwrap();
    *indent -= 2;
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for top_level in &self.top_levels {
            write!(f, "{}\n", top_level)?;
        }

        Ok(())
    }
}

impl Display for TopLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TopLevelKind::InfixOperator(precedence, decl) => {
                write!(f, "infix {} {}", precedence, decl)
            }
            TopLevelKind::MacroDecl(decl) => write!(f, "{}", decl),
            TopLevelKind::MacroInvoc(invoc) => write!(f, "{}", invoc),
            TopLevelKind::FunctionDecl(decl) => write!(f, "{}", decl),
            TopLevelKind::StructDecl(decl) => write!(f, "{}", decl),
            TopLevelKind::TraitDecl(decl) => write!(f, "{}", decl),
            TopLevelKind::EnumDecl(decl) => write!(f, "{}", decl),
            TopLevelKind::Impl(impl_) => write!(f, "{}", impl_),
        }
    }
}

impl Display for StructDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "struct {}\n", self.name)?;

        increase_indent();

        for (name, field) in &self.fields {
            write!(f, "{}", indent())?;
            write!(f, "{}: {}\n", name, field)?;
        }

        decrease_indent();

        Ok(())
    }
}

impl Display for EnumDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "enum {}", self.name)?;

        increase_indent();

        for variant in &self.variants {
            write!(f, "{}", indent())?;
            write!(f, "{}", variant)?;
        }

        decrease_indent();

        Ok(())
    }
}

impl Display for MacroDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "macro {}", self.name)?;

        increase_indent();

        for entry in &self.entries {
            write!(f, "{}", indent())?;
            write!(f, "{}", entry)?;
        }

        decrease_indent();

        Ok(())
    }
}

impl Display for MacroEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        increase_indent();

        for (i, def) in self.defs.iter().enumerate() {
            write!(f, "{}", indent())?;
            write!(f, "{} =>\n{}", def, self.body[i])?;
        }

        decrease_indent();

        Ok(())
    }
}

impl Display for MacroFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MacroFragment::Ident(ident) => write!(f, "${}", ident),
            MacroFragment::Token(token) => write!(f, "{}", token),
            MacroFragment::Repetition(fragments) => {
                write!(f, "$( ")?;

                for fragment in fragments {
                    write!(f, "{}", fragment)?;
                }

                write!(f, ")*")
            }
        }
    }
}

impl Display for MacroInvoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "${}", self.name)?;

        for arg in &self.args {
            write!(f, "{}", arg)?;
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
            write!(f, "{}: {}\n", name, signature)?;
        }

        for (_, method) in &self.methods {
            write!(f, "{}", indent())?;
            write!(f, "{}\n", method)?;
        }

        decrease_indent();

        Ok(())
    }
}

impl Display for Impl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "impl {}", self.name)?;

        increase_indent();

        for (name, method) in &self.methods {
            write!(f, "{}", indent())?;
            write!(f, "{} = {}\n", name, method)?;
        }

        decrease_indent();

        Ok(())
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

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for ParseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;

        for (i, generic) in self.generics.iter().enumerate() {
            write!(f, "{}", generic)?;

            if i < self.generics.len() - 1 {
                write!(f, ", ")?;
            }
        }

        Ok(())
    }
}

impl Display for FunctionDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.name, self.lambda)
    }
}

impl Display for LambdaDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, param) in self.parameters.iter().enumerate() {
            write!(f, "{}", param)?;

            if i < self.parameters.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, " -> {}", self.body)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mono_statement = self.statements.len() <= 1;
        if !mono_statement {
            increase_indent();
            write!(f, "\n")?;
        }

        for stmt in &self.statements {
            if !mono_statement {
                write!(f, "{}", indent())?;
            }
            write!(f, "{}", stmt)?;

            if !mono_statement {
                write!(f, "\n")?;
            }
        }

        if !mono_statement {
            decrease_indent();
        }

        Ok(())
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Assignment(assign) => write!(f, "{}", assign),
            Statement::Expression(expr) => write!(f, "{}", expr),
            Statement::Return(expr) => write!(f, "return {}", expr),
            Statement::Continue(expr) => write!(f, "continue {}", expr),
            Statement::Break(expr) => write!(f, "break {}", expr),
        }
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.lhs, self.rhs)
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
            for secondary in secondaries {
                write!(f, "{}", secondary)?;
            }
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
            Operand::StructInstance(inst) => write!(f, "{}", inst),
            Operand::EnumInstance(inst) => write!(f, "{}", inst),
            Operand::LambdaDecl(decl) => write!(f, "{}", decl),
            Operand::If(if_) => write!(f, "{}", if_),
            Operand::Loop(loop_) => write!(f, "{}", loop_),
            Operand::Expression(expr) => write!(f, "({})", expr),
        }
    }
}

impl Display for SecondaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SecondaryExpr::Arguments(args) => {
                for (i, arg) in args.iter().enumerate() {
                    write!(f, "{}", arg)?;

                    if i < args.len() - 1 {
                        write!(f, ", ")?;
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

impl Display for StructInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;

        increase_indent();

        for (field, value) in &self.fields {
            write!(f, "{}", indent())?;
            write!(f, "{}: {}\n", field, value)?;
        }

        decrease_indent();

        Ok(())
    }
}

impl Display for EnumInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.name, self.variant)
    }
}

impl Display for If {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "if {}", self.condition)?;
        write!(f, "then {}", self.then)?;

        if let Some(else_) = &self.else_ {
            write!(f, " else {}", else_)
        } else {
            Ok(())
        }
    }
}

impl Display for Else {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Else::If(if_) => write!(f, "{}", if_),
            Else::Block(block) => write!(f, "{}", block),
        }
    }
}

impl Display for Loop {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Loop::While(cond, block) => {
                write!(f, "while {}", cond)?;
                write!(f, "do {}", block)
            }
            Loop::For(ident, cond, block) => {
                write!(f, "for {} in {}", ident, cond)?;
                write!(f, "do {}", block)
            }
            Loop::Loop(block) => write!(f, "{}", block),
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
