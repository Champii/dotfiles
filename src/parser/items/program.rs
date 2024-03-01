use crate::{
    ast::{Program, TopLevel},
    lexer::{Token, TokenType},
    parser::{
        parse_ctx::ParseCtx,
        util::{expect_token, ParseError},
        Parsable,
    },
};

impl Parsable for Program {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let mut statements = Vec::new();
        let mut tokens = tokens;

        loop {
            if tokens.is_empty() || tokens[0].token_type == TokenType::Eof {
                break;
            }

            while tokens
                .get(0)
                .map(|t| t.token_type == TokenType::Eol)
                .unwrap_or(false)
            {
                tokens = &tokens[1..];
            }

            let (statement, new_tokens) = TopLevel::parse(tokens, parse_ctx)?;

            tokens = new_tokens;

            statements.push(statement);
        }

        let remaining_tokens = expect_token(tokens, TokenType::Eof)?;

        if !remaining_tokens.is_empty() {
            return Err(ParseError::LeftoverTokens(remaining_tokens.to_vec()));
        }

        Ok((
            Program {
                top_levels: statements,
            },
            tokens,
        ))
    }
}
