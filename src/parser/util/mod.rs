use super::{parse_ctx::ParseCtx, Parsable};
use crate::lexer::{Token, TokenType};

mod parse_error;
pub use parse_error::ParseError;

/// Consumes a token of the given type from the input tokens.
pub fn expect_token(tokens: &[Token], token_type: TokenType) -> Result<&[Token], ParseError> {
    if tokens.is_empty() && token_type == TokenType::Eof {
        return Ok(tokens);
    }

    let token = tokens
        .get(0)
        .ok_or(ParseError::UnexpectedEof(token_type.clone()))?;
    if token.token_type != token_type {
        return Err(ParseError::UnexpectedToken(token.clone(), vec![token_type]));
    }
    Ok(tokens[1..].into())
}

/// Parse a vector of items of type T: Parsable from the input tokens.
pub fn parse_vec_of<'a, T: Parsable>(
    tokens: &'a [Token],
    delim: Option<TokenType>,
    parse_ctx: &mut ParseCtx,
) -> Result<(Vec<T>, &'a [Token]), ParseError> {
    let mut remaining_tokens = tokens;
    let mut items = Vec::new();

    let mut remaining_tokens_with_delim = tokens;

    loop {
        if remaining_tokens.is_empty() {
            break;
        }

        let Ok((item, new_remaining_tokens)) = T::parse(remaining_tokens, parse_ctx) else {
            remaining_tokens = remaining_tokens_with_delim;
            break;
        };

        remaining_tokens = new_remaining_tokens;
        remaining_tokens_with_delim = new_remaining_tokens;

        items.push(item);

        if let Some(ref delim) = delim {
            if let Ok(new_remaining_tokens) =
                expect_token(remaining_tokens_with_delim, delim.clone())
            {
                remaining_tokens = new_remaining_tokens
            } else {
                break;
            };
        }
    }

    Ok((items, remaining_tokens))
}

/// Consumes tokens until a token of the given type is found.
pub fn consume_tokens_until(tokens: &[Token], token_type: TokenType) -> (Vec<Token>, &[Token]) {
    let mut remaining_tokens = tokens;
    let mut consumed_tokens = Vec::new();

    loop {
        if remaining_tokens.is_empty() {
            break;
        }

        if remaining_tokens[0].token_type == token_type {
            break;
        }

        consumed_tokens.push(remaining_tokens[0].clone());
        remaining_tokens = &remaining_tokens[1..];
    }

    (consumed_tokens, remaining_tokens)
}
