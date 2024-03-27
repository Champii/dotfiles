use std::collections::BTreeMap;

use crate::{
    ast::{FunctionDecl, Ident, ParseType, TraitDecl},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{parsable::Parsable, parse_ctx::ParseCtx, util::expect_token},
};

impl Parsable for TraitDecl {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("trait".to_string()))?;

        let (name, remaining_tokens) = ParseType::parse(remaining_tokens, parse_ctx)?;

        let mut remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        let mut methods = BTreeMap::new();
        let mut signatures = BTreeMap::new();

        parse_ctx.indent();

        loop {
            if remaining_tokens.is_empty() {
                break;
            }

            if let Ok(new_remaining_tokens) = parse_ctx.consume_indent(remaining_tokens) {
                remaining_tokens = new_remaining_tokens;
            } else {
                break;
            }

            if let Ok((function, new_remaining_tokens)) =
                FunctionDecl::parse(remaining_tokens, parse_ctx)
            {
                remaining_tokens = new_remaining_tokens;
                methods.insert(function.name.clone(), function);
                continue;
            }

            if let Ok((signature, new_remaining_tokens)) =
                <(Ident, ParseType)>::parse(remaining_tokens, parse_ctx)
            {
                remaining_tokens = new_remaining_tokens;
                signatures.insert(signature.0.clone(), signature.1);
                continue;
            }

            break;
        }

        parse_ctx.dedent();

        Ok((
            TraitDecl {
                name,
                methods,
                signatures,
            },
            remaining_tokens,
        ))
    }
}

#[cfg(test)]
mod parse_trait {
    use crate::{parser::util::lex_test, Config};

    use super::*;

    #[test]
    fn test_parse_trait() {
        let tokens = lex_test(
            r#"trait Foo
  bar = a -> a
  baz : Int
"#,
        );

        let (trait_decl, rest) =
            TraitDecl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(trait_decl.name.name, "Foo");
        assert_eq!(trait_decl.methods.len(), 1);
        assert_eq!(trait_decl.signatures.len(), 1);
        assert_eq!(rest.len(), 0);
    }
}
