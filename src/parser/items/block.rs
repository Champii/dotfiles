use crate::{
    ast::{Block, Statement},
    lexer::Token,
    parser::{
        parsable::Parsable,
        util::{parse_vec_of, ParseError},
    },
};

impl Parsable for Block {
    fn parse(tokens: &[Token]) -> Result<(Self, &[Token]), ParseError> {
        let (statements, new_tokens) = parse_vec_of::<Statement>(tokens, None)?;

        Ok((Block { statements }, new_tokens))
    }
}
