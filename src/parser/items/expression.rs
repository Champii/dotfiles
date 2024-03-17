use crate::{
    ast::{
        Argument, EnumInstance, Expression, Ident, IdentifierPath, LambdaDecl, Literal, Operand,
        Operator, PrimaryExpr, SecondaryExpr, StructInstance, UnaryExpr,
    },
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, parse_vec_of, ParseError},
    },
};

impl Parsable for Expression {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let (unary_expr, remaining_tokens) = UnaryExpr::parse(tokens, parse_ctx)?;

        if remaining_tokens.is_empty() {
            return Ok((Expression::UnaryExpr(unary_expr), remaining_tokens));
        }

        let token =
            remaining_tokens
                .get(0)
                .ok_or(ParseError::UnexpectedEof(TokenType::Operator(
                    "".to_string(),
                )))?;

        match token.token_type {
            TokenType::Operator(_) | TokenType::StuckOperator(_) => {
                let (operator, remaining_tokens) = Operator::parse(remaining_tokens, parse_ctx)?;
                let (expression, remaining_tokens) =
                    Expression::parse(remaining_tokens, parse_ctx)?;

                Ok((
                    Expression::BinopExpr(unary_expr, operator, Box::new(expression)),
                    remaining_tokens,
                ))
            }
            _ => Ok((Expression::UnaryExpr(unary_expr), remaining_tokens)),
        }
    }
}

impl Parsable for UnaryExpr {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Operator(
                "".to_string(),
            )))?;

        if let TokenType::StuckOperator(_) = token.token_type {
            let (operator, remaining_tokens) = Operator::parse(tokens, parse_ctx)?;
            let (unary_expr, remaining_tokens) = UnaryExpr::parse(remaining_tokens, parse_ctx)?;

            Ok((
                UnaryExpr::UnaryExpr(operator, Box::new(unary_expr)),
                remaining_tokens,
            ))
        } else {
            let (primary_expr, remaining_tokens) = PrimaryExpr::parse(tokens, parse_ctx)?;

            Ok((UnaryExpr::PrimaryExpr(primary_expr), remaining_tokens))
        }
    }
}

impl Parsable for PrimaryExpr {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let (operand, remaining_tokens) = Operand::parse(tokens, parse_ctx)?;

        let mut primary_expr = PrimaryExpr {
            operand,
            secondaries: None,
        };

        if remaining_tokens.is_empty() {
            return Ok((primary_expr, remaining_tokens));
        }

        let (secondaries, remaining_tokens_after_secondaries) =
            parse_vec_of::<SecondaryExpr>(remaining_tokens, None, parse_ctx)?;

        // if operand is literal, cannot be function call
        if let Operand::Literal(_) = primary_expr.operand {
            if !secondaries.is_empty() {
                if let SecondaryExpr::Arguments(_) = secondaries[0] {
                    return Ok((primary_expr, remaining_tokens));
                }
            }
        }

        if !secondaries.is_empty() {
            primary_expr.secondaries = Some(secondaries);
        }

        return Ok((primary_expr, remaining_tokens_after_secondaries));
    }
}

impl Parsable for SecondaryExpr {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Operator(
                "".to_string(),
            )))?;

        // FIXME: Trick to have binop expr parsed
        match token.token_type {
            TokenType::Operator(_) | TokenType::StuckOperator(_) => {
                return Err(ParseError::UnexpectedToken(
                    token.clone(),
                    vec![TokenType::Dot, TokenType::OpenBracket, TokenType::OpenParen],
                ));
            }
            _ => {}
        }

        if let TokenType::OpenBracket = token.token_type {
            let (expression, remaining_tokens) = Expression::parse(&tokens[1..], parse_ctx)?;
            let remaining_tokens = expect_token(remaining_tokens, TokenType::CloseBracket)?;

            Ok((
                SecondaryExpr::Indice(Box::new(expression)),
                remaining_tokens,
            ))
        } else if let TokenType::Dot = token.token_type {
            let (ident, remaining_tokens) = Ident::parse(&tokens[1..], parse_ctx)?;

            Ok((SecondaryExpr::Dot(ident), remaining_tokens))
        } else {
            let (arguments, remaining_tokens) =
                parse_vec_of(tokens, Some(TokenType::Coma), parse_ctx)?;

            if arguments.is_empty() {
                return Err(ParseError::UnexpectedToken(
                    token.clone(),
                    vec![TokenType::OpenParen],
                ));
            }

            let arguments = arguments
                .into_iter()
                .map(|expr| Argument { arg: expr })
                .collect::<Vec<_>>();

            Ok((SecondaryExpr::Arguments(arguments), remaining_tokens))
        }
    }
}

