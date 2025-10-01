use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_string() {
    let input = "\"hello\"";
    let literal = parse_literal(input);

    assert_eq!(literal.kind, LiteralKind::String("hello".to_owned()));
}
