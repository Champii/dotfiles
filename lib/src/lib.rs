use std::path::PathBuf;

use ast::debug::debug_ast;

use crate::ast::Program;
pub mod ast;
mod diagnostic;
mod fmt;
mod lexer;
pub mod macro_expansion;
pub mod new_parser;
pub mod parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebugPrint {
    Ast,
    AstFull,
    Expanded,
    Tokens,
}

impl From<&str> for DebugPrint {
    fn from(s: &str) -> Self {
        match s {
            "ast" => DebugPrint::Ast,
            "ast-full" => DebugPrint::AstFull,
            "expanded" => DebugPrint::Expanded,
            "tokens" => DebugPrint::Tokens,
            _ => panic!(
                "Unknown debug print: {}\nValid options are: ast, ast-full, expanded, tokens",
                s
            ),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub entry_file: PathBuf,
    pub output_dir: PathBuf,
    pub debug_print: Vec<DebugPrint>,
    pub meta_files: Vec<(String, PathBuf)>, // crate name, path
}

impl Config {
    pub fn has_debug_print(&self, name: DebugPrint) -> bool {
        self.debug_print.contains(&name)
    }
}

pub fn compile(config: &Config) {
    let ast: Program = match new_parser::parse(&config) {
        Ok(ast) => ast,
        Err(e) => {
            // e.report();
            eprintln!("{:?}", e);
            return;
        }
    };

    if config.has_debug_print(DebugPrint::AstFull) {
        println!("{:#?}", ast);
    }

    if config.has_debug_print(DebugPrint::Ast) {
        debug_ast(&ast);
    }

    let ast = match macro_expansion::expand_macros(ast) {
        Ok(ast) => ast,
        Err(e) => {
            e.report();
            return;
        }
    };

    if config.has_debug_print(DebugPrint::Expanded) {
        println!("{:#?}", ast);
    }
}
