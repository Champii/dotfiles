use crate::new_parser::{
    engine::*, separated1, IdentOrType, IdentifierPath, Path, TokenType, TypePath,
};

use super::{ident, parse_type};

pub fn path(stream: Input) -> IResult<Path> {
    ident_path
        .map(Path::Ident)
        .or(type_path.map(Path::Type))
        .process(stream)
}

pub fn ident_path(stream: Input) -> IResult<IdentifierPath> {
    let (stream, path) = separated1(ident_or_type, TokenType::DoubleColon)
        .map(|idents| IdentifierPath { path: idents })
        .process(stream)?;

    if let Some(last) = path.path.last() {
        if let IdentOrType::Ident(_) = last {
            return Ok((stream, path));
        } else {
            return Err(ParseError::Fail);
        }
    }

    Err(ParseError::Fail)
}

// the same, but finished with a type
pub fn type_path(stream: Input) -> IResult<TypePath> {
    let (stream, path) = separated1(ident_or_type, TokenType::DoubleColon)
        .map(|idents| TypePath { path: idents })
        .process(stream)?;

    if let Some(last) = path.path.last() {
        if let IdentOrType::Type(_) = last {
            return Ok((stream, path));
        } else {
            return Err(ParseError::Fail);
        }
    }

    Err(ParseError::Fail)
}

pub fn ident_or_type(stream: Input) -> IResult<IdentOrType> {
    ident
        .map(IdentOrType::Ident)
        .or(parse_type.map(IdentOrType::Type))
        .process(stream)
}
