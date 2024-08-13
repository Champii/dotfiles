use crate::new_parser::{engine::*, separated1, IdentOrType, IdentifierPath, TokenType, TypePath};

use super::{ident, parse_type};

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

    return Err(ParseError::Fail);
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

    return Err(ParseError::Fail);
}

pub fn ident_or_type(stream: Input) -> IResult<IdentOrType> {
    ident
        .map(IdentOrType::Ident)
        .or(parse_type.map(IdentOrType::Type))
        .process(stream)
}

#[cfg(test)]
mod tests {

    use crate::{new_parser::lex_test, Config};

    use super::*;

    #[test]
    fn test_ident_path() {
        let input = "ident::Type::ident";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, ident) = ident_path
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(ident.path.len(), 3);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_type_path() {
        let input = "ident::Type::Type";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, ident) = type_path.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(ident.path.len(), 3);
        assert_eq!(rest.len(), 0);
    }
}
