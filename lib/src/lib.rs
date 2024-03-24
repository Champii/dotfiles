use std::path::PathBuf;

use crate::{ast::Program, diagnostic::Diagnostic};
pub mod ast;
mod diagnostic;
mod fmt;
mod lexer;
mod macro_expansion;
mod parser;

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub entry_file: PathBuf,
    pub output_dir: PathBuf,
    pub debug_print: Vec<String>,
    pub meta_files: Vec<(String, PathBuf)>, // crate name, path
}

impl Config {
    pub fn has_debug_print(&self, name: &str) -> bool {
        self.debug_print.contains(&name.to_string())
    }
}

pub fn compile(config: &Config) {
    let ast: Program = match parser::parse_root_file(&config) {
        Ok(ast) => ast,
        Err(e) => {
            Diagnostic::from(e).report();
            return;
        }
    };

    if config.has_debug_print("ast") {
        println!("{:#?}", ast);
    }

    // std::fs::write(std::env::args().nth(1).unwrap(), ast.to_string()).unwrap();

    let ast = match macro_expansion::expand_macros(ast) {
        Ok(ast) => ast,
        Err(e) => {
            Diagnostic::from(e).report();
            return;
        }
    };

    if config.has_debug_print("expanded") {
        println!("{:#?}", ast);
    }
}

pub fn format(config: &Config) {
    let ast: Program = match parser::parse_root_file(&config) {
        Ok(ast) => ast,
        Err(e) => {
            Diagnostic::from(e).report();
            return;
        }
    };

    if config.has_debug_print("ast") {
        println!("{:#?}", ast);
    }

    std::fs::write(std::env::args().nth(1).unwrap(), ast.to_string()).unwrap();
}
