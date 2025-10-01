use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn self_ident() {
    let input = "@foo";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, expression) = expression
        .process(ParseCtx::from(&tokens, &config))
        .unwrap();

    assert_eq!(
        expression,
        Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
            operand: Operand::SelfIdent(Ident {
                name: "foo".to_string(),
                span: Span::default(),
            }),
            secondaries: None,
            type_annotation: None,
        })),
    );
    assert_eq!(rest.len(), 0);
}
