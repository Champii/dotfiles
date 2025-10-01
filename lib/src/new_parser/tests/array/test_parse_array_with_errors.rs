use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_array_with_errors() {
    let input = "[1, 2,";
    let tokens = lex_test(input);
    let config = Config::default();

    let result = array.process(ParseCtx::from(&tokens, &config));

    assert!(result.is_err());
}
