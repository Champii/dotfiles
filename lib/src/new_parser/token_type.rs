use crate::lexer::TokenType;

use super::{parse_error::ParseError, parser_trait::Parser, IResult, Input, Token};

impl Parser for TokenType {
    type Output = Self;

    fn process<'a, 'b>(&'b mut self, tokens: Input<'a>) -> IResult<'a, Self> {
        if tokens.is_empty() {
            return Err(ParseError::UnexpectedEOF);
        }

        if &tokens[0].token_type == self {
            Ok((&tokens[1..], self.clone()))
        } else {
            Err(ParseError::UnexpectedToken(tokens[0].clone()))
        }
    }
}
