use crate::new_parser::engine::tests::common::*;
use crate::new_parser::engine::{many1, Parser};

#[test]
fn test_many1_zero_matches_fails() {
    let ctx = make_ctx("foo");
    let mut parser = many1(number_parser());
    
    let result = parser.process(ctx);
    assert!(result.is_err());
}

