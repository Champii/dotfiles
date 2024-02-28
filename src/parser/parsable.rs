use super::{parse_ctx::ParseCtx, util::ParseError};
use crate::lexer::Token;

pub trait Parsable: Sized {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError>;
}
