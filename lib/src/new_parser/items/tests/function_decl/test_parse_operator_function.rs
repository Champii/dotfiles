use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_operator_function() {
    let input = "|> = a -> a\n";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, function_decl) = function_decl
        .process(ParseCtx::from(&tokens, &config))
        .unwrap();

    assert_eq!(function_decl.name.name, "|>");
    assert_eq!(function_decl.lambda.parameters.len(), 1);
    assert_eq!(function_decl.lambda.body.statements.len(), 1);
    assert_eq!(rest.len(), 0);
}
