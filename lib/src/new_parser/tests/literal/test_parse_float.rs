use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_float() {
    let input = "123.456";
    let literal = parse_literal(input);

    assert_eq!(literal.kind, LiteralKind::Float(123.456));
}
