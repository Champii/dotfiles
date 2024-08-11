use crate::{
    lexer::TokenType,
    new_parser::{
        engine::*, ArrayPattern, FieldPattern, FieldsPatternOrArgumentsPattern, InstancePattern,
        Pattern, PatternKind,
    },
};

use super::{ident, ident_path, literal, parenthesis, seek, type_path, type_token};

pub fn pattern(stream: Input) -> IResult<Pattern> {
    (followed(ident, TokenType::Arobase).opt(), pattern_kind)
        .map(|(binding, kind)| Pattern { binding, kind })
        .process(stream)
}

pub fn pattern_kind(stream: Input) -> IResult<PatternKind> {
    parenthesis(separated1(pattern, TokenType::Coma).map(|mut patterns| {
        if patterns.len() == 1 {
            PatternKind::Nested(Box::new(patterns.pop().unwrap()))
        } else {
            PatternKind::Tuple(patterns)
        }
    }))
    .or(delimited(
        TokenType::OpenBracket,
        separated(array_pattern, TokenType::Coma),
        TokenType::CloseBracket,
    )
    .map(PatternKind::Array))
    .or(instance_pattern.map(PatternKind::Instance))
    .or(ident.map(PatternKind::Ident))
    .or(TokenType::Underscore.map(|_| PatternKind::Wildcard))
    .or(literal.map(PatternKind::Literal))
    .process(stream)
}

pub fn array_pattern(stream: Input) -> IResult<ArrayPattern> {
    (
        TokenType::Dot.or(TokenType::SpacedDot),
        TokenType::Dot,
        ident,
    )
        .map(|(_, _, ident)| ArrayPattern::Rest(ident))
        .or(pattern.map(ArrayPattern::Pattern))
        .process(stream)
}

pub fn instance_pattern(stream: Input) -> IResult<InstancePattern> {
    (type_path, field_pattern_or_arguments_pattern)
        .map(|(name, args)| InstancePattern { name, args })
        .process(stream)
}

pub fn field_pattern_or_arguments_pattern(
    stream: Input,
) -> IResult<FieldsPatternOrArgumentsPattern> {
    separated1(field_pattern, TokenType::Coma)
        .map(FieldsPatternOrArgumentsPattern::Fields)
        .or(separated1(pattern, TokenType::Coma).map(FieldsPatternOrArgumentsPattern::Arguments))
        .process(stream)
}

pub fn field_pattern(stream: Input) -> IResult<FieldPattern> {
    (ident, TokenType::Colon, pattern)
        .map(|(name, _, pattern)| FieldPattern { name, pattern })
        .process(stream)
}

#[cfg(test)]
mod pattern {
    use super::*;
    use crate::{ast::*, lexer::Span, new_parser::lex_test, Config};

    #[test]
    fn test_parse_patter_with_binding() {
        let input = "a @ 1";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, pattern) = pattern.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            pattern,
            Pattern {
                binding: Some(Ident {
                    name: "a".to_string(),
                    span: Span::default()
                }),
                kind: PatternKind::Literal(Literal {
                    kind: LiteralKind::Number(1),
                    span: Span::default()
                })
            }
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn instance_pattern_fields_nested() {
        let input = "Player foo: (Ok 1), bar: toto";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, pattern) = pattern.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            pattern,
            Pattern {
                binding: None,
                kind: PatternKind::Instance(InstancePattern {
                    name: TypePath {
                        path: vec![IdentOrType::Type(ParseType::Type(ParseTypeInner {
                            name: "Player".to_string(),
                            generics: vec![],
                            span: Span::default(),
                        }))]
                    },
                    args: FieldsPatternOrArgumentsPattern::Fields(vec![
                        FieldPattern {
                            name: Ident {
                                name: "foo".to_string(),
                                span: Span::default(),
                            },
                            pattern: Pattern {
                                binding: None,
                                kind: PatternKind::Nested(Box::new(Pattern {
                                    binding: None,
                                    kind: PatternKind::Instance(InstancePattern {
                                        name: TypePath {
                                            path: vec![IdentOrType::Type(ParseType::Type(
                                                ParseTypeInner {
                                                    name: "Ok".to_string(),
                                                    generics: vec![],
                                                    span: Span::default(),
                                                }
                                            ))]
                                        },
                                        args: FieldsPatternOrArgumentsPattern::Arguments(vec![
                                            Pattern {
                                                binding: None,
                                                kind: PatternKind::Literal(Literal {
                                                    kind: LiteralKind::Number(1),
                                                    span: Span::default(),
                                                })
                                            }
                                        ])
                                    })
                                }))
                            }
                        },
                        FieldPattern {
                            name: Ident {
                                name: "bar".to_string(),
                                span: Span::default(),
                            },
                            pattern: Pattern {
                                binding: None,
                                kind: PatternKind::Ident(Ident {
                                    name: "toto".to_string(),
                                    span: Span::default(),
                                })
                            }
                        }
                    ])
                })
            }
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn instance_pattern_arguments_nested() {
        let input = "Player (Ok 1), toto";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, pattern) = pattern.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            pattern,
            Pattern {
                binding: None,
                kind: PatternKind::Instance(InstancePattern {
                    name: TypePath {
                        path: vec![IdentOrType::Type(ParseType::Type(ParseTypeInner {
                            name: "Player".to_string(),
                            generics: vec![],
                            span: Span::default(),
                        }))]
                    },
                    args: FieldsPatternOrArgumentsPattern::Arguments(vec![
                        Pattern {
                            binding: None,
                            kind: PatternKind::Nested(Box::new(Pattern {
                                binding: None,
                                kind: PatternKind::Instance(InstancePattern {
                                    name: TypePath {
                                        path: vec![IdentOrType::Type(ParseType::Type(
                                            ParseTypeInner {
                                                name: "Ok".to_string(),
                                                generics: vec![],
                                                span: Span::default(),
                                            }
                                        ))]
                                    },
                                    args: FieldsPatternOrArgumentsPattern::Arguments(vec![
                                        Pattern {
                                            binding: None,
                                            kind: PatternKind::Literal(Literal {
                                                kind: LiteralKind::Number(1),
                                                span: Span::default(),
                                            })
                                        }
                                    ])
                                })
                            }))
                        },
                        Pattern {
                            binding: None,
                            kind: PatternKind::Ident(Ident {
                                name: "toto".to_string(),
                                span: Span::default(),
                            })
                        }
                    ])
                })
            }
        );

        assert_eq!(rest.len(), 0);
    }
}
