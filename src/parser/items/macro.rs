use crate::{
    ast::{Ident, MacroDecl, MacroEntry, MacroInvoc},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{consume_tokens_until, expect_token, parse_vec_of, ParseError},
    },
};

impl Parsable for MacroDecl {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("macro".to_string()))?;

        let (name, mut remaining_tokens) = Ident::parse(remaining_tokens, parse_ctx)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        let (entries, remaining_tokens) =
            parse_vec_of::<MacroEntry>(remaining_tokens, None, parse_ctx)?;

        Ok((MacroDecl { name, entries }, remaining_tokens))
    }
}

impl Parsable for MacroEntry {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let mut remaining_tokens = tokens;

        remaining_tokens = expect_token(remaining_tokens, TokenType::Indent(2))?;

        let (defs, mut remaining_tokens) =
            consume_tokens_until(remaining_tokens, TokenType::FatArrow);

        remaining_tokens = expect_token(remaining_tokens, TokenType::FatArrow)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Indent(4))?;

        let (block, remaining_tokens) = consume_tokens_until(remaining_tokens, TokenType::Eol);

        Ok((MacroEntry { defs, block }, remaining_tokens))
    }
}

impl Parsable for MacroInvoc {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        if let Some(token) = tokens.get(0) {
            if let TokenType::MacroInvoc(ident) = &token.token_type {
                let (args, remaining_tokens) = consume_tokens_until(&tokens[1..], TokenType::Eol);

                return Ok((
                    MacroInvoc {
                        name: Ident {
                            name: ident.clone(),
                            span: token.span.clone(),
                        },
                        args,
                    },
                    remaining_tokens,
                ));
            }
        }

        Err(ParseError::UnexpectedEof)
    }
}
