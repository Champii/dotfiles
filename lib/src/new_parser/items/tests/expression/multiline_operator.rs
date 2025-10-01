use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn multiline_operator() {
    let input = r#"foo
+ 2
+ 3"#;
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, expression) = expression
        .process(ParseCtx::from(&tokens, &config))
        .unwrap();

    assert_eq!(
        expression,
        Expression::BinopExpr(
            UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(IdentifierPath {
                    path: vec![IdentOrType::Ident(Ident {
                        name: "foo".to_string(),
                        span: Span::default(),
                    })],
                }),
                secondaries: None,
                type_annotation: None,
            }),
            Operator {
                value: "+".to_string(),
                span: Span::default(),
            },
            Box::new(Expression::BinopExpr(
                UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(2),
                        span: Span::default(),
                    }),
                    secondaries: None,
                    type_annotation: None,
                }),
                Operator {
                    value: "+".to_string(),
                    span: Span::default(),
                },
                Box::new(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(3),
                        span: Span::default(),
                    }),
                    secondaries: None,
                    type_annotation: None,
                })))
            ))
        )
    );
    assert_eq!(rest.len(), 0);
}
