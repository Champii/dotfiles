pub mod engine;
mod items;

use std::path::PathBuf;

pub use engine::*;
pub use items::*;

use crate::ast::tree::*;
use crate::lexer::{Lexer, TokenType};
use crate::{Config, DebugPrint};

pub fn parse(config: &Config) -> Result<Program, ParseError> {
    let file_path = config.entry_file.clone();

    parse_module(file_path, config).map(|module| Program { module })
}

pub fn parse_module(file_path: PathBuf, config: &Config) -> Result<Module, ParseError> {
    let file = std::fs::read_to_string(file_path.clone())
        .map_err(|_e| ParseError::UnknownFile(file_path.to_str().unwrap().to_string()))?;

    let mut lexer = Lexer::new(file_path, &file).map_err(ParseError::Lexer)?;
    let tokens = lexer.collect().map_err(ParseError::Lexer)?;

    if config.has_debug_print(DebugPrint::Tokens) {
        println!("{:#?}", tokens);
    }

    // Reset the best error tracker at the start of parsing
    engine::reset_best_error();

    let ctx = ParseCtx::from(&tokens, config);
    let result = module_inline.process(ctx);

    match result {
        Ok((_, program)) => Ok(program),
        Err(e) => {
            // Return the best error we've seen during parsing
            Err(engine::get_best_error(e))
        }
    }
}

pub fn parse_string(input: &str, config: &Config) -> Result<Program, ParseError> {
    let mut lexer = Lexer::new(PathBuf::new(), input).map_err(ParseError::Lexer)?;

    let tokens = lexer.collect().map_err(ParseError::Lexer)?;

    if config.has_debug_print(DebugPrint::Tokens) {
        println!("{:#?}", tokens);
    }

    // Reset the best error tracker at the start of parsing
    engine::reset_best_error();

    let ctx = ParseCtx::from(&tokens, config);
    let result = program.process(ctx);

    match result {
        Ok((_, program)) => Ok(program),
        Err(e) => {
            // Return the best error we've seen during parsing
            Err(engine::get_best_error(e))
        }
    }
}
