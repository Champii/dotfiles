mod items;
mod parsable;
mod parse_ctx;
mod util;

use std::path::PathBuf;

use crate::ast::Program;
pub use parsable::Parsable;
pub use parse_ctx::ParseCtx;
use util::ParseError;

pub fn parse_file(file: PathBuf) -> Result<Program, ParseError> {
    let file = std::fs::read_to_string(file.clone())
        .map_err(|_e| ParseError::UnknownFile(file.to_str().unwrap().to_string()))?;

    let tokens = crate::lexer::Lexer::new(&file)
        .map_err(ParseError::Lexer)?
        .collect()
        .map_err(ParseError::Lexer)?;

    println!("{:#?}", tokens);

    let mut parse_ctx = parse_ctx::ParseCtx::new();

    Program::parse(&tokens, &mut parse_ctx).map(|(program, _)| program)
}
