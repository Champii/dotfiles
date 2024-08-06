mod engine;
mod items;

use engine::*;
use items::program;

use crate::ast::tree::*;
use crate::lexer::{Lexer, TokenType};
use crate::{Config, DebugPrint};

/* // Execute parser F if token type is found but does not consume it
fn seek<'a, T, F>(token_type: TokenType, parser: F) -> impl Fn(Input<'a>) -> IResult<'a, T>
where
    F: Fn(Input<'a>) -> IResult<'a, T>,
{
    move |stream| {
        let (_, token) = stream.consume()?;

        if token.token_type == token_type {
            parser(stream)
        } else {
            Err(ParseError::UnexpectedToken(token.clone()))
        }
    }
} */

pub fn parse(config: &Config) -> Result<Program, ParseError> {
    let file_path = config.entry_file.clone();

    let file = std::fs::read_to_string(file_path.clone())
        .map_err(|_e| ParseError::UnknownFile(file_path.to_str().unwrap().to_string()))?;

    let mut lexer = Lexer::new(file_path, &file).map_err(ParseError::Lexer)?;
    let tokens = lexer.collect().map_err(ParseError::Lexer)?;

    if config.has_debug_print(DebugPrint::Tokens) {
        println!("{:#?}", tokens);
    }

    // parse_ctx.deduce_indent_step(&tokens);

    let (_ctx, program) = program.process(ParseCtx::from(&tokens, config))?;

    Ok(program)
}
