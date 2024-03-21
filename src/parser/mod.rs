mod items;
mod parsable;
mod parse_ctx;
mod util;

use std::path::PathBuf;

use crate::lexer::Lexer;
pub use parsable::Parsable;
pub use parse_ctx::ParseCtx;
pub use util::ParseError;

pub fn parse_file<T: Parsable>(file_path: PathBuf) -> Result<T, ParseError> {
    let file = std::fs::read_to_string(file_path.clone())
        .map_err(|_e| ParseError::UnknownFile(file_path.to_str().unwrap().to_string()))?;

    let lexer = Lexer::new(file_path, &file).map_err(ParseError::Lexer)?;

    parse(lexer)
}

#[allow(dead_code)]
pub fn parse_string<T: Parsable>(input: &str) -> Result<T, ParseError> {
    let lexer = Lexer::new(PathBuf::new(), input).map_err(ParseError::Lexer)?;

    parse(lexer)
}

pub fn parse<T: Parsable>(mut lexer: Lexer) -> Result<T, ParseError> {
    let tokens = lexer.collect().map_err(ParseError::Lexer)?;

    println!("{:#?}", tokens);

    let mut parse_ctx = parse_ctx::ParseCtx::new();
    parse_ctx.deduce_indent_step(&tokens);

    T::parse(&tokens, &mut parse_ctx).map(|(program, _)| program)
}
