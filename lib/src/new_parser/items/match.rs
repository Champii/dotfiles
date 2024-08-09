use crate::{
    lexer::TokenType,
    new_parser::{engine::*, Match, MatchArm},
};

use super::{block, disallow_multiline_fn_call, expression, indent, pattern};

pub fn r#match(stream: Input) -> IResult<Match> {
    (
        TokenType::Keyword("match".to_string()),
        disallow_multiline_fn_call(expression),
        TokenType::Eol,
        indented(separated1(match_arm, TokenType::Eol)),
    )
        .map(|(_, expr, _, arms)| Match { expr, arms })
        .process(stream)
}

pub fn match_arm(stream: Input) -> IResult<MatchArm> {
    (
        indent,
        pattern,
        preceded(TokenType::Keyword("if".to_string()), expression).opt(),
        TokenType::FatArrow,
        block,
    )
        .map(|(_, pattern, condition, _, body)| MatchArm {
            pattern,
            condition,
            body,
        })
        .process(stream)
}

#[cfg(test)]
mod r#match {
    use super::*;
    use crate::{ast::*, lexer::Span, new_parser::lex_test, Config};

    #[test]
    fn test_parse_match() {
        let input = r#"match a
    a => 2
    (a, b) => a + b"#;
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = r#match.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            expression,
            Match {
                expr: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Ident(crate::ast::IdentifierPath {
                        path: vec![IdentOrType::Ident(Ident {
                            name: "a".to_string(),
                            span: Span::default(),
                        })]
                    }),
                    secondaries: None,
                    type_annotation: None,
                })),
                arms: vec![
                    MatchArm {
                        pattern: Pattern {
                            binding: None,
                            kind: PatternKind::Ident(Ident {
                                name: "a".to_string(),
                                span: Span::default(),
                            })
                        },
                        condition: None,
                        body: Block {
                            statements: vec![Statement::Expression(Expression::UnaryExpr(
                                UnaryExpr::PrimaryExpr(PrimaryExpr {
                                    operand: Operand::Literal(Literal {
                                        kind: LiteralKind::Number(2),
                                        span: Span::default()
                                    }),
                                    secondaries: None,
                                    type_annotation: None,
                                })
                            ))]
                        }
                    },
                    MatchArm {
                        pattern: Pattern {
                            binding: None,
                            kind: PatternKind::Tuple(vec![
                                Pattern {
                                    binding: None,
                                    kind: PatternKind::Ident(Ident {
                                        name: "a".to_string(),
                                        span: Span::default(),
                                    })
                                },
                                Pattern {
                                    binding: None,
                                    kind: PatternKind::Ident(Ident {
                                        name: "b".to_string(),
                                        span: Span::default(),
                                    })
                                }
                            ])
                        },
                        condition: None,
                        body: Block {
                            statements: vec![Statement::Expression(Expression::BinopExpr(
                                UnaryExpr::PrimaryExpr(PrimaryExpr {
                                    operand: Operand::Ident(crate::ast::IdentifierPath {
                                        path: vec![IdentOrType::Ident(Ident {
                                            name: "a".to_string(),
                                            span: Span::default(),
                                        })]
                                    }),
                                    secondaries: None,
                                    type_annotation: None,
                                }),
                                Operator {
                                    value: "+".to_string(),
                                    span: Span::default(),
                                },
                                Box::new(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(
                                    PrimaryExpr {
                                        operand: Operand::Ident(crate::ast::IdentifierPath {
                                            path: vec![IdentOrType::Ident(Ident {
                                                name: "b".to_string(),
                                                span: Span::default(),
                                            })]
                                        }),
                                        secondaries: None,
                                        type_annotation: None,
                                    }
                                )))
                            ))]
                        }
                    }
                ]
            }
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_match_with_condition() {
        let input = r#"match a
    (a, b) if a => a"#;
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = r#match.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            expression,
            Match {
                expr: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Ident(crate::ast::IdentifierPath {
                        path: vec![IdentOrType::Ident(Ident {
                            name: "a".to_string(),
                            span: Span::default(),
                        })]
                    }),
                    secondaries: None,
                    type_annotation: None,
                })),
                arms: vec![MatchArm {
                    pattern: Pattern {
                        binding: None,
                        kind: PatternKind::Tuple(vec![
                            Pattern {
                                binding: None,
                                kind: PatternKind::Ident(Ident {
                                    name: "a".to_string(),
                                    span: Span::default(),
                                })
                            },
                            Pattern {
                                binding: None,
                                kind: PatternKind::Ident(Ident {
                                    name: "b".to_string(),
                                    span: Span::default(),
                                })
                            }
                        ])
                    },
                    condition: Some(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Ident(crate::ast::IdentifierPath {
                            path: vec![IdentOrType::Ident(Ident {
                                name: "a".to_string(),
                                span: Span::default(),
                            })]
                        }),
                        secondaries: None,
                        type_annotation: None,
                    }))),
                    body: Block {
                        statements: vec![Statement::Expression(Expression::UnaryExpr(
                            UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(crate::ast::IdentifierPath {
                                    path: vec![IdentOrType::Ident(Ident {
                                        name: "a".to_string(),
                                        span: Span::default(),
                                    })]
                                }),
                                secondaries: None,
                                type_annotation: None,
                            })
                        ))]
                    }
                }]
            }
        );

        assert_eq!(rest.len(), 0);
    }
}
