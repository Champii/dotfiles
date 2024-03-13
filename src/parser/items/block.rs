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
                parse_vec_of::<Statement>(&tokens[1..], Some(TokenType::Eol), parse_ctx)?;
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
    use std::path::PathBuf;

    use super::*;
    use crate::lexer::Lexer;

    fn lex(input: &str) -> Vec<Token> {
        Lexer::new(PathBuf::new(), input)
            .unwrap()
            .with_newline_at_end(false)
            .collect()
            .unwrap()
    }

    #[test]
    fn test_parse_block() {
        let input = "statement";
        let tokens = lex(input);
        let tokens = &tokens[1..]; // skip the Indent(0)
        let (block, rest) = Block::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(block.statements.len(), 1);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0].token_type, TokenType::Eof);
    }

    #[test]
    fn test_parse_block_with_indent() {
        let input = "\n  statement\n  statement";
        let tokens = lex(input);
        let tokens = &tokens[1..]; // skip the Indent(0)
        let (block, rest) = Block::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(block.statements.len(), 2);
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0].token_type, TokenType::Eof);
    }
}
