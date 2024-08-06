use crate::{lexer::TokenType, new_parser::engine::*};

pub fn empty_lines(stream: Input) -> IResult<()> {
    many((TokenType::Indent(stream.indent_level as u8), TokenType::Eol))
        .map(|_| ())
        .process(stream)
}
