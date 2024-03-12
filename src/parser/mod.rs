mod items;
mod parsable;
mod parse_ctx;
mod util;

use std::path::PathBuf;

use crate::{ast::Program, lexer::Lexer};
pub use parsable::Parsable;
pub use parse_ctx::ParseCtx;
pub use util::ParseError;

pub fn parse_file(file_path: PathBuf) -> Result<Program, ParseError> {
    let file = std::fs::read_to_string(file_path.clone())
        .map_err(|_e| ParseError::UnknownFile(file_path.to_str().unwrap().to_string()))?;

    let lexer = Lexer::new(file_path, &file).map_err(ParseError::Lexer)?;

    parse(lexer)
}

#[allow(dead_code)]
pub fn parse_string(input: &str) -> Result<Program, ParseError> {
    let lexer = Lexer::new(PathBuf::new(), input).map_err(ParseError::Lexer)?;

    parse(lexer)
}

pub fn parse(mut lexer: Lexer) -> Result<Program, ParseError> {
    let tokens = lexer.collect().map_err(ParseError::Lexer)?;

    println!("{:#?}", tokens);

    let mut parse_ctx = parse_ctx::ParseCtx::new();

    Program::parse(&tokens, &mut parse_ctx).map(|(program, _)| program)
}
