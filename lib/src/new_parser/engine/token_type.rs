use crate::lexer::TokenType;

use super::{parse_error::ParseError, parser_trait::Parser, IResult, Input};

impl Parser for TokenType {
    type Output = Self;

    fn process<'a>(&mut self, stream: Input<'a>) -> IResult<'a, Self> {
        let (stream, token) = stream.consume()?;

        if token.token_type == *self {
            Ok((stream, self.clone()))
        } else {
            Err(ParseError::UnexpectedToken(self.to_string(), token))
        }
    }
}
