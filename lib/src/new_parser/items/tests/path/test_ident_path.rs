use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_ident_path() {
    let input = "ident::Type::ident";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, ident) = ident_path
        .process(ParseCtx::from(&tokens, &config))
        .unwrap();

    assert_eq!(ident.path.len(), 3);
    assert_eq!(rest.len(), 0);
}
