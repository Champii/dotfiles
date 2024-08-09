use crate::lexer::TokenType;
use crate::new_parser::{
    engine::*, Argument, Expression, IdentOrNumber, Operand, PrimaryExpr, SecondaryExpr, UnaryExpr,
};

use super::{ident, ident_path, indent, int, operator, parenthesis, r#loop, r#match};
use super::{literal, stuck_operator_token};
use super::{parse_if, parse_type};

pub fn expression(stream: Input) -> IResult<Expression> {
    (
        unary_expr,
        (operator, expression)
            .or(preceded(
                TokenType::Eol,
                indented(preceded(indent, (operator, expression))),
            ))
            .or(preceded(
                TokenType::Eol,
                preceded(indent, (operator, expression)),
            ))
            .opt(),
    )
        .map(|(unary, binop_opt)| {
            if let Some((op, expr)) = binop_opt {
                Expression::BinopExpr(unary, op, Box::new(expr))
            } else {
                Expression::UnaryExpr(unary)
            }
        })
        .process(stream)
}

pub fn unary_expr(stream: Input) -> IResult<UnaryExpr> {
    (stuck_operator_token, unary_expr)
        .map(|(op, unary)| UnaryExpr::UnaryExpr(op, Box::new(unary)))
        .or(primary_expr.map(UnaryExpr::PrimaryExpr))
        .process(stream)
}

pub fn primary_expr(stream: Input) -> IResult<PrimaryExpr> {
    (
        operand,
        many(secondary),
        preceded(TokenType::Colon, parse_type).opt(),
    )
        .map(|(operand, secondaries, type_annotation)| PrimaryExpr {
            operand,
            secondaries: if secondaries.is_empty() {
                None
            } else {
                Some(secondaries)
            },
            type_annotation,
        })
        .process(stream)
}

pub fn operand(stream: Input) -> IResult<Operand> {
    parse_if
        .map(Box::new)
        .map(Operand::If)
        /* .or(parse_match.map(Operand::Match))
        .or(parse_loop.map(Operand::Loop))
        .or(parse_unsafe.map(Operand::Unsafe))
        .or(lambda_decl.map(Operand::LambdaDecl))
        .or(instance.map(Operand::Instance))
        .or(tuple.map(Operand::Tuple))
        .or(native_operator.map(Operand::NativeOperator)) */
        .or(r#loop.map(Box::new).map(Operand::Loop))
        .or(r#match.map(Box::new).map(Operand::Match))
        // TODO: disallow function calls after literal
        .or(literal.map(Operand::Literal))
        .or(ident_path.map(Operand::Ident))
        .or(parenthesis(expression)
            .map(Box::new)
            .map(Operand::Expression))
        // .or(self_ident.map(Operand::SelfIdent))
        .process(stream)
}

pub fn secondary(stream: Input) -> IResult<SecondaryExpr> {
    indice
        .map(SecondaryExpr::Indice)
        .or(dot.map(SecondaryExpr::Dot))
        .or(arguments.map(SecondaryExpr::Arguments))
        .process(stream)
}

pub fn arguments(stream: Input) -> IResult<Vec<Argument>> {
    TokenType::StuckOperator("!".to_string())
        .map(|_| vec![])
        .or(TokenType::Operator("!".to_string()).map(|_| vec![]))
        .or(separated1(
            expression.map(|arg| Argument { arg }),
            TokenType::Coma,
        ))
        .process(stream)
}

pub fn dot(stream: Input) -> IResult<IdentOrNumber> {
    preceded(TokenType::Dot, ident_or_number).process(stream)
}

pub fn ident_or_number(stream: Input) -> IResult<IdentOrNumber> {
    ident
        .map(IdentOrNumber::Ident)
        .or(int.map(IdentOrNumber::Number))
        .process(stream)
}

pub fn indice(stream: Input) -> IResult<Box<Expression>> {
    preceded(
        TokenType::OpenBracket,
        followed(expression.map(Box::new), TokenType::CloseBracket),
    )
    .process(stream)
}

#[cfg(test)]
mod expression {
    use std::path::PathBuf;

    use super::*;
    use crate::{
        ast::*,
        lexer::Span,
        new_parser::{lex_test, Ident, IdentifierPath, Operator},
        Config,
    };

    #[test]
    fn test_parse_expression() {
        let input = "1 + 2";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(
            expression,
            Expression::BinopExpr(
                UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(1),
                        span: Span {
                            start: 0,
                            end: 1,
                            file_path: PathBuf::default(),
                        },
                    }),
                    secondaries: None,
                    type_annotation: None,
                }),
                Operator {
                    value: "+".to_string(),
                    span: Span {
                        start: 2,
                        end: 3,
                        file_path: PathBuf::default(),
                    },
                },
                Box::new(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(2),
                        span: Span {
                            start: 4,
                            end: 5,
                            file_path: PathBuf::default(),
                        },
                    }),
                    secondaries: None,
                    type_annotation: None,
                })))
            )
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_nested_expression() {
        let input = "a.a + b + 2";
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
                            name: "a".to_string(),
                            span: Span {
                                start: 0,
                                end: 1,
                                file_path: PathBuf::default(),
                            },
                        })],
                    }),
                    secondaries: Some(vec![SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                        name: "a".to_string(),
                        span: Span {
                            start: 2,
                            end: 3,
                            file_path: PathBuf::default(),
                        },
                    }))]),
                    type_annotation: None,
                }),
                Operator {
                    value: "+".to_string(),
                    span: Span::default(),
                },
                Box::new(Expression::BinopExpr(
                    UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Ident(IdentifierPath {
                            path: vec![IdentOrType::Ident(Ident {
                                name: "b".to_string(),
                                span: Span {
                                    start: 6,
                                    end: 7,
                                    file_path: PathBuf::default(),
                                },
                            })],
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
                            kind: crate::ast::LiteralKind::Number(2),
                            span: Span {
                                start: 10,
                                end: 11,
                                file_path: PathBuf::default(),
                            },
                        }),
                        secondaries: None,
                        type_annotation: None,
                    })))
                ))
            )
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn nested_parenthesis_expression() {
        let input = "(1 + (2 + 3))";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Expression(Box::new(Expression::BinopExpr(
                    UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: crate::ast::LiteralKind::Number(1),
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
                        operand: Operand::Expression(Box::new(Expression::BinopExpr(
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
                        ))),
                        secondaries: None,
                        type_annotation: None,
                    })))
                ))),
                secondaries: None,
                type_annotation: None,
            })),
        );

        assert_eq!(rest.len(), 0);
    }

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

    #[test]
    fn bang_call_expression() {
        let input = "hello!";
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
                secondaries: Some(vec![SecondaryExpr::Arguments(vec![])]),
                type_annotation: None,
            })),
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn indice_expression() {
        let input = "hello[1]";
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
                secondaries: Some(vec![SecondaryExpr::Indice(Box::new(
                    Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
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
                    }))
                ))]),
                type_annotation: None,
            })),
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn dot_expression() {
        let input = "hello.world";
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
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                    name: "world".to_string(),
                    span: Span::default(),
                }))]),
                type_annotation: None,
            })),
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn dot_expression_with_literal() {
        let input = "4.test";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Literal(Literal {
                    kind: crate::ast::LiteralKind::Number(4),
                    span: Span::default(),
                }),
                secondaries: Some(vec![SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                    name: "test".to_string(),
                    span: Span::default(),
                }))]),
                type_annotation: None,
            })),
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn dot_expression_with_number() {
        let input = "some_tuple.1";
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
                        name: "some_tuple".to_string(),
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![SecondaryExpr::Dot(IdentOrNumber::Number(1))]),
                type_annotation: None,
            })),
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn complex_secondaries() {
        let input = "hello[1].world 1, 2, 3";
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
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![
                    SecondaryExpr::Indice(Box::new(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(
                        PrimaryExpr {
                            operand: Operand::Literal(Literal {
                                kind: crate::ast::LiteralKind::Number(1),
                                span: Span::default(),
                            }),
                            secondaries: None,
                            type_annotation: None,
                        }
                    )))),
                    SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                        name: "world".to_string(),
                        span: Span::default(),
                    })),
                    SecondaryExpr::Arguments(vec![
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Literal(Literal {
                                    kind: crate::ast::LiteralKind::Number(1),
                                    span: Span::default(),
                                }),
                                secondaries: None,
                                type_annotation: None,
                            })),
                        },
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Literal(Literal {
                                    kind: crate::ast::LiteralKind::Number(2),
                                    span: Span::default(),
                                }),
                                secondaries: None,
                                type_annotation: None,
                            })),
                        },
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Literal(Literal {
                                    kind: crate::ast::LiteralKind::Number(3),
                                    span: Span::default(),
                                }),
                                secondaries: None,
                                type_annotation: None,
                            })),
                        },
                    ]),
                ]),
                type_annotation: None,
            })),
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn multiline_dot() {
        let input = r#"foo
    .bar
    .baz"#;
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
                        name: "foo".to_string(),
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![
                    SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                        name: "bar".to_string(),
                        span: Span::default(),
                    })),
                    SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                        name: "baz".to_string(),
                        span: Span::default(),
                    })),
                ]),
                type_annotation: None,
            })),
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn spaced_dot_closes_fn_call() {
        let input = "foo a, b .bar";
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
                        name: "foo".to_string(),
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![
                    SecondaryExpr::Arguments(vec![
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(IdentifierPath {
                                    path: vec![IdentOrType::Ident(Ident {
                                        name: "a".to_string(),
                                        span: Span::default(),
                                    })],
                                }),
                                secondaries: None,
                                type_annotation: None,
                            })),
                        },
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(IdentifierPath {
                                    path: vec![IdentOrType::Ident(Ident {
                                        name: "b".to_string(),
                                        span: Span::default(),
                                    })],
                                }),
                                secondaries: None,
                                type_annotation: None,
                            })),
                        },
                    ]),
                    SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                        name: "bar".to_string(),
                        span: Span::default(),
                    }))
                ]),
                type_annotation: None,
            })),
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn spaced_dot_closes_fn_call_nested() {
        let input = "foo a, b a .bar .baz";
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
                        name: "foo".to_string(),
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![
                    SecondaryExpr::Arguments(vec![
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(IdentifierPath {
                                    path: vec![IdentOrType::Ident(Ident {
                                        name: "a".to_string(),
                                        span: Span::default(),
                                    })],
                                }),
                                secondaries: None,
                                type_annotation: None,
                            })),
                        },
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(IdentifierPath {
                                    path: vec![IdentOrType::Ident(Ident {
                                        name: "b".to_string(),
                                        span: Span::default(),
                                    })],
                                }),
                                secondaries: Some(vec![SecondaryExpr::Arguments(vec![Argument {
                                    arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(
                                        PrimaryExpr {
                                            operand: Operand::Ident(IdentifierPath {
                                                path: vec![IdentOrType::Ident(Ident {
                                                    name: "a".to_string(),
                                                    span: Span::default(),
                                                })],
                                            }),
                                            secondaries: None,
                                            type_annotation: None,
                                        },
                                    )),
                                },]),]),
                                type_annotation: None,
                            })),
                        },
                    ]),
                    SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                        name: "bar".to_string(),
                        span: Span::default(),
                    })),
                    SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                        name: "baz".to_string(),
                        span: Span::default(),
                    })),
                ]),
                type_annotation: None,
            })),
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn tuple() {
        let input = "(1, 2, 3)";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Tuple(Tuple {
                    elements: vec![
                        Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Literal(Literal {
                                kind: crate::ast::LiteralKind::Number(1),
                                span: Span::default(),
                            }),
                            secondaries: None,
                            type_annotation: None,
                        })),
                        Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Literal(Literal {
                                kind: crate::ast::LiteralKind::Number(2),
                                span: Span::default(),
                            }),
                            secondaries: None,
                            type_annotation: None,
                        })),
                        Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Literal(Literal {
                                kind: crate::ast::LiteralKind::Number(3),
                                span: Span::default(),
                            }),
                            secondaries: None,
                            type_annotation: None,
                        })),
                    ],
                }),
                secondaries: None,
                type_annotation: None,
            })),
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn nested_spaced_dot_should_close_fn_call() {
        let input = "foo a, (b .lol) .toto";
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
                        name: "foo".to_string(),
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![
                    SecondaryExpr::Arguments(vec![
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(IdentifierPath {
                                    path: vec![IdentOrType::Ident(Ident {
                                        name: "a".to_string(),
                                        span: Span::default(),
                                    })],
                                }),
                                secondaries: None,
                                type_annotation: None,
                            })),
                        },
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Expression(Box::new(Expression::UnaryExpr(
                                    UnaryExpr::PrimaryExpr(PrimaryExpr {
                                        operand: Operand::Ident(IdentifierPath {
                                            path: vec![IdentOrType::Ident(Ident {
                                                name: "b".to_string(),
                                                span: Span::default(),
                                            })],
                                        }),
                                        secondaries: Some(vec![SecondaryExpr::Dot(
                                            IdentOrNumber::Ident(Ident {
                                                name: "lol".to_string(),
                                                span: Span::default(),
                                            })
                                        )]),
                                        type_annotation: None,
                                    })
                                ))),
                                secondaries: None,
                                type_annotation: None,
                            })),
                        },
                    ]),
                    SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                        name: "toto".to_string(),
                        span: Span::default(),
                    })),
                ]),
                type_annotation: None,
            })),
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn multiline_fn_call() {
        let input = r#"foo
    bar
    baz
    2 + 2"#;
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
                        name: "foo".to_string(),
                        span: Span {
                            start: 0,
                            end: 3,
                            file_path: PathBuf::default(),
                        },
                    })],
                }),
                secondaries: Some(vec![SecondaryExpr::Arguments(vec![
                    Argument {
                        arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Ident(IdentifierPath {
                                path: vec![IdentOrType::Ident(Ident {
                                    name: "bar".to_string(),
                                    span: Span {
                                        start: 4,
                                        end: 7,
                                        file_path: PathBuf::default(),
                                    },
                                })],
                            }),
                            secondaries: None,
                            type_annotation: None,
                        })),
                    },
                    Argument {
                        arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Ident(IdentifierPath {
                                path: vec![IdentOrType::Ident(Ident {
                                    name: "baz".to_string(),
                                    span: Span {
                                        start: 8,
                                        end: 11,
                                        file_path: PathBuf::default(),
                                    },
                                })],
                            }),
                            secondaries: None,
                            type_annotation: None,
                        })),
                    },
                    Argument {
                        arg: Expression::BinopExpr(
                            UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Literal(Literal {
                                    kind: crate::ast::LiteralKind::Number(2),
                                    span: Span {
                                        start: 12,
                                        end: 13,
                                        file_path: PathBuf::default(),
                                    },
                                }),
                                secondaries: None,
                                type_annotation: None,
                            }),
                            Operator {
                                value: "+".to_string(),
                                span: Span {
                                    start: 14,
                                    end: 15,
                                    file_path: PathBuf::default(),
                                },
                            },
                            Box::new(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Literal(Literal {
                                    kind: crate::ast::LiteralKind::Number(2),
                                    span: Span {
                                        start: 16,
                                        end: 17,
                                        file_path: PathBuf::default(),
                                    },
                                }),
                                secondaries: None,
                                type_annotation: None,
                            })))
                        ),
                    },
                ])]),
                type_annotation: None,
            })),
        );
        assert_eq!(rest.len(), 0);
    }

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

    #[test]
    fn empty_self_ident() {
        let input = "@";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::SelfIdent(Ident {
                    name: "".to_string(),
                    span: Span::default(),
                }),
                secondaries: None,
                type_annotation: None,
            })),
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn double_dot() {
        let input = "foo bar ..baz";
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
                        name: "foo".to_string(),
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![
                    SecondaryExpr::Arguments(vec![Argument {
                        arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Ident(IdentifierPath {
                                path: vec![IdentOrType::Ident(Ident {
                                    name: "bar".to_string(),
                                    span: Span::default(),
                                })],
                            }),
                            secondaries: None,
                            type_annotation: None,
                        }))
                    }]),
                    SecondaryExpr::DoubleDot(Ident {
                        name: "baz".to_string(),
                        span: Span::default(),
                    }),
                ]),
                type_annotation: None,
            })),
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn double_dot_multiline() {
        let input = r#"foo bar
    ..baz
    ..foofoo"#;
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
                        name: "foo".to_string(),
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![
                    SecondaryExpr::Arguments(vec![Argument {
                        arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Ident(IdentifierPath {
                                path: vec![IdentOrType::Ident(Ident {
                                    name: "bar".to_string(),
                                    span: Span::default(),
                                })],
                            }),
                            secondaries: None,
                            type_annotation: None,
                        }))
                    }]),
                    SecondaryExpr::DoubleDot(Ident {
                        name: "baz".to_string(),
                        span: Span::default(),
                    }),
                    SecondaryExpr::DoubleDot(Ident {
                        name: "foofoo".to_string(),
                        span: Span::default(),
                    }),
                ]),
                type_annotation: None,
            })),
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn interogation() {
        let input = "foo? bar, baz?";
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
                        name: "foo".to_string(),
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![
                    SecondaryExpr::Interogation,
                    SecondaryExpr::Arguments(vec![
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(IdentifierPath {
                                    path: vec![IdentOrType::Ident(Ident {
                                        name: "bar".to_string(),
                                        span: Span::default(),
                                    })],
                                }),
                                secondaries: None,
                                type_annotation: None,
                            }))
                        },
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(IdentifierPath {
                                    path: vec![IdentOrType::Ident(Ident {
                                        name: "baz".to_string(),
                                        span: Span::default(),
                                    })],
                                }),
                                secondaries: Some(vec![SecondaryExpr::Interogation]),
                                type_annotation: None,
                            }))
                        }
                    ]),
                ]),

                type_annotation: None,
            })),
        );
        assert_eq!(rest.len(), 0);
    }

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
}
