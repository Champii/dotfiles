use crate::new_parser::*;

pub fn ident(stream: Input) -> IResult<Ident> {
    let (stream, token) = stream.consume()?;

    if let TokenType::Ident(name) = &token.token_type {
        Ok((
            stream,
            Ident {
                name: name.clone(),
                span: token.span.clone(),
            },
        ))
    } else {
        Err(ParseError::ExpectedIdent(token.clone()))
    }
}
