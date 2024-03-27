use super::parse_ctx::ParseCtx;
use crate::{diagnostic::Diagnostics, lexer::Token};

pub trait Parsable: Sized {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics>;
}
