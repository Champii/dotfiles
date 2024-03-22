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
/// It does not consume the last delimiter token.
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
            remaining_tokens = remaining_tokens_with_delim;
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
/// The last token is NOT INCLUDED
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

pub fn ignore_empty_lines(mut tokens: &[Token]) -> &[Token] {
    while tokens
        .get(0)
        .map(|t| {
            if let TokenType::Indent(_) = t.token_type {
                true
            } else {
                false
            }
        })
        .unwrap_or(false)
        && tokens
            .get(1)
            .map(|t| t.token_type == TokenType::Eol)
            .unwrap_or(false)
    {
        tokens = &tokens[2..];
    }

    tokens
}

pub fn look_ahead(tokens: &[Token], expected: &[TokenType]) -> bool {
    if tokens.len() < expected.len() {
        return false;
    }

    for (i, expected_token) in expected.iter().enumerate() {
        if let Some(token) = tokens.get(i) {
            if token.token_type != *expected_token {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
pub fn lex_test(input: &str) -> Vec<Token> {
    use crate::lexer::Lexer;

    let mut tokens = Lexer::new(std::path::PathBuf::new(), input)
        .unwrap()
        .with_newline_at_end(false)
        .collect()
        .unwrap();

    //ignore indent
    tokens.remove(0);
    //ignore EOF
    tokens.pop();

    tokens
}
