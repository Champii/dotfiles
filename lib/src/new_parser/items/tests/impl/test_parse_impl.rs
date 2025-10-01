use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_impl() {
    let input = "impl Test\n";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, r#impl) = r#impl.process(ParseCtx::from(&tokens, &config)).unwrap();

    assert_eq!(r#impl.name.to_string(), "Test");
    assert_eq!(rest.len(), 0);
}
