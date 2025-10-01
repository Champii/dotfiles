use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn literal_pattern() {
    let input = "42";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, pattern) = pattern.process(ParseCtx::from(&tokens, &config)).unwrap();

    match pattern.kind {
        PatternKind::Literal(literal) => {
            match literal.kind {
                LiteralKind::Number(n) => {
                    assert_eq!(n, 42);
                }
                _ => panic!("Expected number literal"),
            }
        }
        _ => panic!("Expected literal pattern, got: {:?}", pattern.kind),
    }

    assert_eq!(rest.len(), 0);
}
