use crate::{
    ast::{Array, Expression, Literal, LiteralKind},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{consume_tokens_until, expect_token, parse_vec_of, ParseError},
    },
};

impl Parsable for Literal {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let mut tokens = tokens;

        let token = tokens
            .get(0)
            .ok_or(ParseError::UnexpectedEof(TokenType::Number("".to_string())))?;

        let literal_kind = match &token.token_type {
            TokenType::Keyword(value) if value == "true" || value == "false" => {
                tokens = &tokens[1..];
                LiteralKind::Bool(value.parse().unwrap())
            }
            TokenType::Number(value) => {
                tokens = &tokens[1..];
                LiteralKind::Number(value.parse().unwrap())
            }
            TokenType::Float(value) => {
                tokens = &tokens[1..];
                LiteralKind::Float(value.parse().unwrap())
            }
            TokenType::DoubleQuote => {
                let (string, remaining_tokens) = String::parse(tokens, parse_ctx)?;
                tokens = remaining_tokens;
                LiteralKind::String(string)
            }
            TokenType::SimpleQuote => {
                let (char, remaining_tokens) = char::parse(tokens, parse_ctx)?;
                tokens = remaining_tokens;
                LiteralKind::Char(char)
            }
            TokenType::OpenBracket => {
                let (array, remaining_tokens) =
                    parse_vec_of::<Expression>(&tokens[1..], Some(TokenType::Coma), parse_ctx)?;
                let remaining_tokens = expect_token(remaining_tokens, TokenType::CloseBracket)?;
                tokens = remaining_tokens;
                LiteralKind::Array(Array { elements: array })
            }
            _ => {
                return Err(ParseError::UnexpectedToken(
                    token.clone(),
                    vec![TokenType::Number("".to_string())],
                )
                .into())
            }
        };

        Ok((
            Literal {
                kind: literal_kind,
                span: token.span.clone(),
            },
            &tokens,
        ))
    }
}

impl Parsable for String {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = expect_token(tokens, TokenType::DoubleQuote)?;

        let (inner_tokens, remaining_tokens) =
            consume_tokens_until(remaining_tokens, TokenType::DoubleQuote);

        let remaining_tokens = expect_token(remaining_tokens, TokenType::DoubleQuote)?;

        let string = inner_tokens
            .iter()
            .map(|token| token.token_type.to_string())
            .collect();

        Ok((string, remaining_tokens))
    }
}

impl Parsable for char {
    fn parse<'a>(
        tokens: &'a [Token],
        _parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let mut remaining_tokens = expect_token(tokens, TokenType::SimpleQuote)?;

        let inner_token = remaining_tokens[0].clone();

        if format!("{}", inner_token.token_type.to_string()).len() > 1 {
            return Err(
                ParseError::UnexpectedToken(inner_token, vec![TokenType::SimpleQuote]).into(),
            );
        }
        remaining_tokens = &remaining_tokens[1..];

        let remaining_tokens = expect_token(remaining_tokens, TokenType::SimpleQuote)?;

        Ok((
            inner_token.token_type.to_string().chars().next().unwrap(),
            remaining_tokens,
        ))
    }
}

#[cfg(test)]
mod literals {

    use super::*;
    use crate::{
        ast::{Ident, IdentOrType, IdentifierPath, Operand, Operator, PrimaryExpr, UnaryExpr},
        lexer::Span,
        parser::util::lex_test,
        Config,
    };

    fn parse_literal(input: &str) -> Literal {
        let tokens = lex_test(input);
        let (literal, remaining_tokens) =
            Literal::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(remaining_tokens.len(), 0);

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

        assert_eq!(literal.kind, LiteralKind::String("hello".to_string()));
    }

    #[test]
    fn test_parse_char() {
        let input = "'a'";
        let literal = parse_literal(input);

        assert_eq!(literal.kind, LiteralKind::Char('a'));
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
                    })),
                    Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: LiteralKind::Number(2),
                            span: Span::default(),
                        }),
                        secondaries: None,
                    })),
                    Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: LiteralKind::Number(3),
                            span: Span::default(),
                        }),
                        secondaries: None,
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
                                })),
                                Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                                    operand: Operand::Literal(Literal {
                                        kind: LiteralKind::Number(3),
                                        span: Span::default(),
                                    }),
                                    secondaries: None,
                                })),
                            ],
                        }),
                    }),
                    secondaries: None,
                })),
                Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: LiteralKind::Number(8),
                        span: Span::default(),
                    }),
                    secondaries: None,
                })),
                Expression::BinopExpr(
                    UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: LiteralKind::Number(5),
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
                            kind: LiteralKind::Number(4),
                            span: Span::default(),
                        }),
                        secondaries: None,
                    }))),
                ),
            ],
        });

        assert_eq!(literal.kind, expected,);
    }
}
