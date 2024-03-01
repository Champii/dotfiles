mod items;
mod parsable;
mod parse_ctx;
mod util;

use std::path::PathBuf;

use crate::ast::Program;
pub use parsable::Parsable;
pub use parse_ctx::ParseCtx;
pub use util::ParseError;

pub fn parse_file(file_path: PathBuf) -> Result<Program, ParseError> {
    let file = std::fs::read_to_string(file_path.clone())
        .map_err(|_e| ParseError::UnknownFile(file_path.to_str().unwrap().to_string()))?;

    let tokens = crate::lexer::Lexer::new(file_path, &file)
        .map_err(ParseError::Lexer)?
        .collect()
        .map_err(ParseError::Lexer)?;

    println!("{:#?}", tokens);

    let mut parse_ctx = parse_ctx::ParseCtx::new();

    Program::parse(&tokens, &mut parse_ctx).map(|(program, _)| program)
}
