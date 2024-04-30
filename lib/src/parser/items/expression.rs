use crate::{
    ast::{
        Argument, Block, Expression, Ident, IdentOrNumber, IdentOrType, IdentifierPath, If,
        Instance, LambdaDecl, Literal, Loop, Match, NativeOperator, Operand, Operator, ParseType,
        PrimaryExpr, SecondaryExpr, Tuple, UnaryExpr,
    },
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, look_ahead, parse_indented_vec_of, parse_vec_of, ParseError},
    },
};

impl Parsable for Expression {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
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
            TokenType::Operator(_) => {
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
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
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
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let (operand, remaining_tokens) = Operand::parse(tokens, parse_ctx)?;

        let mut primary_expr = PrimaryExpr {
            operand,
            secondaries: None,
            type_annotation: None,
        };

        if remaining_tokens.is_empty() {
            return Ok((primary_expr, remaining_tokens));
        }

        let (secondaries, mut remaining_tokens_after_secondaries, _diags) =
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

        if remaining_tokens_after_secondaries.len() > 0
            && TokenType::Colon == remaining_tokens_after_secondaries[0].token_type
        {
            let (type_annotation, remaining_tokens) =
                ParseType::parse(&remaining_tokens_after_secondaries[1..], parse_ctx)?;

            primary_expr.type_annotation = Some(type_annotation);
            remaining_tokens_after_secondaries = remaining_tokens;
        }

        return Ok((primary_expr, remaining_tokens_after_secondaries));
    }
}

impl Parsable for Operand {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        if TokenType::Keyword("if".to_string()) == tokens[0].token_type {
            let (if_, remaining_tokens) = If::parse(tokens, parse_ctx)?;
            return Ok((Operand::If(Box::new(if_)), remaining_tokens));
        }

        if TokenType::Keyword("for".to_string()) == tokens[0].token_type
            || TokenType::Keyword("while".to_string()) == tokens[0].token_type
            || TokenType::Keyword("loop".to_string()) == tokens[0].token_type
        {
            let (for_, remaining_tokens) = Loop::parse(tokens, parse_ctx)?;
            return Ok((Operand::Loop(Box::new(for_)), remaining_tokens));
        }

        if TokenType::Keyword("match".to_string()) == tokens[0].token_type {
            let (match_, remaining_tokens) = Match::parse(tokens, parse_ctx)?;
            return Ok((Operand::Match(Box::new(match_)), remaining_tokens));
        }

        if TokenType::Keyword("unsafe".to_string()) == tokens[0].token_type {
            let (block, remaining_tokens) = Block::parse(&tokens[1..], parse_ctx)?;
            return Ok((Operand::Unsafe(block), remaining_tokens));
        }

        if TokenType::Arobase == tokens[0].token_type {
            if let Ok((expression, remaining_tokens)) = Ident::parse(&tokens[1..], parse_ctx) {
                return Ok((Operand::SelfIdent(expression), remaining_tokens));
            }

            return Ok((
                Operand::SelfIdent(Ident {
                    name: "".to_string(),
                    span: tokens[0].span.clone(),
                }),
                &tokens[1..],
            ));
        }

        if let TokenType::Type(_) = tokens[0].token_type {
            if let Ok((identifier_path, remaining_tokens)) =
                IdentifierPath::parse(tokens, parse_ctx)
            {
                if let IdentOrType::Type(_) = identifier_path.path.last().unwrap() {
                    if let Ok((instance, remaining_tokens)) = Instance::parse(tokens, parse_ctx) {
                        return Ok((Operand::Instance(instance), remaining_tokens));
                    } else {
                        return Err(ParseError::ExpectedType(tokens[0].span.clone()).into());
                    }
                } else {
                    return Ok((Operand::Ident(identifier_path), remaining_tokens));
                }
            } else {
                return Err(ParseError::ExpectedType(tokens[0].span.clone()).into());
            }
        }

        if TokenType::OpenParen == tokens[0].token_type {
            // first, try to parse function shorthand
            if let Ok((tuple, remaining_tokens)) = Tuple::parse(tokens, parse_ctx) {
                return Ok((Operand::Tuple(tuple), remaining_tokens));
            } else if let Ok((lambda, remaining_tokens)) = LambdaDecl::parse(tokens, parse_ctx) {
                return Ok((Operand::LambdaDecl(lambda), remaining_tokens));
            }

            let (expression, remaining_tokens) = Expression::parse(&tokens[1..], parse_ctx)?;

            // FIXME: this shouldnt exist, this is magic
            parse_ctx.inside_argument_list = true;

            let remaining_tokens = expect_token(remaining_tokens, TokenType::CloseParen)?;
            return Ok((Operand::Expression(Box::new(expression)), remaining_tokens));
        }

