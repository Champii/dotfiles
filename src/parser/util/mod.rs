use crate::lexer::{Token, TokenType};
use crate::parser::Parsable;

mod parse_error;
pub use parse_error::ParseError;

/// Consumes a token of the given type from the input tokens.
pub fn expect_token(tokens: &[Token], token_type: TokenType) -> Result<&[Token], ParseError> {
    let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;
    if token.token_type != token_type {
        return Err(ParseError::UnexpectedToken(token.clone()));
    }
    Ok(tokens[1..].into())
}

/// Parse a vector of items of type T: Parsable from the input tokens.
pub fn parse_vec_of<T: Parsable>(
    tokens: &[Token],
    delim: Option<TokenType>,
) -> Result<(Vec<T>, &[Token]), ParseError> {
    let mut remaining_tokens = tokens;
    let mut items = Vec::new();

    loop {
        if remaining_tokens.is_empty() {
            break;
        }

        let Ok((item, new_remaining_tokens)) = T::parse(remaining_tokens) else {
            break;
        };

        remaining_tokens = new_remaining_tokens;

        items.push(item);

        if let Some(ref delim) = delim {
            let Ok(new_remaining_tokens) = expect_token(remaining_tokens, delim.clone()) else {
                break;
            };

            remaining_tokens = new_remaining_tokens;
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
