use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn parse_infix_operator() {
    let input = "infix 5 |>\n";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, (precedence, name)) = infix_operator_decl
        .process(ParseCtx::from(&tokens, &config))
        .unwrap();

    assert_eq!(precedence, 5);
    assert_eq!(name, "|>".to_string());
    assert_eq!(rest.len(), 0);
}
