use crate::{
    ast::{
        ArrayPattern, Block, Expression, FieldPattern, FieldsPatternOrArgumentsPattern, Ident,
        IdentifierPath, InstancePattern, Literal, Match, MatchArm, Pattern, PatternKind,
    },
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, parse_vec_of},
        ParseError,
    },
};

impl Parsable for Match {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("match".to_string()))?;

        let (expr, remaining_tokens) = parse_ctx.disallow_multiline_fn_call(|parse_ctx| {
            Expression::parse(remaining_tokens, parse_ctx)
        })?;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        let (arms, remaining_tokens, _diags) = parse_ctx.indent_block(|parse_ctx| {
            parse_vec_of(remaining_tokens, Some(TokenType::Eol), parse_ctx)
        })?;

        Ok((Match { expr, arms }, remaining_tokens))
    }
}

impl Parsable for MatchArm {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = parse_ctx.consume_indent(tokens)?;

        let (pattern, mut remaining_tokens) = Pattern::parse(remaining_tokens, parse_ctx)?;

        let condition = if TokenType::Keyword("if".to_string()) == remaining_tokens[0].token_type {
            if let Ok((expr, new_remaining_tokens)) =
                Expression::parse(&remaining_tokens[1..], parse_ctx)
            {
                remaining_tokens = new_remaining_tokens;
                Some(expr)
            } else {
                None
            }
        } else {
            None
        };

        let remaining_tokens = expect_token(remaining_tokens, TokenType::FatArrow)?;

        let (body, remaining_tokens) = Block::parse(remaining_tokens, parse_ctx)?;

        Ok((
            MatchArm {
                pattern,
                condition,
                body,
            },
            remaining_tokens,
        ))
    }
}

impl Parsable for Pattern {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let mut remaining_tokens = tokens;

        let binding =
            if let Ok((ident, new_remaining_tokens)) = Ident::parse(remaining_tokens, parse_ctx) {
                if new_remaining_tokens.len() < 2 {
                    None
                } else {
                    if let TokenType::Arobase = new_remaining_tokens[0].token_type {
                        remaining_tokens = &new_remaining_tokens[1..];

                        Some(ident)
                    } else {
                        None
                    }
                }
            } else {
                None
            };

        let (kind, remaining_tokens) = PatternKind::parse(remaining_tokens, parse_ctx)?;

        Ok((Pattern { binding, kind }, remaining_tokens))
    }
}

impl Parsable for PatternKind {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        match tokens[0].token_type {
            TokenType::OpenParen => {
                if let Ok((patterns, remaining_tokens, _diags)) =
                    parse_vec_of(&tokens[1..], Some(TokenType::Coma), parse_ctx)
                {
                    if patterns.len() > 1 {
                        if let Ok(remaining_tokens) =
                            expect_token(remaining_tokens, TokenType::CloseParen)
                        {
                            return Ok((PatternKind::Tuple(patterns), remaining_tokens));
                        }
                    }
                }

                let (pattern, remaining_tokens) = Pattern::parse(&tokens[1..], parse_ctx)?;

                let remaining_tokens = expect_token(remaining_tokens, TokenType::CloseParen)?;

                Ok((PatternKind::Nested(Box::new(pattern)), remaining_tokens))
            }
            TokenType::OpenBracket => {
                let (patterns, remaining_tokens, _diags) =
                    parse_vec_of(&tokens[1..], Some(TokenType::Coma), parse_ctx)?;

                let remaining_tokens = expect_token(remaining_tokens, TokenType::CloseBracket)?;

                Ok((PatternKind::Array(patterns), remaining_tokens))
            }
            TokenType::Type(_) => {
                let (enum_inst, remaining_tokens) = InstancePattern::parse(tokens, parse_ctx)?;

                Ok((PatternKind::Instance(enum_inst), remaining_tokens))
            }
            TokenType::Ident(_) => {
                let (ident, remaining_tokens) = Ident::parse(tokens, parse_ctx)?;

                Ok((PatternKind::Ident(ident), remaining_tokens))
            }
            TokenType::Underscore => Ok((PatternKind::Wildcard, &tokens[1..])),
            _ => {
                if let Ok((literal, remaining_tokens)) = Literal::parse(tokens, parse_ctx) {
                    Ok((PatternKind::Literal(literal), remaining_tokens))
                } else {
                    Err(ParseError::UnexpectedToken(
                        tokens[0].clone(),
                        vec![
                            TokenType::OpenParen,
                            TokenType::Ident("".to_string()),
                            TokenType::Underscore,
                        ],
                    )
                    .into())
                }
            }
        }
    }
}

impl Parsable for ArrayPattern {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        if (TokenType::Dot == tokens[0].token_type || TokenType::SpacedDot == tokens[0].token_type)
            && TokenType::Dot == tokens[1].token_type
        {
            let (ident, remaining_tokens) = Ident::parse(&tokens[2..], parse_ctx)?;

            return Ok((ArrayPattern::Rest(ident), remaining_tokens));
        } else if let Ok((pattern, remaining_tokens)) = Pattern::parse(tokens, parse_ctx) {
            return Ok((ArrayPattern::Pattern(pattern), remaining_tokens));
        } else {
            Err(ParseError::UnexpectedToken(
                tokens[0].clone(),
                vec![TokenType::Dot, TokenType::OpenBracket],
            )
            .into())
        }
    }
}

impl Parsable for InstancePattern {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let (name, remaining_tokens) = IdentifierPath::parse(tokens, parse_ctx)?;

        let (args, remaining_tokens) =
            FieldsPatternOrArgumentsPattern::parse(remaining_tokens, parse_ctx)?;

        Ok((InstancePattern { name, args }, remaining_tokens))
    }
}

