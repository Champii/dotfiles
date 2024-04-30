use crate::{
    ast::{Block, FunctionDecl, FunctionSig, Ident, LambdaDecl, ParseType, Pattern},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{consume_tokens_until, expect_token, parse_vec_of, ParseError},
    },
};

impl Parsable for FunctionDecl {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let mut remaining_tokens = tokens;
        let mut inject_self = false;

        if let TokenType::Arobase = remaining_tokens[0].token_type {
            remaining_tokens = &remaining_tokens[1..];
            inject_self = true;
        }

        let (name, mut remaining_tokens) = Ident::parse(remaining_tokens, parse_ctx)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Equal)?;

        let (lambda, remaining_tokens) = LambdaDecl::parse(remaining_tokens, parse_ctx)?;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        Ok((
            FunctionDecl {
                name,
                lambda,
                inject_self,
            },
            remaining_tokens,
        ))
    }
}

impl Parsable for LambdaDecl {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = tokens;

        if remaining_tokens[0].token_type == TokenType::OpenParen {
            if let Ok((lambda, remaining_tokens)) =
                parse_function_shorthand(remaining_tokens, parse_ctx)
            {
                return Ok((lambda, remaining_tokens));
            }
        }

        let (parameters, mut remaining_tokens, _diags) =
            parse_vec_of::<Pattern>(remaining_tokens, Some(TokenType::Coma), parse_ctx)?;

        // consume token if it's a coma
        if !remaining_tokens.is_empty() && remaining_tokens[0].token_type == TokenType::Coma {
            remaining_tokens = &remaining_tokens[1..];
        }

        remaining_tokens = expect_token(remaining_tokens, TokenType::Arrow)?;

        let (body, remaining_tokens) = Block::parse(remaining_tokens, parse_ctx)?;

        Ok((
            LambdaDecl {
                parameters,
                body,
                shorthand_tokens: None,
            },
            remaining_tokens,
        ))
    }
}

/// Function shorthands
/// (+2) : x -> x + 2
/// (a/) : x -> a / x
/// (.prop) : x -> x.prop

fn parse_function_shorthand<'a>(
    tokens: &'a [Token],
    parse_ctx: &mut ParseCtx,
) -> Result<(LambdaDecl, &'a [Token]), Diagnostics> {
    let remaining_tokens = tokens;

    let remaining_tokens = expect_token(remaining_tokens, TokenType::OpenParen)?;

    // special case of the dot
    let (mut lambda, remaining_tokens) = match remaining_tokens[0].token_type {
        TokenType::Operator(_) | TokenType::StuckOperator(_) | TokenType::Dot => {
            expand_shorthand_prefix_argument(remaining_tokens, parse_ctx)?
        }
        _ => expand_shorthand_suffix_argument(remaining_tokens, parse_ctx)?,
    };

    lambda.shorthand_tokens = Some(tokens[..tokens.len() - remaining_tokens.len()].to_vec());

    Ok((lambda, remaining_tokens))
}

fn expand_shorthand_prefix_argument<'a>(
    tokens: &'a [Token],
    parse_ctx: &mut ParseCtx,
) -> Result<(LambdaDecl, &'a [Token]), Diagnostics> {
    let remaining_tokens = tokens;

    let span = remaining_tokens[0].span.clone();

    let (inner_tokens, remaining_tokens) =
        consume_tokens_until(remaining_tokens, TokenType::CloseParen);

    let inner_tokens = vec![
        Token {
            token_type: TokenType::Ident("x".to_string()),
            span: span.clone(),
        },
        Token {
            token_type: TokenType::Arrow,
            span: span.clone(),
        },
        Token {
            token_type: TokenType::Ident("x".to_string()),
            span: span.clone(),
        },
    ]
    .into_iter()
    .chain(inner_tokens.into_iter().map(|token| {
        if let TokenType::StuckOperator(op) = token.token_type {
            Token {
                token_type: TokenType::Operator(op),
                span: token.span.clone(),
            }
        } else {
            token
        }
    }))
    .collect::<Vec<_>>();

    let (lambda, other_remaining_tokens) = LambdaDecl::parse(&inner_tokens, parse_ctx)?;
    if !other_remaining_tokens.is_empty() {
        return Err(ParseError::UnexpectedToken(
            other_remaining_tokens[0].clone(),
            vec![TokenType::CloseParen],
        )
        .into());
    }
    let remaining_tokens = expect_token(remaining_tokens, TokenType::CloseParen)?;

    Ok((lambda, remaining_tokens))
}

