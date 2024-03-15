use crate::{
    ast::{Expression, IdentifierPath, Literal, Operand, Operator, PrimaryExpr, UnaryExpr},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, ParseError},
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
        if let TokenType::Operator(_) = token.token_type {
            let (operator, remaining_tokens) = Operator::parse(remaining_tokens, parse_ctx)?;
            let (expression, remaining_tokens) = Expression::parse(remaining_tokens, parse_ctx)?;

            Ok((
                Expression::BinopExpr(unary_expr, operator, Box::new(expression)),
                remaining_tokens,
            ))
        } else {
            Ok((Expression::UnaryExpr(unary_expr), remaining_tokens))
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

        if let TokenType::Operator(_) = token.token_type {
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

        let primary_expr = PrimaryExpr {
            operand,
            secondaries: None,
        };

        Ok((primary_expr, remaining_tokens))
    }
}

impl Parsable for Operand {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        if TokenType::OpenParen == tokens[0].token_type {
            let (expression, remaining_tokens) = Expression::parse(&tokens[1..], parse_ctx)?;
            let remaining_tokens = expect_token(remaining_tokens, TokenType::CloseParen)?;
            return Ok((Operand::Expression(Box::new(expression)), remaining_tokens));
        }
        if let Ok((literal, remaining_tokens)) = Literal::parse(tokens, parse_ctx) {
            return Ok((Operand::Literal(literal), remaining_tokens));
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
            TokenType::Operator(value) => Ok((
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
mod tests {

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
}
