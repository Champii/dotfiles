use crate::{
    lexer::TokenType,
    new_parser::{engine::*, Array, Expression},
};

use super::{empty_lines, expression, indent};

pub fn multiline_array(stream: Input) -> IResult<Vec<Expression>> {
    indented(preceded(
        TokenType::Eol,
        preceded(
            empty_lines,
            separated_trailing(
                preceded(indent, separated1(expression, TokenType::Coma)),
                (
                    TokenType::Coma.opt(),
                    TokenType::Eol.followed_by(empty_lines),
                ),
            ),
        )
        .map(|elements| elements.into_iter().flatten().collect::<Vec<_>>()),
    ))
    .followed_by(indent)
    .process(stream)
}

pub fn monoline_array(stream: Input) -> IResult<Vec<Expression>> {
    separated_trailing(expression, TokenType::Coma).process(stream)
}

pub fn array(stream: Input) -> IResult<Array> {
    delimited(
        TokenType::OpenBracket,
        multiline_array.or(monoline_array),
        TokenType::CloseBracket,
    )
    .map(|elements| Array { elements })
    .process(stream)
    .map_err(|e| e.with_context("array"))
}
