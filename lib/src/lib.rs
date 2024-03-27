use std::path::PathBuf;

use crate::ast::Program;
pub mod ast;
mod diagnostic;
mod fmt;
mod lexer;
pub mod macro_expansion;
pub mod parser;

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
            e.report();
            return;
        }
    };

    if config.has_debug_print("ast") {
        println!("{:#?}", ast);
    }

    let ast = match macro_expansion::expand_macros(ast) {
        Ok(ast) => ast,
        Err(e) => {
            e.report();
            return;
        }
    };

    if config.has_debug_print("expanded") {
        println!("{:#?}", ast);
    }
}
