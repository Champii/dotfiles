use crate::{
    ast::{Block, Else, Expression, If},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, ParseError},
    },
};

impl Parsable for If {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("if".to_string()))?;

        let (condition, mut remaining_tokens) = Expression::parse(remaining_tokens, parse_ctx)?;

        if TokenType::Eol == remaining_tokens[0].token_type {
            if TokenType::Indent(parse_ctx.indent_level) == remaining_tokens[1].token_type {
                remaining_tokens = &remaining_tokens[2..];
            }
        }

        if let Ok(new_remaining_tokens) =
            expect_token(remaining_tokens, TokenType::Keyword("then".to_string()))
        {
            remaining_tokens = new_remaining_tokens;
        }

        let (then, mut remaining_tokens) = Block::parse(remaining_tokens, parse_ctx)?;

        if remaining_tokens.is_empty() {
            return Ok((
                If {
                    condition,
                    then,
                    else_: None,
                },
                remaining_tokens,
            ));
        }

        if TokenType::Eol == remaining_tokens[0].token_type {
            if TokenType::Indent(parse_ctx.indent_level) == remaining_tokens[1].token_type {
                remaining_tokens = &remaining_tokens[2..];
            }
        }

        let (else_, remaining_tokens) =
            if TokenType::Keyword("else".to_string()) == remaining_tokens[0].token_type {
                let (else_block, remaining_tokens) = Else::parse(remaining_tokens, parse_ctx)?;
                (Some(else_block), remaining_tokens)
            } else {
                (None, remaining_tokens)
            };

        Ok((
            If {
                condition,
                then,
                else_,
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
        let (_if_, rest) = If::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_if_else_monoline() {
        let input = "if true then 1 else z";
        let tokens = lex_test(input);
        let (_if_, rest) = If::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_if_else_if_else_monoline() {
        let input = "if true then 1 else if false then 2 else 3";
        let tokens = lex_test(input);
        let (_if_, rest) = If::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_if_else_multiline_1() {
        let input = "if true\nthen 1\nelse 2";
        let tokens = lex_test(input);
        let (_if_, rest) = If::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_if_else_multiline_2() {
        let input = "if true then\n  1\nelse if false then\n  2\nelse 3";
        let tokens = lex_test(input);
        let (_if_, rest) = If::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_if_else_multiline_3() {
        let input = "if true\n  1\nelse\n  2";
        let tokens = lex_test(input);
        let (_if_, rest) = If::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(rest.len(), 0);
    }
}
