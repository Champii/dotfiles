use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_match_empty_lines() {
    let input = r#"match a
    
    a => 2
    
    b => 3"#;
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, _expression) = r#match.process(ParseCtx::from(&tokens, &config)).unwrap();

    assert_eq!(rest.len(), 0);
}
