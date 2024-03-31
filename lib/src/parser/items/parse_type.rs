use crate::{
    ast::{ParseType, ParseTypeInner},
    diagnostic::Diagnostics,
    lexer::{Span, Token, TokenType},
    parser::{
        parse_ctx::ParseCtx,
        util::{expect_token, parse_vec_of, ParseError},
        Parsable,
    },
};

// Example of ParseType
// T
// T -> U
// T A, B -> U
// T (A -> B), C -> U

impl Parsable for ParseType {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = tokens;

        let token = remaining_tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Type("".to_string())))?;

        match token.token_type {
            TokenType::Type(_) | TokenType::OpenParen | TokenType::OpenBracket => {}
            _ => {
                return Err(ParseError::UnexpectedToken(
                    token.clone(),
                    vec![
                        TokenType::OpenParen,
                        TokenType::OpenBracket,
                        TokenType::Type("".to_string()),
                    ],
                )
                .into())
            }
        }

        if let Ok((func, remaining_tokens)) = parse_function_type(remaining_tokens, parse_ctx) {
            return Ok((func, remaining_tokens));
        } else if let Ok((array, remaining_tokens)) = parse_array_type(remaining_tokens, parse_ctx)
        {
            return Ok((array, remaining_tokens));
        } else if let Ok((tuple, remaining_tokens)) = parse_tuple_type(remaining_tokens, parse_ctx)
        {
            return Ok((tuple, remaining_tokens));
        } else if let Ok((inner, remaining_tokens)) =
            ParseTypeInner::parse(remaining_tokens, parse_ctx)
        {
            return Ok((ParseType::Type(inner), remaining_tokens));
        } else {
            Err(ParseError::UnexpectedToken(
                tokens[0].clone(),
                vec![
                    TokenType::OpenParen,
                    TokenType::OpenBracket,
                    TokenType::Type("".to_string()),
                ],
            )
            .into())
        }
    }
}

fn parse_function_type<'a>(
    tokens: &'a [Token],
    parse_ctx: &mut ParseCtx,
) -> Result<(ParseType, &'a [Token]), Diagnostics> {
    let mut remaining_tokens = tokens;
    let mut has_toggled_inside_fn_type_decl = false;

    if parse_ctx.is_inside_fn_type_decl {
        remaining_tokens = expect_token(remaining_tokens, TokenType::OpenParen)?;
    }

    if !parse_ctx.is_inside_fn_type_decl {
        has_toggled_inside_fn_type_decl = true;
        parse_ctx.is_inside_fn_type_decl = true;
    }

    let (list, mut remaining_tokens) =
        parse_vec_of(remaining_tokens, Some(TokenType::Arrow), parse_ctx)?;

    if has_toggled_inside_fn_type_decl {
        parse_ctx.is_inside_fn_type_decl = false;
    }

    if list.len() < 2 {
        return Err(ParseError::UnexpectedToken(
            remaining_tokens
                .get(0)
                .unwrap_or(&Token {
                    token_type: TokenType::Eof,
                    span: Span::default(),
                })
                .clone(),
            vec![TokenType::Arrow],
        )
        .into());
    }

    if parse_ctx.is_inside_fn_type_decl {
        remaining_tokens = expect_token(remaining_tokens, TokenType::CloseParen)?;
    }

    Ok((ParseType::Function(list), remaining_tokens))
}

fn parse_array_type<'a>(
    tokens: &'a [Token],
    parse_ctx: &mut ParseCtx,
) -> Result<(ParseType, &'a [Token]), Diagnostics> {
    let mut remaining_tokens = tokens;
    remaining_tokens = expect_token(remaining_tokens, TokenType::OpenBracket)?;

    let (ty, mut remaining_tokens) = ParseType::parse(remaining_tokens, parse_ctx)?;

    remaining_tokens = expect_token(remaining_tokens, TokenType::CloseBracket)?;

    Ok((ParseType::Array(Box::new(ty)), remaining_tokens))
}

fn parse_tuple_type<'a>(
    tokens: &'a [Token],
    parse_ctx: &mut ParseCtx,
) -> Result<(ParseType, &'a [Token]), Diagnostics> {
    let mut remaining_tokens = tokens;
    remaining_tokens = expect_token(remaining_tokens, TokenType::OpenParen)?;

    let (list, mut remaining_tokens) =
        parse_vec_of(remaining_tokens, Some(TokenType::Coma), parse_ctx)?;

    remaining_tokens = expect_token(remaining_tokens, TokenType::CloseParen)?;

    if list.is_empty() {
        return Ok((ParseType::Unit, remaining_tokens));
    }

    if list.len() == 1 {
        return Err(ParseError::UnexpectedToken(
            remaining_tokens[0].clone(),
            vec![TokenType::Coma, TokenType::CloseParen],
        )
        .into());
    }

    Ok((ParseType::Tuple(list), remaining_tokens))
}

impl Parsable for ParseTypeInner {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let mut remaining_tokens = tokens;
        let mut has_paren = false;
        if TokenType::OpenParen == remaining_tokens[0].token_type {
            remaining_tokens = &remaining_tokens[1..];
            has_paren = true;
        }

        let token = remaining_tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Type("".to_string())))?;

        let ((name, span), remaining_tokens) = match &token.token_type {
            TokenType::Type(name) => ((name.clone(), token.span.clone()), &remaining_tokens[1..]),
            _ => {
                return Err(ParseError::UnexpectedToken(
                    token.clone(),
                    vec![TokenType::Ident("".to_string())],
                )
                .into())
            }
        };

        let (generics, mut remaining_tokens) =
            parse_vec_of(&remaining_tokens, Some(TokenType::Coma), parse_ctx)?;

        if has_paren {
            remaining_tokens = expect_token(remaining_tokens, TokenType::CloseParen)?;
        }

        let parse_type = ParseTypeInner {
            name,
            span,
            generics,
        };

        Ok((parse_type, remaining_tokens))
    }
}

#[cfg(test)]
mod parse_type {
    use crate::{parser::util::lex_test, Config};

    use super::*;

    #[test]
    fn test_parse_type() {
        let input = "Type";
        let tokens = lex_test(input);
        let (parse_type, rest) =
            ParseType::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(parse_type.to_string(), "Type");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_type_with_generics() {
        let input = "Type Generics";
        let tokens = lex_test(input);
        let (parse_type, rest) =
            ParseType::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(parse_type.to_string(), "Type Generics");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_type_with_multiple_generics() {
        let input = "Type Generics, Generics2";
        let tokens = lex_test(input);
        let (parse_type, rest) =
            ParseType::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(parse_type.to_string(), "Type Generics, Generics2");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_fn_type() {
        let input = "A -> B";
        let tokens = lex_test(input);
        let (parse_type, rest) =
            ParseType::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(parse_type.to_string(), "A -> B");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_nested_fn_type() {
        let input = "(A -> B) -> C";
        let tokens = lex_test(input);
        let (parse_type, rest) =
            ParseType::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(parse_type.to_string(), "(A -> B) -> C");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_tuple_type() {
        let input = "(A, B)";
        let tokens = lex_test(input);
        let (parse_type, rest) =
            ParseType::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(parse_type.to_string(), "(A, B)");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_unit_type() {
        let input = "()";
        let tokens = lex_test(input);
        let (parse_type, rest) =
            ParseType::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(parse_type.to_string(), "()");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_array_type() {
        let input = "[A]";
        let tokens = lex_test(input);
        let (parse_type, rest) =
            ParseType::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(parse_type.to_string(), "[A]");
        assert_eq!(rest.len(), 0);
    }
}
