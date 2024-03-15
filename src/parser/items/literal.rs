use crate::{
    ast::{Literal, LiteralKind},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{consume_tokens_until, expect_token, ParseError},
    },
};

impl Parsable for Literal {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let mut tokens = tokens;

        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Number("".to_string())))?;

        let literal_kind = match &token.token_type {
            TokenType::Keyword(value) if value == "true" || value == "false" => {
                tokens = &tokens[1..];
                LiteralKind::Bool(value.parse().unwrap())
            }
            TokenType::Number(value) => {
                tokens = &tokens[1..];
                LiteralKind::Number(value.parse().unwrap())
            }
            TokenType::DoubleQuote => {
                let (string, remaining_tokens) = String::parse(tokens, parse_ctx)?;
                tokens = remaining_tokens;
                LiteralKind::String(string)
            }
            TokenType::SimpleQuote => {
                let (char, remaining_tokens) = char::parse(tokens, parse_ctx)?;
                tokens = remaining_tokens;
                LiteralKind::Char(char)
            }
            // TokenType::Float(value) => LiteralKind::Float(value.parse().unwrap()),
            // TokenType::Array(_) => {
            // let (array, remaining_tokens) = Array::parse(tokens, parse_ctx)?;
            // LiteralKind::Array(array)
            // }
            _ => {
                return Err(ParseError::UnexpectedToken(
                    token.clone(),
                    vec![TokenType::Number("".to_string())],
                ))
            }
        };

        Ok((
            Literal {
                kind: literal_kind,
                span: token.span.clone(),
            },
            &tokens,
        ))
    }
}

impl Parsable for String {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let remaining_tokens = expect_token(tokens, TokenType::DoubleQuote)?;

        let (inner_tokens, remaining_tokens) =
            consume_tokens_until(remaining_tokens, TokenType::DoubleQuote);

        let remaining_tokens = expect_token(remaining_tokens, TokenType::DoubleQuote)?;

        let string = inner_tokens
            .iter()
            .map(|token| token.token_type.to_string())
            .collect();

        Ok((string, remaining_tokens))
    }
}

impl Parsable for char {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let mut remaining_tokens = expect_token(tokens, TokenType::SimpleQuote)?;

        let inner_token = remaining_tokens[0].clone();

        if format!("{}", inner_token.token_type.to_string()).len() > 1 {
            return Err(ParseError::UnexpectedToken(
                inner_token,
                vec![TokenType::SimpleQuote],
            ));
        }
        remaining_tokens = &remaining_tokens[1..];

        let remaining_tokens = expect_token(remaining_tokens, TokenType::SimpleQuote)?;

        Ok((
            inner_token.token_type.to_string().chars().next().unwrap(),
            remaining_tokens,
        ))
    }
}

#[cfg(test)]
mod literals {

    use super::*;
    use crate::parser::util::lex_test;

    fn parse_literal(input: &str) -> Literal {
        let tokens = lex_test(input);
        let (literal, remaining_tokens) = Literal::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(remaining_tokens.len(), 0);

        literal
    }

    #[test]
    fn test_parse_number() {
        let input = "123";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::Number(123));
    }

    #[test]
    fn test_parse_string() {
        let input = "\"hello\"";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::String("hello".to_string()));
    }

    #[test]
    fn test_parse_char() {
        let input = "'a'";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::Char('a'));
    }
}
