use crate::{
    ast::ParseType,
    lexer::{Token, TokenType},
    parser::{
        parse_ctx::ParseCtx,
        util::{parse_vec_of, ParseError},
        Parsable,
    },
};

impl Parsable for ParseType {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Type("".to_string())))?;

        let ((name, span), remmaining_tokens) = match &token.token_type {
            TokenType::Type(name) => ((name.clone(), token.span.clone()), &tokens[1..]),
            _ => {
                return Err(ParseError::UnexpectedToken(
                    token.clone(),
                    vec![TokenType::Ident("".to_string())],
                ))
            }
        };

        let (generics, remmaining_tokens) =
            parse_vec_of(&remmaining_tokens, Some(TokenType::Coma), parse_ctx)?;

        let parse_type = ParseType {
            name,
            span,
            generics,
        };

        Ok((parse_type, remmaining_tokens))
    }
}
#[cfg(test)]
mod parse_type {
    use crate::parser::util::lex_test;

    use super::*;

    #[test]
    fn test_parse_type() {
        let input = "Type";
        let tokens = lex_test(input);
        let (parse_type, rest) = ParseType::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(parse_type.name, "Type");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_type_with_generics() {
        let input = "Type Generics";
        let tokens = lex_test(input);
        let (parse_type, rest) = ParseType::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(parse_type.name, "Type");
        assert_eq!(parse_type.generics.len(), 1);
        assert_eq!(parse_type.generics[0].name, "Generics");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_type_with_multiple_generics() {
        let input = "Type Generics, Generics2";
        let tokens = lex_test(input);
        let (parse_type, rest) = ParseType::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(parse_type.name, "Type");
        assert_eq!(parse_type.generics.len(), 2);
        assert_eq!(parse_type.generics[0].name, "Generics");
        assert_eq!(parse_type.generics[1].name, "Generics2");
        assert_eq!(rest.len(), 0);
    }
}
