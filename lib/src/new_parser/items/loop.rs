use crate::{
    lexer::TokenType,
    new_parser::{engine::*, Loop},
};

use super::{block, disallow_multiline_fn_call, expression, ident};

pub fn r#loop(stream: Input) -> IResult<Loop> {
    raw_loop.or(r#for).or(r#while).process(stream)
}

pub fn raw_loop(stream: Input) -> IResult<Loop> {
    (TokenType::Keyword("loop".to_string()), block)
        .map(|(_, body)| Loop::Loop(body))
        .process(stream)
}

pub fn r#while(stream: Input) -> IResult<Loop> {
    (
        TokenType::Keyword("while".to_string()),
        disallow_multiline_fn_call(expression),
        block,
    )
        .map(|(_, condition, body)| Loop::While(condition, body))
        .process(stream)
}

pub fn r#for(stream: Input) -> IResult<Loop> {
    (
        TokenType::Keyword("for".to_string()),
        ident,
        TokenType::Keyword("in".to_string()),
        disallow_multiline_fn_call(expression),
        block,
    )
        .map(|(_, ident, _, expr, body)| Loop::For(ident, expr, body))
        .process(stream)
}

#[cfg(test)]
mod parse_loop {
    use super::*;
    use crate::{
        ast::{
            Block, Expression, Ident, IdentOrType, IdentifierPath, Literal, LiteralKind, Loop,
            Operand, PrimaryExpr, Statement, UnaryExpr,
        },
        new_parser::{engine::*, lex_test},
        Config,
    };

    #[test]
    fn parse_for() {
        let input = "for x in y\n    2";
        let tokens = lex_test(input);
        let config = Config::default();

        let (remaining, loop_) = r#loop.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            loop_,
            Loop::For(
                Ident {
                    name: "x".to_string(),
                    span: tokens[1].span.clone()
                },
                Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Ident(IdentifierPath {
                        path: vec![IdentOrType::Ident(Ident {
                            name: "y".to_string(),
                            span: tokens[3].span.clone()
                        })]
                    }),
                    secondaries: None,
                    type_annotation: None,
                })),
                Block {
                    statements: vec![Statement::Expression(Expression::UnaryExpr(
                        UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Literal(Literal {
                                kind: LiteralKind::Number(2),
                                span: tokens[5].span.clone()
                            }),
                            secondaries: None,
                            type_annotation: None,
                        })
                    ))],
                }
            )
        );
        assert_eq!(remaining.len(), 0);
    }

    #[test]
    fn parse_while() {
        let input = "while x\n    2";
        let tokens = lex_test(input);
        let config = Config::default();

        let (remaining, loop_) = r#loop.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            loop_,
            Loop::While(
                Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                    operand: Operand::Ident(IdentifierPath {
                        path: vec![IdentOrType::Ident(Ident {
                            name: "x".to_string(),
                            span: tokens[1].span.clone()
                        })]
                    }),
                    secondaries: None,
                    type_annotation: None,
                })),
                Block {
                    statements: vec![Statement::Expression(Expression::UnaryExpr(
                        UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Literal(Literal {
                                kind: LiteralKind::Number(2),
                                span: tokens[3].span.clone()
                            }),
                            secondaries: None,
                            type_annotation: None,
                        })
                    ))],
                }
            )
        );
        assert_eq!(remaining.len(), 0);
    }

    #[test]
    fn parse_loop() {
        let input = "loop\n    2";
        let tokens = lex_test(input);
        let config = Config::default();

        let (remaining, loop_) = r#loop.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            loop_,
            Loop::Loop(Block {
                statements: vec![Statement::Expression(Expression::UnaryExpr(
                    UnaryExpr::PrimaryExpr(PrimaryExpr {
                        operand: Operand::Literal(Literal {
                            kind: LiteralKind::Number(2),
                            span: tokens[2].span.clone()
                        }),
                        secondaries: None,
                        type_annotation: None,
                    })
                ))],
            })
        );
        assert_eq!(remaining.len(), 0);
    }
}
