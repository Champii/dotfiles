use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn program_with_no_newlines() {
    let input = r#"main = -> 1
test = -> 2"#;

    assert!(parse_string(input, &Config::default()).is_ok());
}
