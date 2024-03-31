use core::fmt;
use std::{
    fmt::{Display, Formatter},
    sync::Mutex,
};

use crate::{ast::*, lexer::TokenType};

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
        write!(f, "{}", self.module)
    }
}

impl Display for ModuleDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.0.name {
            write!(f, "mod {}\n", name)?;
        }

        Ok(())
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_inline {
            increase_indent();
        }

        for (i, top_level) in self.top_levels.iter().enumerate() {
            write!(f, "{}", indent())?;
            write!(f, "{}", top_level)?;
            if i < self.top_levels.len() - 1 {
                write!(f, "\n")?;
            }
        }

        if self.is_inline {
            decrease_indent();
        }

        Ok(())
    }
}

impl Display for TopLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TopLevelKind::Module(module) => write!(f, "{}", module),
            TopLevelKind::InfixOperator(precedence, decl) => {
                write!(f, "infix {} {}", precedence, decl)
            }
            TopLevelKind::Import(path) => write!(f, "> {}\n", path),
            TopLevelKind::Export(path) => write!(f, "< {}\n", path),
            TopLevelKind::MacroDecl(decl) => write!(f, "{}", decl),
            TopLevelKind::MacroInvoc(invoc) => write!(f, "{}\n", invoc),
            TopLevelKind::Extern(sig) => write!(f, "extern {}", sig),
            TopLevelKind::FunctionSig(sig) => write!(f, "{}", sig),
            TopLevelKind::FunctionDecl(decl) => write!(f, "{}", decl),
            TopLevelKind::StructDecl(decl) => write!(f, "{}", decl),
            TopLevelKind::TraitDecl(decl) => write!(f, "{}", decl),
            TopLevelKind::EnumDecl(decl) => write!(f, "{}", decl),
            TopLevelKind::Impl(impl_) => write!(f, "{}", impl_),
            TopLevelKind::Comment(comment) => write!(f, "#{}\n", comment),
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
            write!(f, "{} : {}\n", name, signature)?;
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

        for (_, method) in &self.methods {
            write!(f, "{}", indent())?;
            write!(f, "{}", method)?;
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
        write!(f, "{} : {}\n", self.name, self.sig)
    }
}

impl Display for FunctionDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}\n", self.name, self.lambda)
    }
}

impl Display for LambdaDecl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(tokens) = &self.shorthand_tokens {
            for token in tokens {
                write!(f, "{}", token)?;
            }

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
            Statement::Return(expr) => write!(f, "return {}", expr),
            Statement::Continue(expr) => write!(f, "continue {}", expr),
            Statement::Break(expr) => write!(f, "break {}", expr),
            Statement::EmptyLine => write!(f, ""),
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
            Operand::NativeOperator(op) => write!(f, "{}", op),
            Operand::EnumInstance(inst) => write!(f, "{}", inst),
            Operand::LambdaDecl(decl) => write!(f, "{}", decl),
            Operand::Tuple(tuple) => write!(f, "{}", tuple),
            Operand::If(if_) => write!(f, "{}", if_),
            Operand::Loop(loop_) => write!(f, "{}", loop_),
            Operand::Expression(expr) => write!(f, "({})", expr),
        }
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

        for arg in &self.args {
            write!(f, " {}", arg)?;
        }

        Ok(())
    }
}

impl Display for SecondaryExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SecondaryExpr::Arguments(args) => {
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
        write!(f, "{}\n", self.name)?;

        increase_indent();

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

impl Display for EnumInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.name, self.variant)
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

infix 7 |> = a -> a

macro my_macro
  $name:ident $($arg:ident)* =>
    $name
    $($arg)*

%my_macro lol

struct MyStruct
  field: Int

enum MyEnum
  Foo Bar
  Baz

trait MyTrait
  foo: Bar
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
  Foo::Bar Baz
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

        let program: Program = crate::parser::parse_string(input).unwrap();

        println!("{}", input);
        println!("{}", program);

        assert_eq!(input, program.to_string());
    }
}
