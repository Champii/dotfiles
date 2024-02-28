use crate::{
    ast::{Block, Statement},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{parse_vec_of, ParseError},
    },
};

impl Parsable for Block {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let (statements, new_tokens) = if tokens[0].token_type == TokenType::Eol {
            parse_ctx.indent_level += 2;
            let (statements, new_tokens) =
                parse_vec_of::<Statement>(&tokens[1..], None, parse_ctx)?;
            parse_ctx.indent_level -= 2;

            (statements, new_tokens)
        } else {
            let (statement, remaining_tokens) = Statement::parse(&tokens, parse_ctx)?;

            (vec![statement], remaining_tokens)
        };

        Ok((Block { statements }, new_tokens))
    }
}
