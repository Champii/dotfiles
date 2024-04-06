use crate::{
    ast::{Block, Expression, Ident, Loop},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{parsable::Parsable, parse_ctx::ParseCtx, util::expect_token},
};

impl Parsable for Loop {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let mut remaining = tokens;

        if TokenType::Keyword("for".to_string()) == remaining[0].token_type {
            let (binding, remaining) = Ident::parse(&remaining[1..], parse_ctx)?;
            let remaining = expect_token(&remaining, TokenType::Keyword("in".to_string()))?;

            let (expr, remaining) =
                parse_ctx.disallow_multiline_fn_call(|parse_ctx| Expression::parse(remaining, parse_ctx))?;

            let (block, remaining) = Block::parse(remaining, parse_ctx)?;

            Ok((Loop::For(binding, expr, block), remaining))
        } else if TokenType::Keyword("while".to_string()) == remaining[0].token_type {
            let (expr, remaining) = parse_ctx
                .disallow_multiline_fn_call(|parse_ctx| Expression::parse(&remaining[1..], parse_ctx))?;

            let (block, remaining) = Block::parse(remaining, parse_ctx)?;

            Ok((Loop::While(expr, block), remaining))
        } else {
            remaining = expect_token(remaining, TokenType::Keyword("loop".to_string()))?;
            let (block, remaining) = Block::parse(remaining, parse_ctx)?;

            Ok((Loop::Loop(block), remaining))
        }
    }
}

#[cfg(test)]
mod parse_loop {
    use crate::{
        ast::{
            Block, Expression, Ident, IdentOrType, IdentifierPath, Literal, LiteralKind, Loop,
            Operand, PrimaryExpr, Statement, UnaryExpr,
        },
        parser::{parsable::Parsable, parse_ctx::ParseCtx, util::lex_test},
        Config,
    };

    #[test]
    fn parse_for() {
        let input = "for x in y\n  2";
        let tokens = lex_test(input);
        let (loop_, remaining) =
            Loop::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let input = "while x\n  2";
        let tokens = lex_test(input);
        let (loop_, remaining) =
            Loop::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
        let input = "loop\n  2";
        let tokens = lex_test(input);
        let (loop_, remaining) =
            Loop::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

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
