use std::collections::BTreeMap;

use crate::{
    ast::{FunctionDecl, FunctionSig, Impl, ParseTypeInner},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parse_ctx::ParseCtx,
        util::{expect_token, ignore_empty_lines},
        Parsable,
    },
};

impl Parsable for Impl {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("impl".to_string()))?;

        let (name, remaining_tokens) = ParseTypeInner::parse(remaining_tokens, parse_ctx)?;

        let mut remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        let mut methods = BTreeMap::new();
        let mut signatures = BTreeMap::new();

        parse_ctx.indent();

        loop {
            if remaining_tokens.is_empty() {
                break;
            }

            remaining_tokens = ignore_empty_lines(remaining_tokens);

            if let Ok(new_remaining_tokens) = parse_ctx.consume_indent(remaining_tokens) {
                remaining_tokens = new_remaining_tokens;
            } else {
                break;
            }

            if let Ok((method, new_remaining_tokens)) =
                FunctionDecl::parse(remaining_tokens, parse_ctx)
            {
                remaining_tokens = new_remaining_tokens;

                methods.insert(method.name.clone(), method);

                continue;
            }

            if let Ok((signature, new_remaining_tokens)) =
                FunctionSig::parse(remaining_tokens, parse_ctx)
            {
                remaining_tokens = new_remaining_tokens;

                signatures.insert(signature.name.clone(), signature);

                continue;
            }

            break;
        }

        parse_ctx.dedent();

        Ok((
            Impl {
                name,
                methods,
                signatures,
            },
            remaining_tokens,
        ))
    }
}

#[cfg(test)]
mod parse_struct {
    use crate::{parser::util::lex_test, Config};

    use super::*;

    #[test]
    fn test_parse_impl() {
        let input = "impl Test\n";
        let tokens = lex_test(input);
        let (r#impl, rest) = Impl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(r#impl.name.to_string(), "Test");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_impl_with_methods() {
        let input = "impl Test\n  new = -> lol\n  @add = -> a\n";
        let tokens = lex_test(input);
        let (r#impl, rest) = Impl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(r#impl.name.to_string(), "Test");
        assert_eq!(r#impl.methods.len(), 2);
        assert_eq!(
            r#impl
                .methods
                .iter()
                .find(|(k, _v)| k.name == "new")
                .unwrap()
                .1
                .name
                .name,
            "new"
        );
        assert_eq!(rest.len(), 0);
    }
}
