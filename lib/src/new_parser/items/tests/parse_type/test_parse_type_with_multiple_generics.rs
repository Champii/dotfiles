use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

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