        if let Ok((literal, remaining_tokens)) = Literal::parse(tokens, parse_ctx) {
            return Ok((Operand::Literal(literal), remaining_tokens));
        }

        if let Ok((lambda, remaining_tokens)) = LambdaDecl::parse(tokens, parse_ctx) {
            return Ok((Operand::LambdaDecl(lambda), remaining_tokens));
        }

        if let Ok((native_operator, remaining_tokens)) = NativeOperator::parse(tokens, parse_ctx) {
            return Ok((Operand::NativeOperator(native_operator), remaining_tokens));
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
            )
            .into()),
        }
    }
}

impl Parsable for SecondaryExpr {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Operator(
                "".to_string(),
            )))?;

        if TokenType::Operator("!".to_string()) == token.token_type
            || TokenType::StuckOperator("!".to_string()) == token.token_type
        {
            Ok((SecondaryExpr::Arguments(vec![]), &tokens[1..]))
        } else if TokenType::Interogation == token.token_type {
            Ok((SecondaryExpr::Interogation, &tokens[1..]))
        } else if look_ahead(
            tokens,
            &vec![
                TokenType::Eol,
                TokenType::Indent(parse_ctx.indent_level() + parse_ctx.indent_step()),
                TokenType::Dot,
            ],
        ) {
            parse_ctx.argument_list_short_circuit()?;

            let (ident_or_number, remaining_tokens) =
                IdentOrNumber::parse(&tokens[3..], parse_ctx)?;

            Ok((SecondaryExpr::Dot(ident_or_number), remaining_tokens))
        } else if let TokenType::OpenBracket = token.token_type {
            let (expression, remaining_tokens) = Expression::parse(&tokens[1..], parse_ctx)?;
            let remaining_tokens = expect_token(remaining_tokens, TokenType::CloseBracket)?;

            Ok((
                SecondaryExpr::Indice(Box::new(expression)),
                remaining_tokens,
            ))
        } else if TokenType::Dot == token.token_type {
            let (ident_or_num, remaining_tokens) = IdentOrNumber::parse(&tokens[1..], parse_ctx)?;

            Ok((SecondaryExpr::Dot(ident_or_num), remaining_tokens))
        } else if TokenType::SpacedDot == token.token_type {
            parse_ctx.argument_list_short_circuit()?;

            let (ident_or_num, remaining_tokens) = IdentOrNumber::parse(&tokens[1..], parse_ctx)?;

            Ok((SecondaryExpr::Dot(ident_or_num), remaining_tokens))
        } else if TokenType::DoubleDot == token.token_type {
            parse_ctx.argument_list_short_circuit()?;

            let (ident, remaining_tokens) = Ident::parse(&tokens[1..], parse_ctx)?;

            Ok((SecondaryExpr::DoubleDot(ident), remaining_tokens))
        } else if {
            look_ahead(
                tokens,
                &vec![
                    TokenType::Eol,
                    TokenType::Indent(parse_ctx.indent_level() + parse_ctx.indent_step()),
                    TokenType::DoubleDot,
                ],
            )
        } {
            parse_ctx.argument_list_short_circuit()?;

            let (ident, remaining_tokens) = Ident::parse(&tokens[3..], parse_ctx)?;

            Ok((SecondaryExpr::DoubleDot(ident), remaining_tokens))
        } else {
            let (arguments, remaining_tokens) =
                parse_ctx.argument_list(|parse_ctx| ArgumentList::parse(tokens, parse_ctx))?;

            if arguments.args.is_empty() {
                return Err(
                    ParseError::UnexpectedToken(token.clone(), vec![TokenType::OpenParen]).into(),
                );
            }

            let arguments = arguments
                .args
                .into_iter()
                .map(|expr| Argument { arg: expr })
                .collect::<Vec<_>>();

            Ok((SecondaryExpr::Arguments(arguments), remaining_tokens))
        }
    }
}

struct ArgumentList {
    args: Vec<Expression>,
}

impl Parsable for ArgumentList {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = tokens;

