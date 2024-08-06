use items::primitives::ident;

use crate::{lexer::TokenType, new_parser::*};

use super::top_level;

pub fn module_inline(stream: Input) -> IResult<Module> {
    (many(top_level), TokenType::Eof)
        .map(|(top_levels, _)| Module {
            name: None,
            top_levels,
            comment: None,
            is_inline: true,
            filepath: None,
        })
        .process(stream)
}

pub fn module(stream: Input) -> IResult<Module> {
    (
        TokenType::Keyword("mod".to_string()),
        ident,
        TokenType::Eol,
        many(top_level),
        TokenType::Eof,
    )
        .map(|(_, name, _, top_levels, _)| Module {
            name: Some(name),
            top_levels,
            comment: None,
            is_inline: false,
            filepath: None,
        })
        .process(stream)
}
