use crate::new_parser::{engine::*, Pattern, PatternKind};

use super::ident;

pub fn pattern(stream: Input) -> IResult<Pattern> {
    ident
        .map(|ident| Pattern {
            binding: Some(ident.clone()),
            kind: PatternKind::Ident(ident),
        })
        .process(stream)
}
