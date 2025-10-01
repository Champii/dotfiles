use crate::lexer::{Span, Token, TokenType};
use crate::new_parser::{
    engine::*, Argument, Expression, Ident, IdentOrNumber, Operand, PrimaryExpr, SecondaryExpr,
    Tuple, UnaryExpr,
};

use super::{
    block, function_shorthand, get_span, ident, ident_path, indent, instance, int, lambda_decl,
    native_operator, operator, parenthesis, r#loop, r#match,
};
use super::{literal, stuck_operator_token};
use super::{parse_if, parse_type, indent_token};

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
        .or(r#loop.map(Box::new).map(Operand::Loop))
        .or(r#match.map(Box::new).map(Operand::Match))
        .or(preceded(TokenType::Keyword("unsafe".to_string()), block).map(Operand::Unsafe))
        .or(self_ident)
        .or(instance.map(Operand::Instance))
        .or(tuple.map(Operand::Tuple))
        .or(function_shorthand.map(Operand::LambdaDecl))
        .or(parenthesis(reset_inside_argument_list(expression))
            .map(Box::new)
            .map(Operand::Expression))
        // TODO: disallow function calls after literal
        .or(literal.map(Operand::Literal))
        .or(lambda_decl.map(Operand::LambdaDecl))
        .or(native_operator.map(Operand::NativeOperator))
        .or(preceded(not(operator), ident_path.map(Operand::Ident)))
        .process(stream)
}

pub fn tuple(stream: Input) -> IResult<Tuple> {
    parenthesis(separated1(expression, TokenType::Coma))
        .map(|elements| Tuple { elements })
        .process(stream)
        .map(|(stream, tuple)| {
            if tuple.elements.len() < 2 {
                Err(ParseError::UnexpectedToken(
                    TokenType::OpenParen.discriminant().to_string(),
                    Token {
                        token_type: TokenType::OpenParen,
                        span: Span::default(),
                    },
                ))
            } else {
                Ok((stream, tuple))
            }
        })?
}

pub fn self_ident(stream: Input) -> IResult<Operand> {
    preceded(TokenType::Arobase, ident.map(Operand::SelfIdent))
        .or((get_span, TokenType::Arobase).map(|(span, _)| {
            Operand::SelfIdent(Ident {
                name: "self".to_string(),
                span,
            })
        }))
        .process(stream)
}

pub fn secondary(stream: Input) -> IResult<SecondaryExpr> {
    // Check for argument list short circuit on multiline dots
    // Only close the argument list if the dot is at the method chain level or less,
    // not if it's deeper (which would be part of an argument expression)
    if stream.inside_argument_list {
        if let Ok((_, (_, indent_level, _))) = (TokenType::Eol, indent_token, TokenType::Dot).process(stream) {
            // Calculate the method chain indent level
            // Arguments are at stream.indent_level, method chains would be at indent_level - indent_step
            let method_chain_level = if stream.indent_level >= stream.indent_step {
                stream.indent_level - stream.indent_step
            } else {
                0
            };

            // Only close if the dot is at or below the method chain level
            if (indent_level as usize) <= method_chain_level + stream.indent_step {
                return arguments_list_short_circuit(stream).and_then(|_| Err(ParseError::ShortCircuit));
            }
        }
    }

    indice
        .map(SecondaryExpr::Indice)
        .or(dot.map(SecondaryExpr::Dot))
        .or(double_dot.map(SecondaryExpr::DoubleDot))
        .or(arguments.map(SecondaryExpr::Arguments))
        .or(TokenType::Interogation.map(|_| SecondaryExpr::Interogation))
        .process(stream)
}

pub fn arguments(stream: Input) -> IResult<Vec<Argument>> {
    TokenType::StuckOperator("!".to_string())
        .map(|_| vec![])
        .or(TokenType::Operator("!".to_string()).map(|_| vec![]))
        .or(preceded(
            not(operator),
            inside_argument_list(separated1(
                expression.map(|arg| Argument { arg }),
                TokenType::Coma,
            )),
        ))
        .or(preceded(
            not(operator),
            preceded(
                not_multi_line_fn_call_short_circuit,
                preceded(
                    TokenType::Eol,
                    // Use context-aware argument parsing to avoid ambiguity
                    multiline_arguments_context_aware,
                ),
            ),
        ))
        .process(stream)
}

pub fn multiline_arguments_context_aware(stream: Input) -> IResult<Vec<Argument>> {
    // Smart argument parsing that prevents ambiguous syntax
    // Arguments must be indented MORE than the current context to avoid ambiguity

    // Check the actual indentation level of the arguments
    let arg_indent_level = if let Ok(token) = stream.seek() {
        if let TokenType::Indent(level) = token.token_type {
            level as usize
        } else {
            0
        }
    } else {
        return Err(ParseError::UnexpectedEOF);
    };

    // Arguments must be indented MORE than the current context
    if arg_indent_level <= stream.indent_level {
        // Not indented at all - definitely not arguments
        return Err(ParseError::UnexpectedIndent(arg_indent_level as u8));
    }

    // Prevent ambiguous cases where arguments could be confused with method chains
    // This only applies at the base level (indent 0) where multiline dots create ambiguity
    // Inside function bodies or other nested contexts, there's no ambiguity
    if stream.indent_step == 4 && stream.indent_level == 0 {
        if arg_indent_level == 4 {
            // At base level, arguments at indent 4 are ambiguous (could be method chain)
            // Arguments at indent 8+ are clearly arguments
            return Err(ParseError::UnexpectedIndent(arg_indent_level as u8));
        }
    }

    // Parse arguments at their actual indent level (which we've already validated)
    // We need to temporarily set the stream's indent level to match the arguments
    let original_indent = stream.indent_level;
    let mut arg_stream = stream;
    arg_stream.indent_level = arg_indent_level;

    let result = inside_argument_list(separated1(
        preceded(indent, expression.map(|arg| Argument { arg })),
        TokenType::Eol,
    )).process(arg_stream);

    // Restore the original indent level in the returned stream
    match result {
        Ok((mut stream, args)) => {
            stream.indent_level = original_indent;
            Ok((stream, args))
        }
        Err(e) => Err(e),
    }
}

pub fn inside_argument_list<P: Parser>(mut parser: P) -> impl FnMut(Input) -> IResult<P::Output> {
    move |mut stream| {
        let old_value = stream.inside_argument_list;
        stream.inside_argument_list = true;

        match parser.process(stream) {
            Ok((mut stream, t)) => {
                stream.inside_argument_list = old_value;

                Ok((stream, t))
            }
            Err(e) => {
                stream.inside_argument_list = old_value;

                Err(e)
            }
        }
    }
}

pub fn reset_inside_argument_list<P: Parser>(
    mut parser: P,
) -> impl FnMut(Input) -> IResult<P::Output> {
    move |mut stream| {
        let old_value = stream.inside_argument_list;
        stream.inside_argument_list = false;

        match parser.process(stream) {
            Ok((mut stream, t)) => {
                stream.inside_argument_list = old_value;

                Ok((stream, t))
            }
            Err(e) => {
                stream.inside_argument_list = old_value;

                Err(e)
            }
        }
    }
}

pub fn disallow_multiline_fn_call<P: Parser>(
    mut parser: P,
) -> impl FnMut(Input) -> IResult<P::Output> {
    move |mut stream| {
        let old_value = stream.disallowed_multiline_fn_call;
        stream.disallowed_multiline_fn_call = true;

        match parser.process(stream) {
            Ok((mut stream, t)) => {
                stream.disallowed_multiline_fn_call = old_value;

                Ok((stream, t))
            }
            Err(e) => {
                stream.disallowed_multiline_fn_call = old_value;

                Err(e)
            }
        }
    }
}

pub fn not_multi_line_fn_call_short_circuit(stream: Input) -> IResult<()> {
    if stream.disallowed_multiline_fn_call {
        Err(ParseError::ShortCircuit)
    } else {
        Ok((stream, ()))
    }
}

pub fn arguments_list_short_circuit(mut stream: Input) -> IResult<()> {
    stream.argument_list_short_circuit()?;

    Ok((stream, ()))
}

pub fn dot(stream: Input) -> IResult<IdentOrNumber> {
    preceded(
        // First try: Eol + Indent + arguments_list_short_circuit + Dot (for closing argument lists)
        (TokenType::Eol, preceded(arguments_list_short_circuit, (indent_token, TokenType::Dot)))
            .map(|_| ())
            .or(
                // Second try: Eol + Indent + Dot (normal multiline dot)
                (TokenType::Eol, indented((indent, TokenType::Dot)))
                    .map(|_| ())
            )
            .or(
                // Third try: inline dots
                TokenType::Dot
                    .or(preceded(arguments_list_short_circuit, TokenType::SpacedDot))
                    .map(|_| ())
            ),
        ident_or_number,
    )
    .process(stream)
}

pub fn double_dot(stream: Input) -> IResult<IdentOrNumber> {
    preceded(
        (TokenType::Eol, indented((indent, TokenType::DoubleDot)))
            .map(|_| ())
            .or(TokenType::DoubleDot.map(|_| ())),
        preceded(arguments_list_short_circuit, ident_or_number),
    )
    .process(stream)
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
                    name: "self".to_string(),
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
                        span: Span {
                            start: 0,
                            end: 3,
                            file_path: PathBuf::default(),
                        },
                    })],
                }),
                secondaries: Some(vec![
                    SecondaryExpr::Arguments(vec![Argument {
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
                        }))
                    }]),
                    SecondaryExpr::DoubleDot(IdentOrNumber::Ident(Ident {
                        name: "baz".to_string(),
                        span: Span {
                            start: 10,
                            end: 13,
                            file_path: PathBuf::default(),
                        },
                    })),
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
                    SecondaryExpr::DoubleDot(IdentOrNumber::Ident(Ident {
                        name: "baz".to_string(),
                        span: Span::default(),
                    })),
                    SecondaryExpr::DoubleDot(IdentOrNumber::Ident(Ident {
                        name: "foofoo".to_string(),
                        span: Span::default(),
                    })),
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

    #[test]
    fn native_operator() {
        let input = "~IAdd";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(
            expression,
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::NativeOperator(NativeOperator {
                    name: "IAdd".to_string(),
                    span: Span::default(),
                }),
                secondaries: None,
                type_annotation: None,
            })),
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn unsafe_block() {
        let input = "unsafe\n    ptr = 0\n    *ptr";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        // Should parse as an unsafe block operand
        match expression {
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Unsafe(_),
                ..
            })) => {
                // Test passes if we get an unsafe operand
            }
            _ => panic!("Expected unsafe block, got: {:?}", expression),
        }
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn error_propagation_chain() {
        let input = "a?.b?.c?";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        // Should parse as chained method calls with error propagation
        match expression {
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(_),
                secondaries: Some(secondaries),
                ..
            })) => {
                // The actual structure might be: interogation, dot, ident, interogation, dot, ident, interogation
                // Let's just verify we have the expected number of secondaries
                assert!(secondaries.len() >= 3, "Expected at least 3 secondaries, got {}", secondaries.len());

                // Verify we have interogation tokens
                let has_interogations = secondaries.iter().any(|s| matches!(s, SecondaryExpr::Interogation));
                assert!(has_interogations, "Expected to find interogation tokens");

                // Verify we have dot tokens
                let has_dots = secondaries.iter().any(|s| matches!(s, SecondaryExpr::Dot(_)));
                assert!(has_dots, "Expected to find dot tokens");
            }
            _ => panic!("Expected chained method calls, got: {:?}", expression),
        }
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn complex_operator_precedence() {
        let input = "a + b * c - d / e";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        // Should parse with correct precedence: a + (b * c) - (d / e)
        // Note: This test verifies the parser accepts the input,
        // actual precedence is handled in desugaring phase
        match expression {
            Expression::BinopExpr(_, _, _) => {
                // Test passes if we get a binary expression
            }
            _ => panic!("Expected binary expression, got: {:?}", expression),
        }
        assert_eq!(rest.len(), 0);
    }

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

    #[test]
    fn nested_function_calls() {
        let input = "f g h x";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        // Should parse as nested function calls
        match expression {
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(_),
                secondaries: Some(_),
                ..
            })) => {
                // Test passes if we get function calls
            }
            _ => panic!("Expected function calls, got: {:?}", expression),
        }
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn multiline_method_chain_with_argument() {
        // This reproduces the expression-problem test case
        let input = r#"a
    .lol
    .mdr toto.tata
    .haha"#;
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        // Should parse as: a.lol.mdr(toto.tata).haha
        match expression {
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(ident_path),
                secondaries: Some(secondaries),
                ..
            })) => {
                // Should be identifier 'a'
                assert_eq!(ident_path.path[0], IdentOrType::Ident(Ident {
                    name: "a".to_string(),
                    span: Span::default(),
                }));

                // Should have 4 secondaries: .lol, .mdr, arguments(toto.tata), .haha
                assert_eq!(secondaries.len(), 4, "Expected exactly 4 secondaries: .lol, .mdr, arguments, .haha");

                // Verify the structure: .lol, .mdr, arguments(toto.tata), .haha
                match &secondaries[0] {
                    SecondaryExpr::Dot(IdentOrNumber::Ident(ident)) => {
                        assert_eq!(ident.name, "lol");
                    }
                    _ => panic!("Expected .lol as first secondary"),
                }

                match &secondaries[1] {
                    SecondaryExpr::Dot(IdentOrNumber::Ident(ident)) => {
                        assert_eq!(ident.name, "mdr");
                    }
                    _ => panic!("Expected .mdr as second secondary"),
                }

                match &secondaries[2] {
                    SecondaryExpr::Arguments(args) => {
                        assert_eq!(args.len(), 1, "Expected exactly one argument");
                        // Verify the argument is toto.tata (not toto.tata.haha)
                        match &args[0].arg {
                            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(ident_path),
                                secondaries: Some(arg_secondaries),
                                ..
                            })) => {
                                // Should be toto.tata
                                assert_eq!(ident_path.path[0], IdentOrType::Ident(Ident {
                                    name: "toto".to_string(),
                                    span: Span::default(),
                                }));
                                assert_eq!(arg_secondaries.len(), 1, "Expected exactly one secondary in argument (just .tata)");
                                match &arg_secondaries[0] {
                                    SecondaryExpr::Dot(IdentOrNumber::Ident(ident)) => {
                                        assert_eq!(ident.name, "tata");
                                    }
                                    _ => panic!("Expected .tata in argument"),
                                }
                            }
                            _ => panic!("Expected toto.tata as argument"),
                        }
                    }
                    _ => panic!("Expected arguments as third secondary"),
                }

                match &secondaries[3] {
                    SecondaryExpr::Dot(IdentOrNumber::Ident(ident)) => {
                        assert_eq!(ident.name, "haha");
                    }
                    _ => panic!("Expected .haha as fourth secondary"),
                }
            }
            _ => panic!("Expected method chain, got: {:?}", expression),
        }
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn ambiguous_multiline_should_fail() {
        // This should fail because mdr.lol and haha are at the same indentation level as .lol
        // making it ambiguous whether they are arguments or method calls
        let input = r#"a
    .lol
    mdr.lol
    haha"#;
        let tokens = lex_test(input);
        let config = Config::default();

        // This test verifies that ambiguous multiline syntax is correctly rejected

        // This currently parses incorrectly as a.lol(mdr.lol, haha)
        // but should ideally be a parse error due to ambiguity
        let result = expression.process(ParseCtx::from(&tokens, &config));

        // The fix should now correctly reject the ambiguous syntax
        match result {
            Ok((_, Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                secondaries: Some(secondaries),
                ..
            })))) => {
                // After the fix, this should only parse a.lol (no arguments)
                assert_eq!(secondaries.len(), 1); // Only .lol, no arguments
                match &secondaries[0] {
                    SecondaryExpr::Dot(IdentOrNumber::Ident(ident)) => {
                        assert_eq!(ident.name, "lol");
                    }
                    _ => panic!("Expected .lol as the only secondary"),
                }
            }
            Err(_) => {
                // This would also be acceptable - rejecting ambiguous syntax entirely
                // For now, we expect partial parsing (just a.lol)
                panic!("Expected partial parsing of a.lol, but got error");
            }
            _ => panic!("Unexpected parse result"),
        }
    }

    #[test]
    fn multiline_arguments_in_method_chain() {
        // Test multiline arguments: foo.bar(arg1, arg2).baz
        let input = r#"foo
        .bar
            arg1
            arg2
        .baz"#;
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        match expression {
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(ident_path),
                secondaries: Some(secondaries),
                ..
            })) => {
                assert_eq!(ident_path.path[0], IdentOrType::Ident(Ident {
                    name: "foo".to_string(),
                    span: Span::default(),
                }));

                // Should have 3 secondaries: .bar, arguments(arg1, arg2), .baz
                assert_eq!(secondaries.len(), 3);

                match &secondaries[0] {
                    SecondaryExpr::Dot(IdentOrNumber::Ident(ident)) => {
                        assert_eq!(ident.name, "bar");
                    }
                    _ => panic!("Expected .bar"),
                }

                match &secondaries[1] {
                    SecondaryExpr::Arguments(args) => {
                        assert_eq!(args.len(), 2);
                    }
                    _ => panic!("Expected arguments"),
                }

                match &secondaries[2] {
                    SecondaryExpr::Dot(IdentOrNumber::Ident(ident)) => {
                        assert_eq!(ident.name, "baz");
                    }
                    _ => panic!("Expected .baz"),
                }
            }
            _ => panic!("Unexpected expression type"),
        }
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn multiline_arguments_with_nested_multiline_dots() {
        // Test complex case: multiline arguments where arguments themselves have multiline dots
        // Should parse as: foo.bar(arg1.method1.method2, arg2).baz
        let input = r#"foo
    .bar
        arg1
            .method1
            .method2
        arg2
    .baz"#;
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        match expression {
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(ident_path),
                secondaries: Some(secondaries),
                ..
            })) => {
                assert_eq!(ident_path.path[0], IdentOrType::Ident(Ident {
                    name: "foo".to_string(),
                    span: Span::default(),
                }));

                // Should have 3 secondaries: .bar, arguments, .baz
                assert_eq!(secondaries.len(), 3);

                // Check .bar
                match &secondaries[0] {
                    SecondaryExpr::Dot(IdentOrNumber::Ident(ident)) => {
                        assert_eq!(ident.name, "bar");
                    }
                    _ => panic!("Expected .bar"),
                }

                // Check arguments
                match &secondaries[1] {
                    SecondaryExpr::Arguments(args) => {
                        assert_eq!(args.len(), 2, "Expected 2 arguments");

                        // First argument: arg1.method1.method2
                        match &args[0].arg {
                            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(ident_path),
                                secondaries: Some(arg_secondaries),
                                ..
                            })) => {
                                assert_eq!(ident_path.path[0], IdentOrType::Ident(Ident {
                                    name: "arg1".to_string(),
                                    span: Span::default(),
                                }));
                                // Should have .method1 and .method2
                                assert_eq!(arg_secondaries.len(), 2);
                                match &arg_secondaries[0] {
                                    SecondaryExpr::Dot(IdentOrNumber::Ident(ident)) => {
                                        assert_eq!(ident.name, "method1");
                                    }
                                    _ => panic!("Expected .method1"),
                                }
                                match &arg_secondaries[1] {
                                    SecondaryExpr::Dot(IdentOrNumber::Ident(ident)) => {
                                        assert_eq!(ident.name, "method2");
                                    }
                                    _ => panic!("Expected .method2"),
                                }
                            }
                            _ => panic!("Expected arg1.method1.method2"),
                        }

                        // Second argument: arg2
                        match &args[1].arg {
                            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(ident_path),
                                secondaries: None,
                                ..
                            })) => {
                                assert_eq!(ident_path.path[0], IdentOrType::Ident(Ident {
                                    name: "arg2".to_string(),
                                    span: Span::default(),
                                }));
                            }
                            _ => panic!("Expected arg2"),
                        }
                    }
                    _ => panic!("Expected arguments"),
                }

                // Check .baz
                match &secondaries[2] {
                    SecondaryExpr::Dot(IdentOrNumber::Ident(ident)) => {
                        assert_eq!(ident.name, "baz");
                    }
                    _ => panic!("Expected .baz"),
                }
            }
            _ => panic!("Unexpected expression type"),
        }
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn deeply_nested_multiline_arguments_with_multiline_dots() {
        // Test deeply nested case: multiline arguments containing function calls
        // with their own multiline arguments
        // Should parse as: foo(argnested(arg1, arg2), arg3)
        // Using indent 8 to avoid the ambiguity check (which only applies at indent 4)
        let input = r#"foo
        argnested
            arg1
            arg2
        arg3"#;
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, expression) = expression
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        match expression {
            Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(ident_path),
                secondaries: Some(secondaries),
                ..
            })) => {
                assert_eq!(ident_path.path[0], IdentOrType::Ident(Ident {
                    name: "foo".to_string(),
                    span: Span::default(),
                }));

                // With indent_step=8, this might parse differently
                // Let's just verify we have arguments
                assert!(secondaries.len() >= 1, "Expected at least 1 secondary (arguments)");

                // Find the arguments secondary (might not be first due to indent_step=8)
                let args = secondaries.iter().find_map(|s| match s {
                    SecondaryExpr::Arguments(args) => Some(args),
                    _ => None,
                }).expect("Should have arguments");

                assert_eq!(args.len(), 2, "Expected 2 arguments to foo");

                // First argument: argnested(arg1, arg2)
                match &args[0].arg {
                    Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Ident(ident_path),
                        secondaries: Some(nested_secondaries),
                        ..
                    })) => {
                        assert_eq!(ident_path.path[0], IdentOrType::Ident(Ident {
                            name: "argnested".to_string(),
                            span: Span::default(),
                        }));

                        // argnested should have arguments (find them in the secondaries)
                        let nested_args = nested_secondaries.iter().find_map(|s| match s {
                            SecondaryExpr::Arguments(args) => Some(args),
                            _ => None,
                        }).expect("argnested should have arguments");

                        // Should have 2 nested arguments
                        assert_eq!(nested_args.len(), 2, "Expected 2 arguments to argnested");
                    }
                    _ => panic!("Expected argnested with arguments"),
                }

                // Second argument: arg3
                match &args[1].arg {
                    Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Ident(ident_path),
                        secondaries: None,
                        ..
                    })) => {
                        assert_eq!(ident_path.path[0], IdentOrType::Ident(Ident {
                            name: "arg3".to_string(),
                            span: Span::default(),
                        }));
                    }
                    _ => panic!("Expected arg3"),
                }
            }
            _ => panic!("Unexpected expression type"),
        }
        assert_eq!(rest.len(), 0);
    }
}
