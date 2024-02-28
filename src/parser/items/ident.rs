use crate::{
    ast::Ident,
    lexer::{Token, TokenType},
    parser::{parsable::Parsable, util::ParseError},
};

impl Parsable for Ident {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;

        match &token.token_type {
            TokenType::Ident(name) => Ok((
                Ident {
                    name: name.clone(),
                    span: token.span.clone(),
                },
                &tokens[1..],
            )),
            _ => Err(ParseError::UnexpectedToken(token.clone())),
        }
    }
}
