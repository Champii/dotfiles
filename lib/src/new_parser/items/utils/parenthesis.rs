//
//
//
//
//
//
//
//
//
//
//
//
use crate::{lexer::TokenType, new_parser::engine::*};

pub fn parenthesis<P: Parser>(parser: P) -> impl FnMut(Input) -> IResult<P::Output> {
    let mut parser = (TokenType::OpenParen, parser, TokenType::CloseParen).map(|(_, x, _)| x);

    move |input: Input| parser.process(input)
}
