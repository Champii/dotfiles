use crate::new_parser::{engine::*, TokenType};

pub fn type_token(stream: Input) -> IResult<String> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Type(name) = &token.token_type {
        Ok((stream, name.clone()))
    } else {
        Err(ParseError::ExpectedType(token.clone()))
    }
}
