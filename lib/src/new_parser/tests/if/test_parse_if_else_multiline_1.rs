use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_if_else_multiline_1() {
    let input = "if true\nthen 1\nelse 2";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, _if_) = parse_if.process(ParseCtx::from(&tokens, &config)).unwrap();

    assert_eq!(rest.len(), 0);
}
