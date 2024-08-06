use crate::new_parser::{engine::*, TokenType};

pub fn macro_invoc_token(stream: Input) -> IResult<String> {
    let (stream, token) = stream.consume()?;

    if let TokenType::MacroInvoc(name) = token.token_type {
        Ok((stream, name))
    } else {
        Err(ParseError::UnexpectedToken(token).into())
    }
}
