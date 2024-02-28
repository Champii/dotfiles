use super::util::ParseError;
use crate::lexer::Token;

pub trait Parsable: Sized {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError>;
}
