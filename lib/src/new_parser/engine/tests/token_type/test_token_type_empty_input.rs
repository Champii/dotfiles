use crate::lexer::TokenType;
use crate::new_parser::engine::tests::common::*;
use crate::new_parser::engine::Parser;

#[test]
fn test_token_type_empty_input() {
    let ctx = make_ctx("");
    let mut parser = TokenType::Equal;
    
    let result = parser.process(ctx);
    assert!(result.is_err());
}

