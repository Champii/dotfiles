use crate::{
    lexer::{Token, TokenType},
    new_parser::{engine::*, FunctionDecl, FunctionSig, LambdaDecl, Pattern},
};

use super::{
    block, consume_tokens_until, get_span, ident, operator_token, parenthesis, parse_type, pattern,
    seek, stuck_operator_token,
};

pub fn function_decl<'a>(stream: Input<'a>) -> IResult<'a, FunctionDecl> {
    (
        TokenType::Arobase.opt(),
        ident,
        TokenType::Equal,
        lambda_decl,
        TokenType::Eol,
    )
        .map(|(inject_self, ident, _, lambda, _)| FunctionDecl {
            name: ident,
            lambda,
            inject_self: inject_self.is_some(),
        })
        .process(stream)
}

pub fn lambda_decl(stream: Input) -> IResult<LambdaDecl> {
    function_shorthand
        .or(
            (parameters, TokenType::Arrow, block).map(|(parameters, _, body)| LambdaDecl {
                parameters,
                body,
                shorthand_tokens: None,
            }),
        )
        .process(stream)
}

fn parameters(stream: Input) -> IResult<Vec<Pattern>> {
    many((pattern, TokenType::Coma.opt()))
        .map(|pat_vec| pat_vec.into_iter().map(|(pat, _)| pat).collect::<Vec<_>>())
        .process(stream)
}

pub fn function_shorthand(stream: Input) -> IResult<LambdaDecl> {
    parenthesis(prefix_function_shorthand.or(suffix_function_shorthand)).process(stream)
}

pub fn prefix_function_shorthand(stream: Input) -> IResult<LambdaDecl> {
    let (stream, (span, _, inner_tokens)) = (
        get_span,
        seek(
            operator_token
                .or(stuck_operator_token)
                .map(|_| ())
                .or(TokenType::Dot.map(|_| ())),
        ),
        consume_tokens_until(TokenType::CloseParen),
    )
        .process(stream)?;

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

    let (other_remaining_tokens, mut lambda) =
        lambda_decl.process(ParseCtx::from(&inner_tokens, &stream.config))?;

    if !other_remaining_tokens.is_empty() {
        return Err(ParseError::UnexpectedToken(
            TokenType::CloseParen.discriminant().to_string(),
            other_remaining_tokens.tokens[0].clone(),
        )
        .into());
    }

    lambda.shorthand_tokens = Some(inner_tokens);

    Ok((stream, lambda))
}

pub fn suffix_function_shorthand(stream: Input) -> IResult<LambdaDecl> {
    let (stream, (span, inner_tokens)) =
        (get_span, consume_tokens_until(TokenType::CloseParen)).process(stream)?;

    let operator = inner_tokens.last().unwrap().clone();

    if let TokenType::Operator(_) | TokenType::StuckOperator(_) = operator.token_type {
    } else {
        return Err(ParseError::UnexpectedToken(
            TokenType::Operator("".to_string())
                .discriminant()
                .to_string(),
            operator.clone(),
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

    let (other_remaining_tokens, mut lambda) =
        lambda_decl.process(ParseCtx::from(&inner_tokens, &stream.config))?;

    if !other_remaining_tokens.is_empty() {
        return Err(ParseError::UnexpectedToken(
            TokenType::CloseParen.discriminant().to_string(),
            other_remaining_tokens.tokens[0].clone(),
        )
        .into());
    }

    lambda.shorthand_tokens = Some(inner_tokens);

    Ok((stream, lambda))
}

pub fn function_sig(stream: Input) -> IResult<FunctionSig> {
    (ident, TokenType::Colon, parse_type, TokenType::Eol)
        .map(|(name, _, sig, _)| FunctionSig { name, sig })
        .process(stream)
}

#[cfg(test)]
mod function_decl {
    use super::*;
    use crate::{new_parser::lex_test, Config};

    #[test]
    fn test_parse_function_decl_monoline() {
        let input = "myfn = -> statement\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, function_decl) = function_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(function_decl.name.name, "myfn");
        assert_eq!(function_decl.lambda.parameters.len(), 0);
        assert_eq!(function_decl.lambda.body.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_function_decl() {
        let input = "myfn = a, b, c ->\n    statement\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, function_decl) = function_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

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
        let config = Config::default();

        let (rest, function_decl) = function_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(function_decl.name.name, "myfn");
        assert_eq!(function_decl.lambda.parameters.len(), 3);
        assert_eq!(function_decl.lambda.body.statements.len(), 2);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_function_shorthand() {
        let input = "myfn = (+2)\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, function_decl) = function_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(function_decl.name.name, "myfn");
        assert_eq!(function_decl.lambda.parameters.len(), 1);
        assert_eq!(function_decl.lambda.body.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_function_shorthand_2() {
        let input = "myfn = (a/)\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, function_decl) = function_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(function_decl.name.name, "myfn");
        assert_eq!(function_decl.lambda.parameters.len(), 1);
        assert_eq!(function_decl.lambda.body.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_operator_function() {
        let input = "|> = a -> a\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, function_decl) = function_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(function_decl.name.name, "|>");
        assert_eq!(function_decl.lambda.parameters.len(), 1);
        assert_eq!(function_decl.lambda.body.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }
}
