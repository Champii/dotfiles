use crate::{
    lexer::{Token, TokenType},
    new_parser::{engine::*, Block, FunctionDecl, FunctionSig, LambdaDecl, Pattern},
};

use super::{
    block, consume_tokens_until, get_span, ident, operator_token, parenthesis, parse_type, pattern,
    reset_inside_argument_list, seek,
};

pub fn function_decl(stream: Input<'_>) -> IResult<'_, FunctionDecl> {
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
        .map_err(|e| e.with_context("function declaration"))
}

fn lambda_block(stream: Input) -> IResult<Block> {
    // Save the current indent level before parsing the lambda body
    let saved_indent = stream.indent_level;

    // Parse the block with reset argument list flags
    let result = reset_inside_argument_list(block).process(stream);

    // Restore the indent level after parsing
    match result {
        Ok((mut stream, block)) => {
            stream.indent_level = saved_indent;
            Ok((stream, block))
        }
        Err(e) => Err(e),
    }
}

pub fn lambda_decl(stream: Input) -> IResult<LambdaDecl> {
    function_shorthand
        .or(
            (parameters, TokenType::Arrow, lambda_block).map(|(parameters, _, body)| LambdaDecl {
                parameters,
                body,
                shorthand_tokens: None,
            }),
        )
        .process(stream)
        .map_err(|e| e.with_context("lambda declaration"))
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
                .map(|_| ())
                .or(TokenType::Dot.map(|_| ())),
        ),
        consume_tokens_until(TokenType::CloseParen),
    )
        .process(stream)?;

    let new_inner_tokens = vec![
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
    .chain(inner_tokens.clone().into_iter().map(|token| {
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
        lambda_decl.process(ParseCtx::from(&new_inner_tokens, stream.config))?;

    if !other_remaining_tokens.is_empty() {
        return Err(ParseError::UnexpectedToken(
            TokenType::CloseParen.discriminant().to_string(),
            other_remaining_tokens.tokens[0].clone(),
        ));
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
        ));
    }

    let inner_tokens = inner_tokens[..inner_tokens.len() - 1].to_vec();

    let new_inner_tokens = vec![
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
    .chain(inner_tokens.clone().into_iter().map(|token| {
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
        lambda_decl.process(ParseCtx::from(&new_inner_tokens, stream.config))?;

    if !other_remaining_tokens.is_empty() {
        return Err(ParseError::UnexpectedToken(
            TokenType::CloseParen.discriminant().to_string(),
            other_remaining_tokens.tokens[0].clone(),
        ));
    }

    lambda.shorthand_tokens = Some(inner_tokens);

    Ok((stream, lambda))
}

pub fn function_sig(stream: Input) -> IResult<FunctionSig> {
    (
        TokenType::Arobase.opt(),
        ident,
        TokenType::Colon,
        parse_type,
        TokenType::Eol,
    )
        .map(|(inject_self, name, _, sig, _)| FunctionSig {
            name,
            sig,
            inject_self: inject_self.is_some(),
        })
        .process(stream)
}
