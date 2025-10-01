use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_array_empty() {
    let input = "[]";
    let literal = parse_literal(input);

    assert_eq!(literal.kind, LiteralKind::Array(Array { elements: vec![] }));
}