impl Parsable for Operand {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        if TokenType::Arobase == tokens[0].token_type {
            let (expression, remaining_tokens) = Ident::parse(&tokens[1..], parse_ctx)?;
            return Ok((Operand::SelfIdent(expression), remaining_tokens));
        }

        if let TokenType::Type(_) = tokens[0].token_type {
            if let TokenType::DoubleColon = tokens[1].token_type {
                let (enum_instance, remaining_tokens) = EnumInstance::parse(tokens, parse_ctx)?;
                return Ok((Operand::EnumInstance(enum_instance), remaining_tokens));
            } else {
                let (instance, remaining_tokens) = StructInstance::parse(tokens, parse_ctx)?;
                return Ok((Operand::StructInstance(instance), remaining_tokens));
            }
        }

        if TokenType::OpenParen == tokens[0].token_type {
            // first, try to parse function shorthand
            if let Ok((lambda, remaining_tokens)) = LambdaDecl::parse(tokens, parse_ctx) {
                return Ok((Operand::LambdaDecl(lambda), remaining_tokens));
            }
            let (expression, remaining_tokens) = Expression::parse(&tokens[1..], parse_ctx)?;
            let remaining_tokens = expect_token(remaining_tokens, TokenType::CloseParen)?;
            return Ok((Operand::Expression(Box::new(expression)), remaining_tokens));
        }

        if let Ok((literal, remaining_tokens)) = Literal::parse(tokens, parse_ctx) {
            return Ok((Operand::Literal(literal), remaining_tokens));
        }

        if let Ok((lambda, remaining_tokens)) = LambdaDecl::parse(tokens, parse_ctx) {
            return Ok((Operand::LambdaDecl(lambda), remaining_tokens));
        }

        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Ident("".to_string())))?;

        match &token.token_type {
            TokenType::Ident(_) => {
                let (identifier_path, remaining_tokens) = IdentifierPath::parse(tokens, parse_ctx)?;
                Ok((Operand::Ident(identifier_path), remaining_tokens))
            }
            _ => Err(ParseError::UnexpectedToken(
                token.clone(),
                vec![
                    TokenType::Ident("".to_string()),
                    TokenType::MacroInvoc("".to_string()),
                    TokenType::Number("".to_string()),
                ],
            )),
        }
    }
}

impl Parsable for Operator {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Operator(
                "".to_string(),
            )))?;

        match &token.token_type {
            TokenType::Operator(value) | TokenType::StuckOperator(value) => Ok((
                Operator {
                    value: value.clone(),
                    span: token.span.clone(),
                },
                &tokens[1..],
            )),
            _ => Err(ParseError::UnexpectedToken(
                token.clone(),
                vec![TokenType::Operator("".to_string())],
            )),
        }
    }
}

#[cfg(test)]
mod expression {
    use super::*;
    use crate::{
        ast::{Literal, Operand, PrimaryExpr, UnaryExpr},
        lexer::Span,
        parser::util::lex_test,
    };

