#[cfg(test)]
mod new_parser {
    use crate::ast::tree::*;
    use crate::lexer::{Span, Token};
    use crate::new_parser::items::{function_decl, ident, struct_decl, top_level};
    use crate::new_parser::{ParseCtx, Parser};
    use crate::Config;
    use std::path::PathBuf;

    fn lex_test(input: &str) -> Vec<Token> {
        use crate::lexer::Lexer;

        let mut tokens = Lexer::new(std::path::PathBuf::new(), input)
            .unwrap()
            .with_newline_at_end(false)
            .collect()
            .unwrap();

        //ignore indent
        tokens.remove(0);

        //ignore EOF
        tokens.pop();

        tokens
    }

    #[test]
    fn test_parse_ident() {
        let tokens = lex_test("foo");
        let config = Config::default();

        let (parse_ctx, ident) = ident(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            ident,
            Ident {
                name: "foo".to_string(),
                span: Span {
                    start: 0,
                    end: 3,
                    file_path: PathBuf::new(),
                },
            }
        );

        assert_eq!(parse_ctx.len(), 0);
    }

    #[test]
    fn test_parse_function_decl() {
        let tokens = lex_test("a = foo -> foo\n");
        let config = Config::default();

        let (tokens, function_decl) = function_decl(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            function_decl,
            FunctionDecl {
                name: Ident {
                    name: "a".to_string(),
                    span: Span {
                        start: 0,
                        end: 1,
                        file_path: PathBuf::new(),
                    },
                },
                lambda: LambdaDecl {
                    parameters: vec![Pattern {
                        binding: None,
                        kind: PatternKind::Ident(Ident {
                            name: "foo".to_string(),
                            span: Span {
                                start: 4,
                                end: 7,
                                file_path: PathBuf::new(),
                            },
                        }),
                    },],
                    body: Block {
                        statements: vec![Statement::Expression(Expression::UnaryExpr(
                            UnaryExpr::PrimaryExpr(PrimaryExpr {
                                operand: Operand::Ident(IdentifierPath {
                                    path: vec![IdentOrType::Ident(Ident {
                                        name: "foo".to_string(),
                                        span: Span {
                                            start: 11,
                                            end: 14,
                                            file_path: PathBuf::new(),
                                        },
                                    })]
                                }),
                                secondaries: None,
                                type_annotation: None,
                            })
                        ))]
                    },
                    shorthand_tokens: None,
                },
                inject_self: false,
            }
        );

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn parse_top_level() {
        use crate::lexer::Lexer;

        let mut tokens = Lexer::new(std::path::PathBuf::new(), "a = foo -> foo\n")
            .unwrap()
            .with_newline_at_end(false)
            .collect()
            .unwrap();

        tokens.pop();

        // let tokens = lex_test();
        let config = Config::default();

        let (tokens, top_level) = top_level(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            top_level,
            TopLevel {
                ident: Ident {
                    name: "a".to_string(),
                    span: Span {
                        start: 0,
                        end: 1,
                        file_path: PathBuf::new(),
                    },
                },
                kind: TopLevelKind::FunctionDecl(FunctionDecl {
                    name: Ident {
                        name: "a".to_string(),
                        span: Span {
                            start: 0,
                            end: 1,
                            file_path: PathBuf::new(),
                        },
                    },
                    lambda: LambdaDecl {
                        parameters: vec![Pattern {
                            binding: None,
                            kind: PatternKind::Ident(Ident {
                                name: "foo".to_string(),
                                span: Span {
                                    start: 4,
                                    end: 7,
                                    file_path: PathBuf::new(),
                                },
                            }),
                        },],
                        body: Block {
                            statements: vec![Statement::Expression(Expression::UnaryExpr(
                                UnaryExpr::PrimaryExpr(PrimaryExpr {
                                    operand: Operand::Ident(IdentifierPath {
                                        path: vec![IdentOrType::Ident(Ident {
                                            name: "foo".to_string(),
                                            span: Span {
                                                start: 11,
                                                end: 14,
                                                file_path: PathBuf::new(),
                                            },
                                        })]
                                    }),
                                    secondaries: None,
                                    type_annotation: None,
                                })
                            ))]
                        },
                        shorthand_tokens: None,
                    },
                    inject_self: false,
                }),
            }
        );

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_parse_struct_decl() {
        let tokens = lex_test("struct Foo\n    a: Int\n    b: Int = 0\n");
        let config = Config::default();

        let (tokens, struct_decl) = struct_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(
            struct_decl,
            StructDecl {
                name: ParseTypeInner {
                    name: "Foo".to_string(),
                    generics: vec![],
                    span: Span {
                        start: 7,
                        end: 10,
                        file_path: PathBuf::new(),
                    },
                },
                fields: vec![
                    StructDeclField {
                        name: Ident {
                            name: "a".to_string(),
                            span: Span {
                                start: 15,
                                end: 16,
                                file_path: PathBuf::new(),
                            },
                        },
                        ty: ParseType::Type(ParseTypeInner {
                            name: "Int".to_string(),
                            generics: vec![],
                            span: Span {
                                start: 18,
                                end: 21,
                                file_path: PathBuf::new(),
                            }
                        }),
                        public: false,
                        default: None,
                    },
                    StructDeclField {
                        name: Ident {
                            name: "b".to_string(),
                            span: Span {
                                start: 26,
                                end: 27,
                                file_path: PathBuf::new(),
                            },
                        },
                        ty: ParseType::Type(ParseTypeInner {
                            name: "Int".to_string(),
                            generics: vec![],
                            span: Span {
                                start: 29,
                                end: 32,
                                file_path: PathBuf::new(),
                            }
                        }),
                        public: false,
                        default: Some(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Literal(Literal {
                                kind: LiteralKind::Number(0),
                                span: Span {
                                    start: 35,
                                    end: 36,
                                    file_path: PathBuf::new(),
                                }
                            }),
                            secondaries: None,
                            type_annotation: None,
                        }))),
                    }
                ]
            }
        );

        assert_eq!(tokens.len(), 0);
    }
}
