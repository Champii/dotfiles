use crate::new_parser::{engine::*, Ident};

use super::{ident_token, operator};

pub fn ident(stream: Input) -> IResult<Ident> {
    ident_token
        .or(operator.map(|op| Ident {
            name: op.to_string(),
            span: op.span.clone(),
        }))
        .process(stream)
}
