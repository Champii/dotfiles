use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_struct_instance_empty_args() {
    let input = "Test";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, struct_instance) = instance.process(ParseCtx::from(&tokens, &config)).unwrap();

    assert_eq!(struct_instance.name.to_string(), "Test");
    assert_eq!(struct_instance.fields.len(), 0);
    assert_eq!(rest.len(), 0);
}
