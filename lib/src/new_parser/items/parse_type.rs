use crate::lexer::TokenType;
use crate::new_parser::engine::*;
use crate::new_parser::ParseType;
use crate::new_parser::ParseTypeInner;

use super::get_span;
use super::type_token;

pub fn parse_type(stream: Input) -> IResult<ParseType> {
    // parse_function_type
    /* .or(parse_array_type)
    .or(parse_tuple_type) */
    // .or(parse_type_inner.map(ParseType::Type))
    parse_type_inner.map(ParseType::Type).process(stream)
}

/* fn parse_function_type(stream: Input) -> IResult<ParseType> {
    (
        TokenType::OpenParen,
        parse_type,
        TokenType::Coma,
        parse_type,
        TokenType::CloseParen,
        TokenType::Arrow,
        parse_type,
    )
        .map(|(_, input, _, output, _, _, _)| FunctionType { input, output })
        .process(stream)
} */

pub fn parse_type_inner(stream: Input) -> IResult<ParseTypeInner> {
    (
        get_span,
        type_token,
        many((parse_type, TokenType::Coma.opt())),
    )
        .map(|(span, name, generics)| ParseTypeInner {
            name,
            generics: generics.into_iter().map(|(name, _)| name).collect(),
            span,
        })
        .process(stream)
}

#[cfg(test)]
mod parse_type {
    use crate::{new_parser::lex_test, Config};

    use super::*;

    #[test]
    fn test_parse_type() {
        let input = "Type";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, parse_type) = parse_type
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(parse_type.to_string(), "Type");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_type_with_generics() {
        let input = "Type Generics";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, parse_type) = parse_type
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(parse_type.to_string(), "Type Generics");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_type_with_multiple_generics() {
        let input = "Type Generics, Generics2";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, parse_type) = parse_type
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(parse_type.to_string(), "Type Generics, Generics2");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_fn_type() {
        let input = "A -> B";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, parse_type) = parse_type
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(parse_type.to_string(), "A -> B");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_nested_fn_type() {
        let input = "(A -> B) -> C";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, parse_type) = parse_type
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(parse_type.to_string(), "(A -> B) -> C");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_tuple_type() {
        let input = "(A, B)";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, parse_type) = parse_type
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();
        assert_eq!(parse_type.to_string(), "(A, B)");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_unit_type() {
        let input = "()";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, parse_type) = parse_type
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();
        assert_eq!(parse_type.to_string(), "()");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_array_type() {
        let input = "[A]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, parse_type) = parse_type
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(parse_type.to_string(), "[A]");
        assert_eq!(rest.len(), 0);
    }
}
