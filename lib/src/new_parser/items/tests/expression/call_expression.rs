use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn call_expression() {
    let input = "hello 1, 2, 3";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, expression) = expression
        .process(ParseCtx::from(&tokens, &config))
        .unwrap();

    assert_eq!(
        expression,
        Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
            operand: Operand::Ident(IdentifierPath {
                path: vec![IdentOrType::Ident(Ident {
                    name: "hello".to_string(),
                    span: Span {
                        start: 0,
                        end: 5,
                        file_path: PathBuf::default(),
                    },
                })],
            }),
            secondaries: Some(vec![SecondaryExpr::Arguments(vec![
                Argument {
                    arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: crate::ast::LiteralKind::Number(1),
                            span: Span {
                                start: 6,
                                end: 7,
                                file_path: PathBuf::default(),
                            },
                        }),
                        secondaries: None,
                        type_annotation: None,
                    })),
                },
                Argument {
                    arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: crate::ast::LiteralKind::Number(2),
                            span: Span {
                                start: 9,
                                end: 10,
                                file_path: PathBuf::default(),
                            },
                        }),
                        secondaries: None,
                        type_annotation: None,
                    })),
                },
                Argument {
                    arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: crate::ast::LiteralKind::Number(3),
                            span: Span {
                                start: 12,
                                end: 13,
                                file_path: PathBuf::default(),
                            },
                        }),
                        secondaries: None,
                        type_annotation: None,
                    })),
                },
            ])]),
            type_annotation: None,
        })),
    );

    assert_eq!(rest.len(), 0);
}
