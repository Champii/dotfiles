use crate::{
    ast::Ident,
    lexer::{Token, TokenType},
    parser::{parsable::Parsable, parse_ctx::ParseCtx, util::ParseError},
};

impl Parsable for Ident {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let token = tokens.get(0).ok_or(ParseError::UnexpectedEof)?;

        match &token.token_type {
            TokenType::Ident(name) => Ok((
                Ident {
                    name: name.clone(),
                    span: token.span.clone(),
                },
                &tokens[1..],
            )),
            _ => Err(ParseError::UnexpectedToken(
                token.clone(),
                vec![TokenType::Ident("".to_string())],
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::lexer::Lexer;

    fn lex(input: &str) -> Vec<Token> {
        Lexer::new(PathBuf::new(), input)
            .unwrap()
            .collect()
            .unwrap()
    }

    #[test]
    fn test_parse_ident() {
        let input = "ident";
        let tokens = lex(input);
        let tokens = &tokens[1..]; // skip the Indent
        let (ident, rest) = Ident::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(ident.name, "ident");
        assert_eq!(rest.len(), 1);
        assert_eq!(rest[0].token_type, TokenType::Eof);
    }

    #[test]
    fn test_parse_ident_error() {
        let input = "123";
        let tokens = lex(input);
        let tokens = &tokens[1..]; // skip the Indent
        let result = Ident::parse(&tokens, &mut ParseCtx::new());

        assert!(result.is_err());
    }
}
