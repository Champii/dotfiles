use crate::{
    lexer::TokenType,
    new_parser::{engine::*, Block, Statement},
};

use super::{empty_lines, indent, indent_token, seek, statement};

pub fn block(stream: Input) -> IResult<Block> {
    preceded(
        TokenType::Eol,
        indented(separated1(
            preceded(indent, statement)
                .or(followed(indent_token, seek(TokenType::Eol)).map(|_| Statement::EmptyLine)),
            TokenType::Eol,
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
}
