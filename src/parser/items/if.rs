use crate::{
    ast::{Block, Else, Expression, Ident, IdentifierPath, If},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, parse_vec_of, ParseError},
    },
};

impl Parsable for If {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("if".to_string()))?;

        let (condition, remaining_tokens) = Expression::parse(remaining_tokens, parse_ctx)?;

        let remaining_tokens =
            expect_token(remaining_tokens, TokenType::Keyword("then".to_string()))?;

        let (then_block, remaining_tokens) = Block::parse(remaining_tokens, parse_ctx)?;

        if remaining_tokens.is_empty() {
            return Ok((
                If {
                    condition,
                    then: then_block,
                    else_: None,
                },
                remaining_tokens,
            ));
        }

        if let TokenType::Keyword(keyword) = &remaining_tokens[0].token_type {
            if keyword == "else" {
                let (else_block, remaining_tokens) = Else::parse(remaining_tokens, parse_ctx)?;

                return Ok((
                    If {
                        condition,
                        then: then_block,
                        else_: Some(else_block),
                    },
                    remaining_tokens,
                ));
            }
        }
        Ok((
            If {
                condition,
                then: then_block,
                else_: None,
            },
            remaining_tokens,
        ))
    }
}

impl Parsable for Else {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("else".to_string()))?;

        if let TokenType::Keyword(keyword) = &remaining_tokens[0].token_type {
            if keyword == "if" {
                let (if_, remaining_tokens) = If::parse(remaining_tokens, parse_ctx)?;
                Ok((Else::If(Box::new(if_)), remaining_tokens))
            } else {
                let (block, remaining_tokens) = Block::parse(remaining_tokens, parse_ctx)?;
                Ok((Else::Block(block), remaining_tokens))
            }
        } else {
            let (block, remaining_tokens) = Block::parse(remaining_tokens, parse_ctx)?;
            Ok((Else::Block(block), remaining_tokens))
        }
    }
}

#[cfg(test)]
mod test_if {
    use super::*;
    use crate::parser::util::lex_test;

    #[test]
    fn test_parse_if_monoline() {
        let input = "if a then 1";
        let tokens = lex_test(input);
        let (if_, rest) = If::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_if_else_monoline() {
        let input = "if true then 1 else z";
        let tokens = lex_test(input);
        let (if_, rest) = If::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_if_else_if_else_monoline() {
        let input = "if true then 1 else if false then 2 else 3";
        let tokens = lex_test(input);
        let (if_, rest) = If::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(rest.len(), 0);
    }
}
