mod items;
mod parsable;
mod util;

use std::path::PathBuf;

use crate::ast::Program;
use parsable::Parsable;
use util::ParseError;

pub fn parse_file(file: PathBuf) -> Result<Program, ParseError> {
    let file = std::fs::read_to_string(file.clone())
        .map_err(|_e| ParseError::UnknownFile(file.to_str().unwrap().to_string()))?;

    let tokens = crate::lexer::Lexer::new(&file)
        .map_err(ParseError::Lexer)?
        .collect()
        .map_err(ParseError::Lexer)?;

    Program::parse(&tokens).map(|(program, _)| program)
}
