use crate::{
    ast::{Block, FunctionDecl, Ident},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        util::{expect_token, parse_vec_of, ParseError},
    },
};

impl Parsable for FunctionDecl {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let (name, mut remaining_tokens) = Ident::parse(tokens)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Equal)?;

        let (parameters, mut remaining_tokens) =
            parse_vec_of::<Ident>(remaining_tokens, Some(TokenType::Coma))?;

        remaining_tokens = expect_token(remaining_tokens, TokenType::Arrow)?;

        let (body, remaining_tokens) = Block::parse(remaining_tokens)?;

        Ok((
            FunctionDecl {
                name,
                parameters,
                body,
            },
            remaining_tokens,
        ))
    }
}
