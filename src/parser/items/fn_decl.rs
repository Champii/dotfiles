use crate::{
    ast::{Block, FunctionDecl, Ident},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, parse_vec_of, ParseError},
    },
};

impl Parsable for FunctionDecl {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let (name, mut remaining_tokens) = Ident::parse(tokens, parse_ctx)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Equal)?;

        let (parameters, mut remaining_tokens) =
            parse_vec_of::<Ident>(remaining_tokens, Some(TokenType::Coma), parse_ctx)?;

        remaining_tokens = expect_token(remaining_tokens, TokenType::Arrow)?;

        let (body, remaining_tokens) = Block::parse(remaining_tokens, parse_ctx)?;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

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
