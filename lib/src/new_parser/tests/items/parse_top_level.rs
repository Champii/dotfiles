use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

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
        TopLevel::FunctionDecl(FunctionDecl {
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
                    kind: PatternKind::Ident(IdentPattern {
                        name: Ident {
                            name: "foo".to_string(),
                            span: Span {
                                start: 4,
                                end: 7,
                                file_path: PathBuf::new(),
                            },
                        },
                        mut_: false,
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
    );

    assert_eq!(tokens.len(), 0);
}
