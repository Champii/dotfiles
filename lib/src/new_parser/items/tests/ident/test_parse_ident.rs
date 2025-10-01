use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_ident() {
    let input = "ident";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, ident) = ident.process(ParseCtx::from(&tokens, &config)).unwrap();

    assert_eq!(ident.name, "ident");
    assert_eq!(rest.len(), 0);
}
