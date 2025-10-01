use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn unary_operator_expression() {
    let input = "-x";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, expression) = expression
        .process(ParseCtx::from(&tokens, &config))
        .unwrap();

    match expression {
        Expression::UnaryExpr(UnaryExpr::UnaryExpr(op, _)) => {
            assert_eq!(op.value, "-");
        }
        _ => panic!("Expected unary expression, got: {:?}", expression),
    }
    assert_eq!(rest.len(), 0);
}
