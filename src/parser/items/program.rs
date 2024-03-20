use crate::{
    ast::{Program, TopLevel},
    lexer::{Token, TokenType},
    parser::{
        parse_ctx::ParseCtx,
        util::{expect_token, ignore_empty_lines, ParseError},
        Parsable,
    },
};

impl Parsable for Program {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let mut statements = Vec::new();
        let mut tokens = tokens;

        loop {
            tokens = ignore_empty_lines(&tokens);

            if tokens.is_empty() || tokens[0].token_type == TokenType::Eof {
                break;
            }

            tokens = expect_token(tokens, TokenType::Indent(0))?;

            let (statement, new_tokens) = TopLevel::parse(tokens, parse_ctx)?;

            if tokens.is_empty() || tokens[0].token_type == TokenType::Eof {
                break;
            }

            tokens = new_tokens;

            statements.push(statement);
        }

        let remaining_tokens = expect_token(tokens, TokenType::Eof)?;

        if !remaining_tokens.is_empty() {
            return Err(ParseError::LeftoverTokens(remaining_tokens.to_vec()));
        }

        Ok((
            Program {
                top_levels: statements,
            },
            tokens,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_string;

    #[test]
    fn program_with_newlines() {
        let input = r#"

main = -> 1


test = -> 2


"#;

        assert!(parse_string(input).is_ok());
    }

    #[test]
    fn program_with_no_newlines() {
        let input = r#"main = -> 1
test = -> 2"#;

        assert!(parse_string(input).is_ok());
    }

    #[test]
    fn program_with_good_2_indent() {
        let input = r#"main = ->
  a
  2"#;

        assert!(parse_string(input).is_ok());
    }

    #[test]
    fn program_with_good_4_indent() {
        let input = r#"main = ->
    a
    2"#;

        assert!(parse_string(input).is_ok());
    }

    #[test]
    fn program_with_bad_indent() {
        let input = r#"main = ->
    a
  2"#;

        assert!(parse_string(input).is_err());
    }
}
