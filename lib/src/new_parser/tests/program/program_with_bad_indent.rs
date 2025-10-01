use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn program_with_bad_indent() {
    let input = r#"main = ->
a
  2"#;

    assert!(parse_string(input, &Config::default()).is_err());
}
