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
    (TokenType::DoubleDot, ident_pattern)
        .map(|(_, ident)| ArrayPattern::Rest(ident))
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
