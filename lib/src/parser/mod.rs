mod items;
mod parsable;
mod parse_ctx;
mod util;

use std::path::PathBuf;

use crate::lexer::Lexer;
use crate::Config;
pub use parsable::Parsable;
pub use parse_ctx::ParseCtx;
pub use util::ParseError;

pub fn parse_root_file<T: Parsable>(config: &Config) -> Result<T, ParseError> {
    let file_path = config.entry_file.clone();

    let file = std::fs::read_to_string(file_path.clone())
        .map_err(|_e| ParseError::UnknownFile(file_path.to_str().unwrap().to_string()))?;

    let lexer = Lexer::new(file_path, &file).map_err(ParseError::Lexer)?;

    parse(lexer, &mut ParseCtx::new(config))
}

pub fn parse_file<T: Parsable>(
    file_path: PathBuf,
    parse_ctx: &mut ParseCtx,
) -> Result<T, ParseError> {
    let file = std::fs::read_to_string(file_path.clone())
        .map_err(|_e| ParseError::UnknownFile(file_path.to_str().unwrap().to_string()))?;

    let lexer = Lexer::new(file_path, &file).map_err(ParseError::Lexer)?;

    parse(lexer, parse_ctx)
}

#[allow(dead_code)]
pub fn parse_string<T: Parsable>(input: &str) -> Result<T, ParseError> {
    let lexer = Lexer::new(PathBuf::new(), input).map_err(ParseError::Lexer)?;

    parse(lexer, &mut ParseCtx::new(&Config::default()))
}

pub fn parse<T: Parsable>(mut lexer: Lexer, parse_ctx: &mut ParseCtx) -> Result<T, ParseError> {
    let tokens = lexer.collect().map_err(ParseError::Lexer)?;

    if parse_ctx.config.has_debug_print("tokens") {
        println!("{:#?}", tokens);
    }

    parse_ctx.deduce_indent_step(&tokens);

    T::parse(&tokens, parse_ctx).map(|(program, _)| program)
}
