use crate::{
    lexer::TokenType,
    new_parser::{engine::*, Block},
};

use super::statement;

pub fn block(stream: Input) -> IResult<Block> {
    many((statement, TokenType::Eol))
        .map(|statements| Block {
            statements: statements.into_iter().map(|(stmt, _)| stmt).collect(),
        })
        .process(stream)
}
