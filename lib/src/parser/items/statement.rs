use crate::{
    ast::{Assignment, AssignmentLHS, Expression, Pattern, Statement, UnaryExpr},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, ParseError},
    },
};

impl Parsable for Statement {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        if tokens.len() == 0 {
            return Err(ParseError::UnexpectedEof(TokenType::Eol).into());
        }
        let (_, remaining_tokens) = parse_ctx.consume_indent_if_any(tokens);

        if TokenType::Keyword("return".to_string()) == remaining_tokens[0].token_type {
            if let Ok((expression, remaining_tokens)) =
                Expression::parse(&remaining_tokens[1..], parse_ctx)
            {
                return Ok((Statement::Return(Some(expression)), remaining_tokens));
            } else {
                return Ok((Statement::Return(None), &remaining_tokens[1..]));
            }
        }

        if TokenType::Keyword("continue".to_string()) == remaining_tokens[0].token_type {
            if let Ok((expression, remaining_tokens)) =
                Expression::parse(&remaining_tokens[1..], parse_ctx)
            {
                return Ok((Statement::Continue(Some(expression)), remaining_tokens));
            } else {
                return Ok((Statement::Continue(None), &remaining_tokens[1..]));
            }
        }

        if TokenType::Keyword("break".to_string()) == remaining_tokens[0].token_type {
            if let Ok((expression, remaining_tokens)) =
                Expression::parse(&remaining_tokens[1..], parse_ctx)
            {
                return Ok((Statement::Break(Some(expression)), remaining_tokens));
            } else {
                return Ok((Statement::Break(None), &remaining_tokens[1..]));
            }
        }
        let assignment_res = Assignment::parse(remaining_tokens, parse_ctx);

        let diagnostics = match assignment_res {
            Ok((assignment, remaining_tokens)) => {
                return Ok((Statement::Assignment(assignment), remaining_tokens))
            }
            Err(diags) => diags,
        };

        let (expression, remaining_tokens) = Expression::parse(remaining_tokens, parse_ctx)?;

        if !remaining_tokens.is_empty() && remaining_tokens[0].token_type == TokenType::Equal {
            return Err(diagnostics);
        }

        Ok((Statement::Expression(expression), remaining_tokens))
    }
}

impl Parsable for Assignment {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let (lhs, remaining_tokens) = AssignmentLHS::parse(tokens, parse_ctx)?;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Equal)?;

        let (rhs, remaining_tokens) = Expression::parse(remaining_tokens, parse_ctx)?;

        Ok((Assignment { lhs, rhs }, remaining_tokens))
    }
}

impl Parsable for AssignmentLHS {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        if let Ok((pattern, remaining_tokens)) = Pattern::parse(tokens, parse_ctx) {
            if remaining_tokens.len() > 0 && remaining_tokens[0].token_type == TokenType::Equal {
                return Ok((AssignmentLHS::Pattern(pattern), remaining_tokens));
            }
        }

        if let Ok((unary, remaining_tokens)) = UnaryExpr::parse(tokens, parse_ctx) {
            if remaining_tokens.len() > 0 && remaining_tokens[0].token_type == TokenType::Equal {
                return Ok((AssignmentLHS::Expression(unary), remaining_tokens));
            }
        }

        Err(ParseError::InvalidLHS(tokens[0].span.clone()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{
            Ident, IdentOrNumber, IdentOrType, IdentifierPath, Literal, Operand, PatternKind,
            PrimaryExpr, SecondaryExpr, UnaryExpr,
        },
        lexer::Span,
        parser::util::lex_test,
        Config,
    };

    #[test]
    fn test_parse_statement() {
        let input = "1";
        let tokens = lex_test(input);
        let (statement, rest) =
            Statement::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            statement,
            Statement::Expression(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Literal(Literal {
                    kind: crate::ast::LiteralKind::Number(1),
                    span: Span::default(),
                }),
                secondaries: None,
                type_annotation: None,
            })))
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_assignment() {
        let input = "a = 1";
        let tokens = lex_test(input);
        let (statement, rest) =
            Statement::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            statement,
            Statement::Assignment(Assignment {
                lhs: AssignmentLHS::Pattern(Pattern {
                    binding: None,
                    kind: PatternKind::Ident(Ident {
                        name: "a".to_string(),
                        span: Span::default(),
                    })
                }),
                rhs: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(1),
                        span: Span::default(),
                    }),
                    secondaries: None,
                    type_annotation: None,
                })),
            })
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_assignment_complex() {
        let input = "a.b[2].c = 1";
        let tokens = lex_test(input);
        let (statement, rest) =
            Statement::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            statement,
            Statement::Assignment(Assignment {
                lhs: AssignmentLHS::Expression(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Ident(IdentifierPath {
                        path: vec![IdentOrType::Ident(Ident {
                            name: "a".to_string(),
                            span: Span::default(),
                        })],
                    }),
                    secondaries: Some(vec![
                        SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                            name: "b".to_string(),
                            span: Span::default(),
                        })),
                        SecondaryExpr::Indice(Box::new(Expression::UnaryExpr(
                            UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Literal(Literal {
                                    kind: crate::ast::LiteralKind::Number(2),
                                    span: Span::default(),
                                }),
                                secondaries: None,
                                type_annotation: None,
                            })
                        )),),
                        SecondaryExpr::Dot(IdentOrNumber::Ident(Ident {
                            name: "c".to_string(),
                            span: Span::default(),
                        })),
                    ]),
                    type_annotation: None,
                })),
                rhs: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(1),
                        span: Span::default(),
                    }),
                    secondaries: None,
                    type_annotation: None,
                })),
            })
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_return() {
        let input = "return 1";
        let tokens = lex_test(input);
        let (statement, rest) =
            Statement::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            statement,
            Statement::Return(Some(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(
                PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(1),
                        span: Span::default(),
                    }),
                    secondaries: None,
                    type_annotation: None,
                }
            ))))
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_continue() {
        let input = "continue 1";
        let tokens = lex_test(input);
        let (statement, rest) =
            Statement::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            statement,
            Statement::Continue(Some(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(
                PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(1),
                        span: Span::default(),
                    }),
                    secondaries: None,
                    type_annotation: None,
                }
            ))))
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_break() {
        let input = "break 1";
        let tokens = lex_test(input);
        let (statement, rest) =
            Statement::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(
            statement,
            Statement::Break(Some(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(
                PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(1),
                        span: Span::default(),
                    }),
                    secondaries: None,
                    type_annotation: None,
                }
            ))))
        );

        assert_eq!(rest.len(), 0);
    }
}
