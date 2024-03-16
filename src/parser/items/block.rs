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
                match parse_vec_of::<Statement>(&tokens[1..], Some(TokenType::Eol), parse_ctx) {
                    Ok((statements, new_tokens)) => (statements, new_tokens),
                    Err(e) => {
                        parse_ctx.indent_level -= 2;
                        return Err(e);
                    }
                };

            parse_ctx.indent_level -= 2;
            (statements, new_tokens)
        } else {
            let (statement, remaining_tokens) = Statement::parse(&tokens, parse_ctx)?;

            (vec![statement], remaining_tokens)
        };

        Ok((Block { statements }, new_tokens))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser::util::lex_test;

    #[test]
    fn test_parse_block() {
        let input = "statement";
        let tokens = lex_test(input);
        let (block, rest) = Block::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(block.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_block_with_indent() {
        let input = "\n  statement\n  statement";
        let tokens = lex_test(input);
        let (block, rest) = Block::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(block.statements.len(), 2);
        assert_eq!(rest.len(), 0);
    }
}