fn expand_shorthand_suffix_argument<'a>(
    tokens: &'a [Token],
    parse_ctx: &mut ParseCtx,
) -> Result<(LambdaDecl, &'a [Token]), Diagnostics> {
    let remaining_tokens = tokens;

    let span = remaining_tokens[0].span.clone();

    let (inner_tokens, remaining_tokens) =
        consume_tokens_until(remaining_tokens, TokenType::CloseParen);

    let operator = inner_tokens[inner_tokens.len() - 1].clone();

    if let TokenType::Operator(_) | TokenType::StuckOperator(_) = operator.token_type {
    } else {
        return Err(ParseError::UnexpectedToken(
            operator.clone(),
            vec![TokenType::Operator("".to_string())],
        )
        .into());
    }

    let inner_tokens = inner_tokens[..inner_tokens.len() - 1].to_vec();

    let inner_tokens = vec![
        Token {
            token_type: TokenType::Ident("x".to_string()),
            span: span.clone(),
        },
        Token {
            token_type: TokenType::Arrow,
            span: span.clone(),
        },
    ]
    .into_iter()
    .chain(inner_tokens.into_iter().map(|token| {
        if let TokenType::StuckOperator(op) = token.token_type {
            Token {
                token_type: TokenType::Operator(op),
                span: token.span.clone(),
            }
        } else {
            token
        }
    }))
    .chain(vec![
        operator,
        Token {
            token_type: TokenType::Ident("x".to_string()),
            span: span.clone(),
        },
    ])
    .collect::<Vec<_>>();

    let (lambda, other_remaining_tokens) = LambdaDecl::parse(&inner_tokens, parse_ctx)?;
    if !other_remaining_tokens.is_empty() {
        return Err(ParseError::UnexpectedToken(
            other_remaining_tokens[0].clone(),
            vec![TokenType::CloseParen],
        )
        .into());
    }
    let remaining_tokens = expect_token(remaining_tokens, TokenType::CloseParen)?;

    Ok((lambda, remaining_tokens))
}

impl<'a> Parsable for FunctionSig {
    fn parse<'b>(
        tokens: &'b [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'b [Token]), Diagnostics> {
        let remaining_tokens = tokens;

        let (name, remaining_tokens) = Ident::parse(remaining_tokens, parse_ctx)?;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Colon)?;

        let (sig, remaining_tokens) = ParseType::parse(remaining_tokens, parse_ctx)?;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        Ok((FunctionSig { name, sig }, remaining_tokens))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parser::util::lex_test, Config};

    #[test]
    fn test_parse_function_decl_monoline() {
        let input = "myfn = -> statement\n";
        let tokens = lex_test(input);
        let (function_decl, rest) =
            FunctionDecl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(function_decl.name.name, "myfn");
        assert_eq!(function_decl.lambda.parameters.len(), 0);
        assert_eq!(function_decl.lambda.body.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_function_decl() {
        let input = "myfn = a, b, c ->\n  statement\n";
        let tokens = lex_test(input);
        let (function_decl, rest) =
            FunctionDecl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(function_decl.name.name, "myfn");
        assert_eq!(function_decl.lambda.parameters.len(), 3);
        assert_eq!(function_decl.lambda.body.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_function_decl_multiline() {
        let input = r#"myfn = a, b, c ->
  statement
  3 + 3
"#;
        let tokens = lex_test(input);
        let (function_decl, rest) =
            FunctionDecl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(function_decl.name.name, "myfn");
        assert_eq!(function_decl.lambda.parameters.len(), 3);
        assert_eq!(function_decl.lambda.body.statements.len(), 2);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_function_shorthand() {
        let input = "myfn = (+2)\n";
        let tokens = lex_test(input);
        let (function_decl, rest) =
            FunctionDecl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(function_decl.name.name, "myfn");
        assert_eq!(function_decl.lambda.parameters.len(), 1);
        assert_eq!(function_decl.lambda.body.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_function_shorthand_2() {
        let input = "myfn = (a/)\n";
        let tokens = lex_test(input);
        let (function_decl, rest) =
            FunctionDecl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(function_decl.name.name, "myfn");
        assert_eq!(function_decl.lambda.parameters.len(), 1);
        assert_eq!(function_decl.lambda.body.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_operator_function() {
        let input = "|> = a -> a\n";
        let tokens = lex_test(input);
        let (function_decl, rest) =
            FunctionDecl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(function_decl.name.name, "|>");
        assert_eq!(function_decl.lambda.parameters.len(), 1);
        assert_eq!(function_decl.lambda.body.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }
}
