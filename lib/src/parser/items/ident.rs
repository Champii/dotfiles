use crate::{
    ast::{Ident, IdentOrType, IdentifierPath, ParseType},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{parse_vec_of, ParseError},
    },
};

impl Parsable for IdentifierPath {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let (idents, remaining_tokens) =
            parse_vec_of(&tokens, Some(TokenType::DoubleColon), parse_ctx)?;

        Ok((IdentifierPath { path: idents }, remaining_tokens))
    }
}

impl Parsable for IdentOrType {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        if let Ok((ident, remaining_tokens)) = Ident::parse(tokens, parse_ctx) {
            return Ok((IdentOrType::Ident(ident), remaining_tokens));
        }

        let (parse_type, remaining_tokens) = ParseType::parse(tokens, parse_ctx)?;

        Ok((IdentOrType::Type(parse_type), remaining_tokens))
    }
}

impl Parsable for Ident {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Ident("".to_string())))?;

        match &token.token_type {
            TokenType::Ident(name) => Ok((
                Ident {
                    name: name.clone(),
                    span: token.span.clone(),
                },
                &tokens[1..],
            )),
            TokenType::Operator(op) => Ok((
                Ident {
                    name: op.clone(),
                    span: token.span.clone(),
                },
                &tokens[1..],
            )),
            _ => Err(ParseError::UnexpectedToken(
                token.clone(),
                vec![TokenType::Ident("".to_string())],
            )
            .into()),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{parser::util::lex_test, Config};

    use super::*;

    #[test]
    fn test_parse_ident() {
        let input = "ident";
        let tokens = lex_test(input);
        let (ident, rest) = Ident::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(ident.name, "ident");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_ident_error() {
        let input = "123";
        let tokens = lex_test(input);
        let result = Ident::parse(&tokens, &mut ParseCtx::new(&Config::default()));

        assert!(result.is_err());
    }

    #[test]
    fn test_ident_path() {
        let input = "ident::ident";
        let tokens = lex_test(input);
        let (ident_path, rest) =
            IdentifierPath::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(ident_path.path.len(), 2);
        assert_eq!(rest.len(), 0);
    }
}
