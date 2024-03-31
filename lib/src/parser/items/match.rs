use crate::{
    ast::{Block, Expression, Ident, Match, MatchArm, MatchPattern},
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
        println!("remaining_tokens: {:?}", tokens);
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("match".to_string()))?;

        let (expr, remaining_tokens) = Expression::parse(remaining_tokens, parse_ctx)?;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        parse_ctx.indent();

        let (arms, remaining_tokens) =
            parse_vec_of(remaining_tokens, Some(TokenType::Eol), parse_ctx)?;

        parse_ctx.dedent();

        Ok((Match { expr, arms }, remaining_tokens))
    }
}

impl Parsable for MatchArm {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = parse_ctx.consume_indent(tokens)?;

        let (pattern, remaining_tokens) = MatchPattern::parse(remaining_tokens, parse_ctx)?;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Arrow)?;

        let (body, remaining_tokens) = Block::parse(remaining_tokens, parse_ctx)?;

        Ok((MatchArm { pattern, body }, remaining_tokens))
    }
}

impl Parsable for MatchPattern {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        if let TokenType::OpenParen = tokens[0].token_type {
            let (patterns, remaining_tokens) =
                parse_vec_of(&tokens[1..], Some(TokenType::Coma), parse_ctx)?;

            let remaining_tokens = expect_token(remaining_tokens, TokenType::CloseParen)?;

            Ok((MatchPattern::Tuple(patterns), remaining_tokens))
        } else if let TokenType::Ident(_) = tokens[0].token_type {
            let (ident, remaining_tokens) = Ident::parse(tokens, parse_ctx)?;

            Ok((MatchPattern::Ident(ident), remaining_tokens))
        } else if let TokenType::Underscore = tokens[0].token_type {
            Ok((MatchPattern::Wildcard, &tokens[1..]))
        } else {
            return Err(ParseError::UnexpectedToken(
                tokens[0].clone(),
                vec![
                    TokenType::OpenParen,
                    TokenType::Ident("".to_string()),
                    TokenType::Underscore,
                ],
            )
            .into());
        }
    }
}
#[cfg(test)]
mod r#match {
    use super::*;
    use crate::{
        ast::{
            IdentOrType, Literal, LiteralKind, Operand, Operator, PrimaryExpr, Statement, UnaryExpr,
        },
        lexer::Span,
        parser::util::lex_test,
        Config,
    };

    #[test]
    fn test_parse_match() {
        let input = r#"match a
  a -> 2
  (a, b) -> a + b"#;
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
                    secondaries: None
                })),
                arms: vec![
                    MatchArm {
                        pattern: MatchPattern::Ident(Ident {
                            name: "a".to_string(),
                            span: Span::default(),
                        }),
                        body: Block {
                            statements: vec![Statement::Expression(Expression::UnaryExpr(
                                UnaryExpr::PrimaryExpr(PrimaryExpr {
                                    operand: Operand::Literal(Literal {
                                        kind: LiteralKind::Number(2),
                                        span: Span::default()
                                    }),
                                    secondaries: None
                                })
                            ))]
                        }
                    },
                    MatchArm {
                        pattern: MatchPattern::Tuple(vec![
                            MatchPattern::Ident(Ident {
                                name: "a".to_string(),
                                span: Span::default(),
                            }),
                            MatchPattern::Ident(Ident {
                                name: "b".to_string(),
                                span: Span::default(),
                            }),
                        ]),
                        body: Block {
                            statements: vec![Statement::Expression(Expression::BinopExpr(
                                UnaryExpr::PrimaryExpr(PrimaryExpr {
                                    operand: Operand::Ident(crate::ast::IdentifierPath {
                                        path: vec![IdentOrType::Ident(Ident {
                                            name: "a".to_string(),
                                            span: Span::default(),
                                        })]
                                    }),
                                    secondaries: None
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
                                        secondaries: None
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
}
