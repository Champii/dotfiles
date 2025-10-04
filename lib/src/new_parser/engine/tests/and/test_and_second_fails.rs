use crate::new_parser::engine::tests::common::*;
use crate::new_parser::engine::Parser;

#[test]
fn test_and_second_fails() {
    let ctx = make_ctx("foo bar");
    let mut parser = ident_parser().and(number_parser());
    
    let result = parser.process(ctx);
    assert!(result.is_err());
}

