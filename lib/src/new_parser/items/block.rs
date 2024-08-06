use crate::{
    lexer::TokenType,
    new_parser::{engine::*, Block},
};

use super::statement;

pub fn block(stream: Input) -> IResult<Block> {
    delimited(statement, TokenType::Eol)
        .map(|statements| Block { statements })
        .process(stream)
}
