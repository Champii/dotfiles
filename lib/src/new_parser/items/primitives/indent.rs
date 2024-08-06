use crate::{lexer::TokenType, new_parser::engine::*};

pub fn indent(stream: Input) -> IResult<()> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Indent(level) = &token.token_type {
        if *level as usize != stream.indent_level {
            return Err(ParseError::UnexpectedIndent(*level));
        }

        Ok((stream, ()))
    } else {
        Err(ParseError::UnexpectedToken(token.clone()))
    }
}
