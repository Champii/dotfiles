use crate::new_parser::engine::tests::common::*;
use crate::new_parser::engine::{not, Parser};

#[test]
fn test_not_fails_when_parser_succeeds() {
    let ctx = make_ctx("foo bar");
    let mut parser = not(ident_parser());

    let result = parser.process(ctx);
    assert!(result.is_err());
}

