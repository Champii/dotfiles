use crate::{lexer::TokenType, new_parser::engine::*};

pub fn boolean(stream: Input) -> IResult<bool> {
    TokenType::Keyword("true".to_string())
        .map(|_| true)
        .or(TokenType::Keyword("false".to_string()).map(|_| false))
        .process(stream)
}

pub fn int(stream: Input) -> IResult<u64> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Number(value) = &token.token_type {
        Ok((stream, value.parse().unwrap()))
    } else {
        Err(ParseError::ExpectedNumber(token.clone()))
    }
}

pub fn float(stream: Input) -> IResult<f64> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Float(value) = &token.token_type {
        Ok((stream, value.parse().unwrap()))
    } else {
        Err(ParseError::UnexpectedToken(token.clone()))
    }
}

pub fn string(stream: Input) -> IResult<String> {
    let (stream, token) = stream.consume()?;

    if let TokenType::String(value) = &token.token_type {
        Ok((stream, value.clone()))
    } else {
        Err(ParseError::UnexpectedToken(token.clone()))
    }
}

pub fn char(stream: Input) -> IResult<String> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Char(value) = &token.token_type {
        Ok((stream, value.clone()))
    } else {
        Err(ParseError::UnexpectedToken(token.clone()))
    }
}
