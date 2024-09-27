use crate::{
    lexer::TokenType,
    new_parser::{engine::*, Assignment, AssignmentLHS, Statement},
};

use super::{expression, pattern, seek, unary_expr};

pub fn statement(stream: Input) -> IResult<Statement> {
    preceded(TokenType::Keyword("return".to_string()), expression.opt())
        .map(Statement::Return)
        .or(
            preceded(TokenType::Keyword("continue".to_string()), expression.opt())
                .map(Statement::Continue),
        )
        .or(
            preceded(TokenType::Keyword("break".to_string()), expression.opt())
                .map(Statement::Break),
        )
        .or(assignment.map(Statement::Assignment))
        .or(expression.map(Statement::Expression))
        .process(stream)
}

pub fn assignment(stream: Input) -> IResult<Assignment> {
    (assignment_lhs, TokenType::Equal, expression)
        .map(|(lhs, _, rhs)| Assignment { lhs, rhs })
        .process(stream)
}

pub fn assignment_lhs(stream: Input) -> IResult<AssignmentLHS> {
    followed(pattern, seek(TokenType::Equal))
        .map(AssignmentLHS::Pattern)
        .or(followed(unary_expr, seek(TokenType::Equal)).map(AssignmentLHS::Expression))
        .process(stream)
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
        new_parser::{lex_test, Assignment, AssignmentLHS, Expression, IdentPattern, Pattern},
        Config,
    };

    #[test]
    fn test_parse_statement() {
        let input = "1";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, statement) = statement.process(ParseCtx::from(&tokens, &config)).unwrap();

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
        let config = Config::default();

        let (rest, statement) = statement.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            statement,
            Statement::Assignment(Assignment {
                lhs: AssignmentLHS::Pattern(Pattern {
                    binding: None,
                    kind: PatternKind::Ident(IdentPattern {
                        name: Ident {
                            name: "a".to_string(),
                            span: Span::default(),
                        },
                        mut_: false,
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
        let config = Config::default();

        let (rest, statement) = statement.process(ParseCtx::from(&tokens, &config)).unwrap();

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
        let config = Config::default();

        let (rest, statement) = statement.process(ParseCtx::from(&tokens, &config)).unwrap();

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
        let config = Config::default();

        let (rest, statement) = statement.process(ParseCtx::from(&tokens, &config)).unwrap();

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
        let config = Config::default();

        let (rest, statement) = statement.process(ParseCtx::from(&tokens, &config)).unwrap();

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
