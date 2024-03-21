use crate::{
    ast::{Assignment, Expression, Statement},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, ignore_empty_lines, ParseError},
    },
};

impl Parsable for Statement {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let remaining_tokens = ignore_empty_lines(tokens);
        let (_, remaining_tokens) = parse_ctx.consume_indent_if_any(remaining_tokens);

        if TokenType::Keyword("return".to_string()) == remaining_tokens[0].token_type {
            let (expression, remaining_tokens) =
                Expression::parse(&remaining_tokens[1..], parse_ctx)?;

            return Ok((Statement::Return(expression), remaining_tokens));
        }

        if TokenType::Keyword("continue".to_string()) == remaining_tokens[0].token_type {
            let (expression, remaining_tokens) =
                Expression::parse(&remaining_tokens[1..], parse_ctx)?;

            return Ok((Statement::Continue(expression), remaining_tokens));
        }

        if TokenType::Keyword("break".to_string()) == remaining_tokens[0].token_type {
            let (expression, remaining_tokens) =
                Expression::parse(&remaining_tokens[1..], parse_ctx)?;

            return Ok((Statement::Break(expression), remaining_tokens));
        }

        let (expression, remaining_tokens) = Expression::parse(remaining_tokens, parse_ctx)?;

        if remaining_tokens.is_empty() {
            return Ok((Statement::Expression(expression), remaining_tokens));
        }

        if let TokenType::Equal = remaining_tokens[0].token_type {
            let remaining_tokens = expect_token(remaining_tokens, TokenType::Equal)?;
            let (rhs, remaining_tokens) = Expression::parse(remaining_tokens, parse_ctx)?;

            return Ok((
                Statement::Assignment(Assignment {
                    lhs: expression,
                    rhs,
                }),
                remaining_tokens,
            ));
        }

        Ok((Statement::Expression(expression), remaining_tokens))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        ast::{
            Ident, IdentOrType, IdentifierPath, Literal, Operand, PrimaryExpr, SecondaryExpr,
            UnaryExpr,
        },
        lexer::Span,
        parser::util::lex_test,
    };

    #[test]
    fn test_parse_statement() {
        let input = "1";
        let tokens = lex_test(input);
        let (statement, rest) = Statement::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            statement,
            Statement::Expression(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Literal(Literal {
                    kind: crate::ast::LiteralKind::Number(1),
                    span: Span::default(),
                }),
                secondaries: None,
            })))
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_assignment() {
        let input = "a = 1";
        let tokens = lex_test(input);
        let (statement, rest) = Statement::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            statement,
            Statement::Assignment(Assignment {
                lhs: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Ident(crate::ast::IdentifierPath {
                        path: vec![IdentOrType::Ident(crate::ast::Ident {
                            name: "a".to_string(),
                            span: Span::default(),
                        })],
                    }),
                    secondaries: None,
                })),
                rhs: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(1),
                        span: Span::default(),
                    }),
                    secondaries: None,
                })),
            })
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_assignment_complex() {
        let input = "a.b[2].c = 1";
        let tokens = lex_test(input);
        let (statement, rest) = Statement::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            statement,
            Statement::Assignment(Assignment {
                lhs: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Ident(IdentifierPath {
                        path: vec![IdentOrType::Ident(Ident {
                            name: "a".to_string(),
                            span: Span::default(),
                        })],
                    }),
                    secondaries: Some(vec![
                        SecondaryExpr::Dot(Ident {
                            name: "b".to_string(),
                            span: Span::default(),
                        }),
                        SecondaryExpr::Indice(Box::new(Expression::UnaryExpr(
                            UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Literal(Literal {
                                    kind: crate::ast::LiteralKind::Number(2),
                                    span: Span::default(),
                                }),
                                secondaries: None,
                            })
                        )),),
                        SecondaryExpr::Dot(Ident {
                            name: "c".to_string(),
                            span: Span::default(),
                        }),
                    ]),
                })),
                rhs: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Literal(Literal {
                        kind: crate::ast::LiteralKind::Number(1),
                        span: Span::default(),
                    }),
                    secondaries: None,
                })),
            })
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_return() {
        let input = "return 1";
        let tokens = lex_test(input);
        let (statement, rest) = Statement::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            statement,
            Statement::Return(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Literal(Literal {
                    kind: crate::ast::LiteralKind::Number(1),
                    span: Span::default(),
                }),
                secondaries: None,
            })))
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_continue() {
        let input = "continue 1";
        let tokens = lex_test(input);
        let (statement, rest) = Statement::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            statement,
            Statement::Continue(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Literal(Literal {
                    kind: crate::ast::LiteralKind::Number(1),
                    span: Span::default(),
                }),
                secondaries: None,
            })))
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_break() {
        let input = "break 1";
        let tokens = lex_test(input);
        let (statement, rest) = Statement::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            statement,
            Statement::Break(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Literal(Literal {
                    kind: crate::ast::LiteralKind::Number(1),
                    span: Span::default(),
                }),
                secondaries: None,
            })))
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_statement_with_emptynewlines() {
        let input = "\n    \n1";
        let mut tokens = lex_test(input);

        // Add back the first indent
        tokens.insert(
            0,
            Token {
                token_type: TokenType::Indent(0),
                span: Span::default(),
            },
        );

        let (statement, rest) = Statement::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(
            statement,
            Statement::Expression(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Literal(Literal {
                    kind: crate::ast::LiteralKind::Number(1),
                    span: Span::default(),
                }),
                secondaries: None,
            })))
        );

        assert_eq!(rest.len(), 0);
    }
}