    #[test]
    fn test_parse_expression() {
        let input = "1 + 2";
        let tokens = lex_test(input);
        let (expression, rest) = Expression::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            expression,
            Expression::BinopExpr(
                UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(1),
                        span: Span::default(),
                    }),
                    secondaries: None,
                }),
                Operator {
                    value: "+".to_string(),
                    span: Span::default(),
                },
                Box::new(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(2),
                        span: Span::default(),
                    }),
                    secondaries: None,
                })))
            )
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_nested_expression() {
        let input = "a.a + b + 2";
        let tokens = lex_test(input);
        let (expression, rest) = Expression::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            expression,
            Expression::BinopExpr(
                UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Ident(IdentifierPath {
                        path: vec![Ident {
                            name: "a".to_string(),
                            span: Span::default(),
                        }],
                    }),
                    secondaries: Some(vec![SecondaryExpr::Dot(Ident {
                        name: "a".to_string(),
                        span: Span::default(),
                    })]),
                }),
                Operator {
                    value: "+".to_string(),
                    span: Span::default(),
                },
                Box::new(Expression::BinopExpr(
                    UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Ident(IdentifierPath {
                            path: vec![Ident {
                                name: "b".to_string(),
                                span: Span::default(),
                            }],
                        }),
                        secondaries: None,
                    }),
                    Operator {
                        value: "+".to_string(),
                        span: Span::default(),
                    },
                    Box::new(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: crate::ast::LiteralKind::Number(2),
                            span: Span::default(),
                        }),
                        secondaries: None,
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
        let (expression, rest) = Expression::parse(&tokens, &mut ParseCtx::new()).unwrap();

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
                            })))
                        ))),
                        secondaries: None,
                    })))
                ))),
                secondaries: None,
            })),
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn call_expression() {
        let input = "hello 1, 2, 3";
        let tokens = lex_test(input);
        let (expression, rest) = Expression::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(IdentifierPath {
                    path: vec![Ident {
                        name: "hello".to_string(),
                        span: Span::default(),
                    }],
                }),
                secondaries: Some(vec![SecondaryExpr::Arguments(vec![
                    Argument {
                        arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Literal(Literal {
                                kind: crate::ast::LiteralKind::Number(1),
                                span: Span::default(),
                            }),
                            secondaries: None,
                        })),
                    },
                    Argument {
                        arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Literal(Literal {
                                kind: crate::ast::LiteralKind::Number(2),
                                span: Span::default(),
                            }),
                            secondaries: None,
                        })),
                    },
                    Argument {
                        arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Literal(Literal {
                                kind: crate::ast::LiteralKind::Number(3),
                                span: Span::default(),
                            }),
                            secondaries: None,
                        })),
                    },
                ])]),
            })),
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn indice_expression() {
        let input = "hello[1]";
        let tokens = lex_test(input);
        let (expression, rest) = Expression::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(IdentifierPath {
                    path: vec![Ident {
                        name: "hello".to_string(),
                        span: Span::default(),
                    }],
                }),
                secondaries: Some(vec![SecondaryExpr::Indice(Box::new(
                    Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: crate::ast::LiteralKind::Number(1),
                            span: Span::default(),
                        }),
                        secondaries: None,
                    }))
                ))]),
            })),
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn dot_expression() {
        let input = "hello.world";
        let tokens = lex_test(input);
        let (expression, rest) = Expression::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(IdentifierPath {
                    path: vec![Ident {
                        name: "hello".to_string(),
                        span: Span::default(),
                    }],
                }),
                secondaries: Some(vec![SecondaryExpr::Dot(Ident {
                    name: "world".to_string(),
                    span: Span::default(),
                })]),
            })),
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn dot_expression_with_literal() {
        let input = "4.test";
        let tokens = lex_test(input);
        let (expression, rest) = Expression::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Literal(Literal {
                    kind: crate::ast::LiteralKind::Number(4),
                    span: Span::default(),
                }),
                secondaries: Some(vec![SecondaryExpr::Dot(Ident {
                    name: "test".to_string(),
                    span: Span::default(),
                })]),
            })),
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn complex_secondaries() {
        let input = "hello[1].world 1, 2, 3";
        let tokens = lex_test(input);
        let (expression, rest) = Expression::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(IdentifierPath {
                    path: vec![Ident {
                        name: "hello".to_string(),
                        span: Span::default(),
                    }],
                }),
                secondaries: Some(vec![
                    SecondaryExpr::Indice(Box::new(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(
                        PrimaryExpr {
                            operand: Operand::Literal(Literal {
                                kind: crate::ast::LiteralKind::Number(1),
                                span: Span::default(),
                            }),
                            secondaries: None,
                        }
                    )))),
                    SecondaryExpr::Dot(Ident {
                        name: "world".to_string(),
                        span: Span::default(),
                    }),
                    SecondaryExpr::Arguments(vec![
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Literal(Literal {
                                    kind: crate::ast::LiteralKind::Number(1),
                                    span: Span::default(),
                                }),
                                secondaries: None,
                            })),
                        },
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Literal(Literal {
                                    kind: crate::ast::LiteralKind::Number(2),
                                    span: Span::default(),
                                }),
                                secondaries: None,
                            })),
                        },
                        Argument {
                            arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Literal(Literal {
                                    kind: crate::ast::LiteralKind::Number(3),
                                    span: Span::default(),
                                }),
                                secondaries: None,
                            })),
                        },
                    ]),
                ]),
            })),
        );

        assert_eq!(rest.len(), 0);
    }
}
