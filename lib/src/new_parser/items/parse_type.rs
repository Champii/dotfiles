use crate::lexer::TokenType;
use crate::new_parser::engine::*;
use crate::new_parser::ParseType;
use crate::new_parser::ParseTypeInner;

use super::get_span;
use super::parenthesis;
use super::seek;
use super::type_token;

pub fn parse_type(stream: Input) -> IResult<ParseType> {
    preceded(
        seek(
            type_token
                .map(|_| ())
                .or(TokenType::OpenParen.map(|_| ()))
                .or(TokenType::OpenBracket.map(|_| ()))
                .or(TokenType::StuckOperator("*".to_string()).map(|_| ()))
                .or(TokenType::StuckOperator("&".to_string()).map(|_| ())),
        ),
        parse_function_type
            .or(parse_array_type)
            .or(parse_tuple_type)
            .or(parse_reference_type)
            .or(parse_pointer_type)
            .or(parse_type_inner.map(ParseType::Type)),
    )
    .process(stream)
}

fn parse_reference_type(stream: Input) -> IResult<ParseType> {
    (
        TokenType::StuckOperator("&".to_string()),
        TokenType::Keyword("mut".to_string()).opt(),
        parse_type,
    )
        .map(|(_, mut_, t)| ParseType::Reference {
            is_mut: mut_.is_some(),
            pointee: Box::new(t),
        })
        .process(stream)
}

fn parse_pointer_type(stream: Input) -> IResult<ParseType> {
    (TokenType::StuckOperator("*".to_string()), parse_type)
        .map(|(_, t)| ParseType::Pointer(Box::new(t)))
        .process(stream)
}

fn parse_function_type(stream: Input) -> IResult<ParseType> {
    (parenthesis_if_inside_fn_type_decl(separated1(parse_type, TokenType::Arrow)))
        .assert(|types| types.len() >= 2)
        .map(ParseType::Function)
        .process(stream)
}

fn parenthesis_if_inside_fn_type_decl<P: Parser>(
    mut parser: P,
) -> impl FnMut(Input) -> IResult<P::Output> {
    move |mut stream| {
        if stream.is_inside_fn_type_decl {
            let (stream, _) = TokenType::OpenParen.process(stream)?;
            let (stream, x) = parser.process(stream)?;
            let (stream, _) = TokenType::CloseParen.process(stream)?;
            Ok((stream, x))
        } else {
            stream.is_inside_fn_type_decl = true;

            let (mut stream, t) = parser.process(stream)?;

            stream.is_inside_fn_type_decl = false;

            Ok((stream, t))
        }
    }
}

pub fn parse_array_type(stream: Input) -> IResult<ParseType> {
    (delimited(TokenType::OpenBracket, parse_type, TokenType::CloseBracket))
        .map(Box::new)
        .map(ParseType::Array)
        .process(stream)
}

pub fn parse_tuple_type(stream: Input) -> IResult<ParseType> {
    (TokenType::OpenParen, TokenType::CloseParen)
        .map(|_| ParseType::Tuple(vec![]))
        .or((parenthesis(separated(parse_type, TokenType::Coma))).map(ParseType::Tuple))
        .process(stream)
}

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
