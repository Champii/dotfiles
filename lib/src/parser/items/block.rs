use crate::{
    ast::{Block, Statement},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, ParseError},
    },
};

fn parse_statements_loop<'a>(
    tokens: &'a [Token],
    parse_ctx: &mut ParseCtx,
) -> Result<(Vec<Statement>, &'a [Token]), Diagnostics> {
    let mut remaining_tokens = &tokens[1..];
    let mut remaining_tokens_with_leading_newlines = remaining_tokens;
    let mut statements = vec![];
    let mut nb_statements_without_leading_newlines = 0;

    loop {
        if let Ok((statement, new_tokens)) = Statement::parse(remaining_tokens, parse_ctx) {
            remaining_tokens = new_tokens;
            statements.push(statement);
        } else {
            remaining_tokens = remaining_tokens_with_leading_newlines;
            statements = statements
                .into_iter()
                .take(nb_statements_without_leading_newlines)
                .collect();
            break;
        }

        remaining_tokens_with_leading_newlines = remaining_tokens;

        if let Ok(new_tokens) = expect_token(remaining_tokens, TokenType::Eol) {
            remaining_tokens = new_tokens;
        } else {
            break;
        }

        nb_statements_without_leading_newlines = statements.len();

        while remaining_tokens
            .get(0)
            .map(|t| {
                if let TokenType::Indent(_) = t.token_type {
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false)
            && remaining_tokens
                .get(1)
                .map(|t| t.token_type == TokenType::Eol)
                .unwrap_or(false)
        {
            remaining_tokens = &remaining_tokens[2..];
            statements.push(Statement::EmptyLine);
        }
    }

    Ok((statements, remaining_tokens))
}

impl Parsable for Block {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        if tokens.len() == 0 {
            return Err(ParseError::UnexpectedEof(TokenType::Eol).into());
        }
        let (statements, new_tokens) = if tokens[0].token_type == TokenType::Eol {
            parse_ctx.indent_block(|parse_ctx| parse_statements_loop(&tokens[1..], parse_ctx))?
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
    use crate::{parser::util::lex_test, Config};

    #[test]
    fn test_parse_block() {
        let input = "statement";
        let tokens = lex_test(input);
        let (block, rest) = Block::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(block.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_block_with_indent() {
        let input = "\n  statement\n  statement";
        let tokens = lex_test(input);
        let (block, rest) = Block::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(block.statements.len(), 2);
        assert_eq!(rest.len(), 0);
    }
}
