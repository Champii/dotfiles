use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_nested_arrays() {
    let input = "[[1, 2], [3, 4]]";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, array_outer) = array.process(ParseCtx::from(&tokens, &config)).unwrap();

    assert_eq!(array_outer.elements.len(), 2);
    assert_eq!(rest.len(), 0);
    // Vous pouvez descendre dans les éléments pour vérifier les tableaux imbriqués
}
