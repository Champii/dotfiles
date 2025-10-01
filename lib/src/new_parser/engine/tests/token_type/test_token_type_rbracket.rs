use crate::lexer::TokenType;
use crate::new_parser::engine::tests::common::*;
use crate::new_parser::engine::Parser;

#[test]
fn test_token_type_rbracket() {
    let ctx = make_ctx("]");
    let mut parser = TokenType::CloseBracket;
    
    let result = parser.process(ctx);
    assert!(result.is_ok());
    
    let (rest, output) = result.unwrap();
    assert_eq!(output, TokenType::CloseBracket);
    assert_eq!(rest.len(), 0);
}

