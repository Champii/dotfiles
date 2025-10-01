use crate::new_parser::{engine::*, items::primitives, Literal, LiteralKind};

use super::{array, utils::get_span};

pub fn literal(stream: Input) -> IResult<Literal> {
    let (stream, span) = get_span(stream)?;

    primitives::boolean
        .map(LiteralKind::Bool)
        .or(primitives::int.map(LiteralKind::Number))
        .or(primitives::float.map(LiteralKind::Float))
        .or(array.map(LiteralKind::Array))
        .or(primitives::string.map(LiteralKind::String))
        .or(primitives::char.map(LiteralKind::Char))
        .map(|kind| Literal {
            kind,
            span: span.clone(),
        })
        .process(stream)
}
