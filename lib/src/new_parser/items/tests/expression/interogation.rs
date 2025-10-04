use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::*;
use crate::Config;

#[test]
fn interogation() {
    let input = "foo? bar, baz?";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, expression) = expression
        .process(ParseCtx::from(&tokens, &config))
        .unwrap();

    assert_eq!(
        expression,
        Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
            operand: Operand::Ident(IdentifierPath {
                path: vec![IdentOrType::Ident(Ident {
                    name: "foo".to_string(),
                    span: Span::default(),
                })],
            }),
            secondaries: Some(vec![
                SecondaryExpr::Interogation,
                SecondaryExpr::Arguments(vec![
                    Argument {
                        arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Ident(IdentifierPath {
                                path: vec![IdentOrType::Ident(Ident {
                                    name: "bar".to_string(),
                                    span: Span::default(),
                                })],
                            }),
                            secondaries: None,
                            type_annotation: None,
                        }))
                    },
                    Argument {
                        arg: Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                            operand: Operand::Ident(IdentifierPath {
                                path: vec![IdentOrType::Ident(Ident {
                                    name: "baz".to_string(),
                                    span: Span::default(),
                                })],
                            }),
                            secondaries: Some(vec![SecondaryExpr::Interogation]),
                            type_annotation: None,
                        }))
                    }
                ]),
            ]),

            type_annotation: None,
        })),
    );
    assert_eq!(rest.len(), 0);
}
