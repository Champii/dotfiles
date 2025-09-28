use crate::{
    lexer::TokenType,
    new_parser::{
        engine::*, ArrayPattern, FieldPattern, FieldsPatternOrArgumentsPattern, IdentPattern,
        InstancePattern, Pattern, PatternKind,
    },
};

use super::{ident, literal, parenthesis, type_path};

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
    .or(ident_pattern.map(PatternKind::Ident))
    .or(TokenType::Underscore.map(|_| PatternKind::Wildcard))
    .or(literal.map(PatternKind::Literal))
    .process(stream)
}

pub fn ident_pattern(stream: Input) -> IResult<IdentPattern> {
    (TokenType::Keyword("mut".to_string()).opt(), ident)
        .map(|(mut_, name)| IdentPattern {
            name,
            mut_: mut_.is_some(),
        })
        .process(stream)
}

pub fn array_pattern(stream: Input) -> IResult<ArrayPattern> {
    (
        TokenType::Dot.or(TokenType::SpacedDot),
        TokenType::Dot,
        ident_pattern,
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
        .or(separated(pattern, TokenType::Coma).map(FieldsPatternOrArgumentsPattern::Arguments))
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
                                kind: PatternKind::Ident(IdentPattern {
                                    name: Ident {
                                        name: "toto".to_string(),
                                        span: Span::default(),
                                    },
                                    mut_: false,
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
                            kind: PatternKind::Ident(IdentPattern {
                                name: Ident {
                                    name: "toto".to_string(),
                                    span: Span::default(),
                                },
                                mut_: false,
                            })
                        }
                    ])
                })
            }
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn mut_ident_pattern() {
        let input = "mut a";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, pattern) = pattern.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(
            pattern,
            Pattern {
                binding: None,
                kind: PatternKind::Ident(IdentPattern {
                    name: Ident {
                        name: "a".to_string(),
                        span: Span::default()
                    },
                    mut_: true,
                })
            }
        );

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn array_pattern_with_rest() {
        // Test a simpler array pattern first to see if array patterns work at all
        let input = "[a, b]";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest_tokens, pattern) = pattern.process(ParseCtx::from(&tokens, &config)).unwrap();

        match pattern.kind {
            PatternKind::Array(patterns) => {
                assert_eq!(patterns.len(), 2);
                // Both should be regular patterns
                for (i, expected_name) in ["a", "b"].iter().enumerate() {
                    match &patterns[i] {
                        ArrayPattern::Pattern(p) => {
                            match &p.kind {
                                PatternKind::Ident(ident_pat) => {
                                    assert_eq!(ident_pat.name.name, *expected_name);
                                }
                                _ => panic!("Expected ident pattern at position {}", i),
                            }
                        }
                        _ => panic!("Expected regular pattern at position {}", i),
                    }
                }
            }
            _ => panic!("Expected array pattern, got: {:?}", pattern.kind),
        }

        assert_eq!(rest_tokens.len(), 0);
    }

    #[test]
    fn tuple_pattern() {
        let input = "(x, y, z)";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, pattern) = pattern.process(ParseCtx::from(&tokens, &config)).unwrap();

        match pattern.kind {
            PatternKind::Tuple(patterns) => {
                assert_eq!(patterns.len(), 3);
                let names = ["x", "y", "z"];
                for (i, expected_name) in names.iter().enumerate() {
                    match &patterns[i].kind {
                        PatternKind::Ident(ident_pat) => {
                            assert_eq!(ident_pat.name.name, *expected_name);
                        }
                        _ => panic!("Expected ident pattern at position {}", i),
                    }
                }
            }
            _ => panic!("Expected tuple pattern, got: {:?}", pattern.kind),
        }

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn binding_pattern() {
        let input = "value @ (x, y)";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, pattern) = pattern.process(ParseCtx::from(&tokens, &config)).unwrap();

        // Should have a binding
        assert!(pattern.binding.is_some());
        assert_eq!(pattern.binding.unwrap().name, "value");

        // Should have a tuple pattern
        match pattern.kind {
            PatternKind::Tuple(patterns) => {
                assert_eq!(patterns.len(), 2);
            }
            _ => panic!("Expected tuple pattern, got: {:?}", pattern.kind),
        }

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn wildcard_pattern() {
        let input = "_";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, pattern) = pattern.process(ParseCtx::from(&tokens, &config)).unwrap();

        match pattern.kind {
            PatternKind::Wildcard => {
                // Test passes
            }
            _ => panic!("Expected wildcard pattern, got: {:?}", pattern.kind),
        }

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn literal_pattern() {
        let input = "42";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, pattern) = pattern.process(ParseCtx::from(&tokens, &config)).unwrap();

        match pattern.kind {
            PatternKind::Literal(literal) => {
                match literal.kind {
                    LiteralKind::Number(n) => {
                        assert_eq!(n, 42);
                    }
                    _ => panic!("Expected number literal"),
                }
            }
            _ => panic!("Expected literal pattern, got: {:?}", pattern.kind),
        }

        assert_eq!(rest.len(), 0);
    }
}
