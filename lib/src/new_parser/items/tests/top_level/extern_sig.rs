use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn extern_sig() {
    let input = "extern toto : Toto -> Tata\n";
    let tokens = lex_test_toplevel(input);
    let config = Config::default();

    let (rest, _top_level) = top_level.process(ParseCtx::from(&tokens, &config)).unwrap();

    assert_eq!(rest.len(), 0);
}
