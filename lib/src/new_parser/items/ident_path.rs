use crate::new_parser::{delimited1, engine::*, IdentOrType, IdentifierPath, TokenType};

use super::{ident, parse_type};

pub fn ident_path(stream: Input) -> IResult<IdentifierPath> {
    delimited1(ident_or_type, TokenType::DoubleColon)
        .map(|idents| IdentifierPath { path: idents })
        .process(stream)
}

pub fn ident_or_type(stream: Input) -> IResult<IdentOrType> {
    ident
        .map(IdentOrType::Ident)
        .or(parse_type.map(IdentOrType::Type))
        .process(stream)
}
