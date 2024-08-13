use crate::new_parser::{engine::*, items::primitives, Literal, LiteralKind};

use super::{array, utils::get_span};

pub fn literal(stream: Input) -> IResult<Literal> {
    let (stream, span) = get_span(stream)?;

    primitives::boolean
        .map(LiteralKind::Bool)
        .or(primitives::int.map(LiteralKind::Number))
        .or(primitives::float.map(LiteralKind::Float))
        .or(array.map(LiteralKind::Array))
        .or(primitives::string.map(LiteralKind::String))
        .or(primitives::char.map(LiteralKind::Char))
        .map(|kind| Literal {
            kind,
            span: span.clone(),
        })
        .process(stream)
}

#[cfg(test)]
mod literals {

    use super::*;
    use crate::{
        ast::{Ident, IdentOrType, IdentifierPath, Operand, Operator, PrimaryExpr, UnaryExpr},
        lexer::Span,
        new_parser::{lex_test, Array, Expression},
        Config,
    };

    fn parse_literal(input: &str) -> Literal {
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, literal) = literal.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(rest.len(), 0);

        literal
    }

    #[test]
    fn test_parse_number() {
        let input = "123";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::Number(123));
    }

    #[test]
    fn test_parse_string() {
        let input = "\"hello\"";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::String("hello".to_owned()));
    }

    #[test]
    fn test_parse_char() {
        let input = "'a'";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::Char("a".to_owned()));
    }

    #[test]
    fn test_parse_float() {
        let input = "123.456";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::Float(123.456));
    }

    #[test]
    fn test_parse_bool() {
        let input = "true";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::Bool(true));
    }

    #[test]
    fn test_parse_array() {
        let input = "[1, 2, 3]";
        let literal = parse_literal(input);

        assert_eq!(
            literal.kind,
            LiteralKind::Array(Array {
                elements: vec![
                    Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: LiteralKind::Number(1),
                            span: Span::default(),
                        }),
                        secondaries: None,
                        type_annotation: None,
                    })),
                    Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: LiteralKind::Number(2),
                            span: Span::default(),
                        }),
                        type_annotation: None,
                        secondaries: None,
                    })),
                    Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: LiteralKind::Number(3),
                            span: Span::default(),
                        }),
                        secondaries: None,
                        type_annotation: None,
                    })),
                ],
            })
        );
    }

    #[test]
    fn test_parse_array_empty() {
        let input = "[]";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::Array(Array { elements: vec![] }));
    }

    #[test]
    fn test_parse_array_nested_expr() {
        let input = "[1, [hello, 3], 8, 5 + 4]";
        let literal = parse_literal(input);

        let expected = LiteralKind::Array(Array {
            elements: vec![
                Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: LiteralKind::Number(1),
                        span: Span::default(),
                    }),
                    secondaries: None,
                    type_annotation: None,
                })),
                Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        span: Span::default(),
                        kind: LiteralKind::Array(Array {
                            elements: vec![
                                Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                    operand: Operand::Ident(IdentifierPath {
                                        path: vec![IdentOrType::Ident(Ident {
                                            name: "hello".to_string(),
                                            span: Span::default(),
                                        })],
                                    }),
                                    secondaries: None,
                                    type_annotation: None,
                                })),
                                Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                    operand: Operand::Literal(Literal {
                                        kind: LiteralKind::Number(3),
                                        span: Span::default(),
                                    }),
                                    secondaries: None,
                                    type_annotation: None,
                                })),
                            ],
                        }),
                    }),
                    secondaries: None,
                    type_annotation: None,
                })),
                Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: LiteralKind::Number(8),
                        span: Span::default(),
                    }),
                    secondaries: None,
                    type_annotation: None,
                })),
                Expression::BinopExpr(
                    UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: LiteralKind::Number(5),
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
                            kind: LiteralKind::Number(4),
                            span: Span::default(),
                        }),
                        secondaries: None,
                        type_annotation: None,
                    }))),
                ),
            ],
        });

        assert_eq!(literal.kind, expected,);
    }
}
