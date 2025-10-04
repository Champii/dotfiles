use crate::{
    lexer::TokenType,
    new_parser::{engine::*, indent_token},
};

pub fn empty_lines(stream: Input) -> IResult<usize> {
    many(
        (indent_token, TokenType::Eol)
            .map(|_| ())
            .or(TokenType::Eol.map(|_| ()))
    )
        .map(|x| x.len())
        .process(stream)
}