impl Parsable for FieldsPatternOrArgumentsPattern {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        if let Ok((fields, remaining_tokens, _diags)) =
            parse_vec_of(tokens, Some(TokenType::Coma), parse_ctx)
        {
            if fields.len() > 0 {
                return Ok((
                    FieldsPatternOrArgumentsPattern::Fields(fields),
                    remaining_tokens,
                ));
            }
        }

        let (args, remaining_tokens, _diags) =
            parse_vec_of(tokens, Some(TokenType::Coma), parse_ctx)?;

        Ok((
            FieldsPatternOrArgumentsPattern::Arguments(args),
            remaining_tokens,
        ))
    }
}

impl Parsable for FieldPattern {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let (ident, remaining_tokens) = Ident::parse(tokens, parse_ctx)?;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Colon)?;

        let (pattern, remaining_tokens) = Pattern::parse(remaining_tokens, parse_ctx)?;

        Ok((
            FieldPattern {
                name: ident,
                pattern,
            },
            remaining_tokens,
        ))
    }
}

#[cfg(test)]
mod r#match {
    use super::*;
    use crate::{
        ast::{
            IdentOrType, Literal, LiteralKind, Operand, Operator, ParseType, ParseTypeInner,
            PatternKind, PrimaryExpr, Statement, UnaryExpr,
        },
        lexer::Span,
        parser::util::lex_test,
        Config,
    };

    #[test]
    fn test_parse_match() {
        let input = r#"match a
    a => 2
    (a, b) => a + b"#;
        let tokens = lex_test(input);
        let (expression, rest) =
            Match::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
    fn test_parse_patter_with_binding() {
        let input = "a @ 1";
        let tokens = lex_test(input);
        let (pattern, rest) =
            Pattern::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            pattern,
            Pattern {
                binding: Some(Ident {
                    name: "a".to_string(),
                    span: Span::default()
                }),
                kind: PatternKind::Literal(Literal {
                    kind: LiteralKind::Number(1),
                    span: Span::default()
                })
            }
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_match_with_condition() {
        let input = r#"match a
    (a, b) if a => a"#;
        let tokens = lex_test(input);
        let (expression, rest) =
            Match::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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

    #[test]
    fn instance_pattern_fields_nested() {
        let input = "Player foo: (Ok 1), bar: toto";
        let tokens = lex_test(input);
        let (pattern, rest) =
            Pattern::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            pattern,
            Pattern {
                binding: None,
                kind: PatternKind::Instance(InstancePattern {
                    name: IdentifierPath {
                        path: vec![IdentOrType::Type(ParseType::Type(ParseTypeInner {
                            name: "Player".to_string(),
                            generics: vec![],
                            span: Span::default(),
                        }))]
                    },
                    args: FieldsPatternOrArgumentsPattern::Fields(vec![
                        FieldPattern {
                            name: Ident {
                                name: "foo".to_string(),
                                span: Span::default(),
                            },
                            pattern: Pattern {
                                binding: None,
                                kind: PatternKind::Nested(Box::new(Pattern {
                                    binding: None,
                                    kind: PatternKind::Instance(InstancePattern {
                                        name: IdentifierPath {
                                            path: vec![IdentOrType::Type(ParseType::Type(
                                                ParseTypeInner {
                                                    name: "Ok".to_string(),
                                                    generics: vec![],
                                                    span: Span::default(),
                                                }
                                            ))]
                                        },
                                        args: FieldsPatternOrArgumentsPattern::Arguments(vec![
                                            Pattern {
                                                binding: None,
                                                kind: PatternKind::Literal(Literal {
                                                    kind: LiteralKind::Number(1),
                                                    span: Span::default(),
                                                })
                                            }
                                        ])
                                    })
                                }))
                            }
                        },
                        FieldPattern {
                            name: Ident {
                                name: "bar".to_string(),
                                span: Span::default(),
                            },
                            pattern: Pattern {
                                binding: None,
                                kind: PatternKind::Ident(Ident {
                                    name: "toto".to_string(),
                                    span: Span::default(),
                                })
                            }
                        }
                    ])
                })
            }
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn instance_pattern_arguments_nested() {
        let input = "Player (Ok 1), toto";
        let tokens = lex_test(input);
        let (pattern, rest) =
            Pattern::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            pattern,
            Pattern {
                binding: None,
                kind: PatternKind::Instance(InstancePattern {
                    name: IdentifierPath {
                        path: vec![IdentOrType::Type(ParseType::Type(ParseTypeInner {
                            name: "Player".to_string(),
                            generics: vec![],
                            span: Span::default(),
                        }))]
                    },
                    args: FieldsPatternOrArgumentsPattern::Arguments(vec![
                        Pattern {
                            binding: None,
                            kind: PatternKind::Nested(Box::new(Pattern {
                                binding: None,
                                kind: PatternKind::Instance(InstancePattern {
                                    name: IdentifierPath {
                                        path: vec![IdentOrType::Type(ParseType::Type(
                                            ParseTypeInner {
                                                name: "Ok".to_string(),
                                                generics: vec![],
                                                span: Span::default(),
                                            }
                                        ))]
                                    },
                                    args: FieldsPatternOrArgumentsPattern::Arguments(vec![
                                        Pattern {
                                            binding: None,
                                            kind: PatternKind::Literal(Literal {
                                                kind: LiteralKind::Number(1),
                                                span: Span::default(),
                                            })
                                        }
                                    ])
                                })
                            }))
                        },
                        Pattern {
                            binding: None,
                            kind: PatternKind::Ident(Ident {
                                name: "toto".to_string(),
                                span: Span::default(),
                            })
                        }
                    ])
                })
            }
        );

        assert_eq!(rest.len(), 0);
    }
}
