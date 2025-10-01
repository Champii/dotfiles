use crate::{
    lexer::TokenType,
    new_parser::{engine::*, Block},
};

use super::{empty_lines, indent, statement};

pub fn block(stream: Input) -> IResult<Block> {
    preceded(
        TokenType::Eol,
        indented(separated1(
            preceded(indent, statement),
            TokenType::Eol.followed_by(empty_lines),
        )),
    )
    .or(statement.map(|statement| vec![statement]))
    .map(|statements| Block { statements })
    .process(stream)
}
