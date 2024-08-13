use crate::{
    lexer::TokenType,
    new_parser::{engine::*, Array},
};

use super::expression;

pub fn array(stream: Input) -> IResult<Array> {
    (
        TokenType::OpenBracket,
        separated(expression, TokenType::Coma),
        TokenType::CloseBracket,
    )
        .map(|(_, elements, _)| Array { elements })
        .process(stream)
}
