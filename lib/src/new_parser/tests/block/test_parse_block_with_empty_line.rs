use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_block_with_empty_line() {
    let input = "\n    statement\n\n    statement\n    \n    statement";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, block) = block.process(ParseCtx::from(&tokens, &config)).unwrap();

    assert_eq!(block.statements.len(), 3);
    assert_eq!(rest.len(), 0);
}
