use crate::{
    ast::{Block, FunctionDecl, Ident, LambdaDecl},
    lexer::{Token, TokenType},
    parser::{
        parsable::Parsable,
        parse_ctx::ParseCtx,
        util::{expect_token, parse_vec_of, ParseError},
    },
};

impl Parsable for FunctionDecl {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let (name, mut remaining_tokens) = Ident::parse(tokens, parse_ctx)?;
        remaining_tokens = expect_token(remaining_tokens, TokenType::Equal)?;

        let (lambda, remaining_tokens) = LambdaDecl::parse(remaining_tokens, parse_ctx)?;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        Ok((FunctionDecl { name, lambda }, remaining_tokens))
    }
}

impl Parsable for LambdaDecl {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), ParseError> {
        let remaining_tokens = tokens;

        let (parameters, mut remaining_tokens) =
            parse_vec_of::<Ident>(remaining_tokens, Some(TokenType::Coma), parse_ctx)?;

        // consume token if it's a coma
        if !remaining_tokens.is_empty() && remaining_tokens[0].token_type == TokenType::Coma {
            remaining_tokens = &remaining_tokens[1..];
        }

        remaining_tokens = expect_token(remaining_tokens, TokenType::Arrow)?;

        let (body, remaining_tokens) = Block::parse(remaining_tokens, parse_ctx)?;

        Ok((LambdaDecl { parameters, body }, remaining_tokens))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::util::lex_test;

    #[test]
    fn test_parse_function_decl_monoline() {
        let input = "myfn = -> statement\n";
        let tokens = lex_test(input);
        let (function_decl, rest) = FunctionDecl::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(function_decl.name.name, "myfn");
        assert_eq!(function_decl.lambda.parameters.len(), 0);
        assert_eq!(function_decl.lambda.body.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_function_decl() {
        let input = "myfn = a, b, c ->\n  statement\n";
        let tokens = lex_test(input);
        let (function_decl, rest) = FunctionDecl::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(function_decl.name.name, "myfn");
        assert_eq!(function_decl.lambda.parameters.len(), 3);
        assert_eq!(function_decl.lambda.body.statements.len(), 1);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_function_decl_multiline() {
        let input = r#"myfn = a, b, c ->
  statement
  3 + 3
"#;
        let tokens = lex_test(input);
        let (function_decl, rest) = FunctionDecl::parse(&tokens, &mut ParseCtx::new()).unwrap();

        assert_eq!(function_decl.name.name, "myfn");
        assert_eq!(function_decl.lambda.parameters.len(), 3);
        assert_eq!(function_decl.lambda.body.statements.len(), 2);
        assert_eq!(rest.len(), 0);
    }
}
