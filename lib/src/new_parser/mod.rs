use iresult::IResult;
use many::many;
use parse_error::ParseError;
use parser_trait::Parser;

use crate::ast::tree::*;
use crate::lexer::{Token, TokenType};

mod and;
mod fns;
mod iresult;
mod many;
mod map;
mod opt;
mod or;
mod parse_error;
mod parser_trait;
mod token_type;
mod tuples;

pub type Input<'a> = &'a [Token];

fn parse_function_decl<'a>(stream: Input<'a>) -> IResult<'a, FunctionDecl> {
    (
        TokenType::Arobase.opt(),
        parse_ident,
        TokenType::Equal,
        parse_lambda_decl,
        TokenType::Eol,
    )
        .map(|(inject_self, ident, _, lambda, _)| FunctionDecl {
            name: ident,
            lambda,
            inject_self: inject_self.is_some(),
        })
        .process(stream)
}

fn parse_ident(stream: Input) -> IResult<Ident> {
    if stream.is_empty() {
        return Err(ParseError::UnexpectedEOF);
    }

    let token = &stream[0];

    if let TokenType::Ident(name) = &token.token_type {
        Ok((
            &stream[1..],
            Ident {
                name: name.clone(),
                span: token.span.clone(),
            },
        ))
    } else {
        Err(ParseError::ExpectedIdent(token.clone()))
    }
}

fn parse_lambda_decl(stream: Input) -> IResult<LambdaDecl> {
    (parse_parameters, TokenType::Arrow, parse_block)
        .map(|(parameters, _, body)| LambdaDecl {
            parameters,
            body,
            shorthand_tokens: None,
        })
        .process(stream)
}

fn parse_parameters(stream: Input) -> IResult<Vec<Pattern>> {
    many((parse_pattern, TokenType::Coma.opt()))
        .map(|pat_vec| pat_vec.into_iter().map(|(pat, _)| pat).collect::<Vec<_>>())
        .process(stream)
}

fn parse_pattern(stream: Input) -> IResult<Pattern> {
    parse_ident
        .map(|ident| Pattern {
            binding: Some(ident.clone()),
            kind: PatternKind::Ident(ident),
        })
        .process(stream)
}

fn parse_block(stream: Input) -> IResult<Block> {
    many(parse_statement)
        .map(|statements| Block { statements })
        .process(stream)
}

fn parse_statement(stream: Input) -> IResult<Statement> {
    /* parse_assignment
    .map(Statement::Assignment)
    .or(parse_expression.map(Statement::Expression))
    .or(parse_return.map(Statement::Return))
    .or(parse_continue.map(Statement::Continue))
    .or(parse_break.map(Statement::Break))
    .or(TokenType::Eol.map(|_| Statement::EmptyLine)) */
    parse_ident
        .map(|ident| {
            Statement::Expression(Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
                operand: Operand::Ident(IdentifierPath {
                    path: vec![IdentOrType::Ident(ident)],
                }),
                secondaries: None,
                type_annotation: None,
            })))
        })
        .process(stream)
}

#[cfg(test)]
mod new_parser {
    use super::*;
    use crate::lexer::Span;
    use std::path::PathBuf;

    use crate::{parser::util::lex_test, Config};

    #[test]
    fn test_parse_ident() {
        let tokens = lex_test("foo");

        let (tokens, ident) = parse_ident(&tokens).unwrap();

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

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_parse_function_decl() {
        let tokens = lex_test("a = foo -> foo\n");

        let (tokens, function_decl) = parse_function_decl(&tokens).unwrap();

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
                        binding: Some(Ident {
                            name: "foo".to_string(),
                            span: Span {
                                start: 4,
                                end: 7,
                                file_path: PathBuf::new(),
                            },
                        }),
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
}