        if !parse_ctx.disallowed_multiline_fn_call
            && look_ahead(
                remaining_tokens,
                &vec![
                    TokenType::Eol,
                    TokenType::Indent(parse_ctx.indent_level() + parse_ctx.indent_step()),
                ],
            )
        {
            let (args, new_remaining_tokens, _) =
                parse_indented_vec_of::<Expression>(&remaining_tokens[1..], parse_ctx, true)?;

            if args.is_empty() {
                return Ok((ArgumentList { args: Vec::new() }, remaining_tokens));
            }

            Ok((ArgumentList { args }, new_remaining_tokens))
        } else {
            let (args, new_remaining_tokens, _diags) =
                parse_ctx.disallow_multiline_fn_call(|parse_ctx| {
                    parse_vec_of::<Expression>(remaining_tokens, Some(TokenType::Coma), parse_ctx)
                })?;

            if args.is_empty() {
                return Err(ParseError::UnexpectedToken(
                    remaining_tokens[0].clone(),
                    vec![TokenType::Coma, TokenType::CloseParen],
                )
                .into());
            }

            Ok((ArgumentList { args }, new_remaining_tokens))
        }
    }
}

impl Parsable for Tuple {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = expect_token(tokens, TokenType::OpenParen)?;

        let (elements, remaining_tokens, _diags) =
            parse_vec_of::<Expression>(remaining_tokens, Some(TokenType::Coma), parse_ctx)?;

        if elements.len() < 2 {
            return Err(ParseError::UnexpectedToken(
                remaining_tokens[0].clone(),
                vec![TokenType::Coma, TokenType::CloseParen],
            )
            .into());
        }

        let remaining_tokens = expect_token(remaining_tokens, TokenType::CloseParen)?;

        Ok((Tuple { elements }, remaining_tokens))
    }
}

impl Parsable for NativeOperator {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Operator(
                "".to_string(),
            )))?;

        match &token.token_type {
            TokenType::NativeOperator(name) => {
                let (args, remaining_tokens, _diags) =
                    parse_vec_of(&tokens[1..], Some(TokenType::Coma), _parse_ctx)?;

                Ok((
                    NativeOperator {
                        name: name.clone(),
                        args,
                    },
                    remaining_tokens,
                ))
            }
            _ => Err(ParseError::UnexpectedToken(
                token.clone(),
                vec![TokenType::Operator("".to_string())],
            )
            .into()),
        }
    }
}

impl Parsable for Operator {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
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
            )
            .into()),
        }
    }
}

#[cfg(test)]
mod expression {
    use super::*;
    use crate::{
        ast::{IdentOrType, Literal, Operand, PrimaryExpr, UnaryExpr},
        lexer::Span,
        parser::util::lex_test,
        Config,
    };

    #[test]
    fn test_parse_expression() {
        let input = "1 + 2";
        let tokens = lex_test(input);
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            expression,
            Expression::BinopExpr(
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
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(2),
                        span: Span::default(),
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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            expression,
            Expression::BinopExpr(
                UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Ident(IdentifierPath {
                        path: vec![IdentOrType::Ident(Ident {
                            name: "a".to_string(),
                            span: Span::default(),
                        })],
                    }),
                    secondaries: Some(vec![SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                        name: "a".to_string(),
                        span: Span::default(),
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
                    Box::new(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: crate::ast::LiteralKind::Number(2),
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

    #[test]
    fn nested_parenthesis_expression() {
        let input = "(1 + (2 + 3))";
        let tokens = lex_test(input);
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(IdentifierPath {
                    path: vec![IdentOrType::Ident(Ident {
                        name: "hello".to_string(),
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![SecondaryExpr::Arguments(vec![
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
                ])]),
                type_annotation: None,
            })),
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn indice_expression() {
        let input = "hello[1]";
        let tokens = lex_test(input);
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(IdentifierPath {
                    path: vec![IdentOrType::Ident(Ident {
                        name: "hello".to_string(),
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![SecondaryExpr::Indice(Box::new(
                    Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: crate::ast::LiteralKind::Number(1),
                            span: Span::default(),
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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(IdentifierPath {
                    path: vec![IdentOrType::Ident(Ident {
                        name: "foo".to_string(),
                        span: Span::default(),
                    })],
                }),
                secondaries: Some(vec![SecondaryExpr::Arguments(vec![
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
                        })),
                    },
                    Argument {
                        arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Ident(IdentifierPath {
                                path: vec![IdentOrType::Ident(Ident {
                                    name: "baz".to_string(),
                                    span: Span::default(),
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
                                    kind: crate::ast::LiteralKind::Number(2),
                                    span: Span::default(),
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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let (expression, rest) =
            Expression::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
}
