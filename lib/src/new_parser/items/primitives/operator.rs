use crate::new_parser::{engine::*, ParseError, TokenType};

pub fn operator(stream: Input) -> IResult<String> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Operator(name) = &token.token_type {
        Ok((stream, name.clone()))
    } else {
        Err(ParseError::ExpectedOperator(token.clone()))
    }
}
