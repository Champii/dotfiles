use crate::lexer::TokenType;
use crate::new_parser::engine::*;
use crate::new_parser::ParseType;
use crate::new_parser::ParseTypeInner;

use super::get_span;
use super::type_token;

pub fn parse_type(stream: Input) -> IResult<ParseType> {
    // parse_function_type
    /* .or(parse_array_type)
    .or(parse_tuple_type) */
    // .or(parse_type_inner.map(ParseType::Type))
    parse_type_inner.map(ParseType::Type).process(stream)
}

/* fn parse_function_type(stream: Input) -> IResult<ParseType> {
    (
        TokenType::OpenParen,
        parse_type,
        TokenType::Coma,
        parse_type,
        TokenType::CloseParen,
        TokenType::Arrow,
        parse_type,
    )
        .map(|(_, input, _, output, _, _, _)| FunctionType { input, output })
        .process(stream)
} */

pub fn parse_type_inner(stream: Input) -> IResult<ParseTypeInner> {
    (
        get_span,
        type_token,
        many((parse_type, TokenType::Coma.opt())),
    )
        .map(|(span, name, generics)| ParseTypeInner {
            name,
            generics: generics.into_iter().map(|(name, _)| name).collect(),
            span,
        })
        .process(stream)
}
