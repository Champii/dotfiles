use crate::{
    lexer::TokenType,
    new_parser::{engine::*, Block},
};

use super::{empty_lines, indent, statement};

pub fn block(stream: Input) -> IResult<Block> {
    preceded(
        TokenType::Eol,
        indented(separated1(
            preceded(indent, statement),
            TokenType::Eol.followed_by(empty_lines),
        )),
    )
    .or(statement.map(|statement| vec![statement]))
    .map(|statements| Block { statements })
    .process(stream)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{new_parser::lex_test, Config};

    #[test]
    fn test_parse_block() {
        let input = "statement";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, block) = block.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(block.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_block_with_indent() {
        let input = "\n    statement\n    statement";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, block) = block.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(block.statements.len(), 2);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_block_with_empty_line() {
        let input = "\n    statement\n\n    statement\n    \n    statement";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, block) = block.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(block.statements.len(), 3);
        assert_eq!(rest.len(), 0);
    }
}
