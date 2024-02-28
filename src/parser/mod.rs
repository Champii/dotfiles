mod items;
mod parsable;
mod util;

use parsable::Parsable;

use crate::{ast::Program, lexer::Token};

use self::util::ParseError;

pub fn parse_root(tokens: &[Token]) -> Result<Program, ParseError> {
    Program::parse(tokens).map(|(program, _)| program)
}
