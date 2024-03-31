use crate::{
    ast::{EnumDecl, EnumInstance, ParseTypeInner},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        util::{expect_token, ignore_empty_lines, parse_vec_of},
        Parsable, ParseCtx,
    },
};

impl Parsable for EnumDecl {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("enum".to_string()))?;

        let (name, remaining_tokens) = ParseTypeInner::parse(remaining_tokens, parse_ctx)?;

        let mut remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        let mut variants = Vec::new();

        parse_ctx.indent();

        loop {
            if remaining_tokens.is_empty() {
                break;
            }

            remaining_tokens = ignore_empty_lines(remaining_tokens);

            let Ok(new_remaining_tokens) = parse_ctx.consume_indent(remaining_tokens) else {
                break;
            };

            if let Ok((variant, new_remaining_tokens)) =
                ParseTypeInner::parse(new_remaining_tokens, parse_ctx)
            {
                remaining_tokens = new_remaining_tokens;

                variants.push(variant);

                if let Ok(new_remaining_tokens) = expect_token(remaining_tokens, TokenType::Eol) {
                    remaining_tokens = new_remaining_tokens;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        parse_ctx.dedent();

        Ok((EnumDecl { name, variants }, remaining_tokens))
    }
}

impl Parsable for EnumInstance {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let (name, remaining_tokens) = ParseTypeInner::parse(tokens, parse_ctx)?;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::DoubleColon)?;

        let (variant, remaining_tokens) = ParseTypeInner::parse(remaining_tokens, parse_ctx)?;

        let (args, remaining_tokens) =
            parse_vec_of(remaining_tokens, Some(TokenType::Coma), parse_ctx)?;

        Ok((
            EnumInstance {
                name,
                variant,
                args,
            },
            remaining_tokens,
        ))
    }
}

#[cfg(test)]
mod parse_enum {
    use super::*;
    use crate::{parser::util::lex_test, Config};

    #[test]
    fn test_parse_enum() {
        let input = "enum Type\n  Variant1\n  Variant2";
        let tokens = lex_test(input);
        let (enum_decl, rest) =
            EnumDecl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(enum_decl.name.to_string(), "Type");
        assert_eq!(enum_decl.variants.len(), 2);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_enum_instance() {
        let input = "Type::Variant1";
        let tokens = lex_test(input);
        let (enum_instance, rest) =
            EnumInstance::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(enum_instance.name.to_string(), "Type");
        assert_eq!(enum_instance.variant.to_string(), "Variant1");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_enum_instance_with_args() {
        let input = "Type::Variant1 arg1, arg2";
        let tokens = lex_test(input);
        let (enum_instance, rest) =
            EnumInstance::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(enum_instance.name.to_string(), "Type");
        assert_eq!(enum_instance.variant.to_string(), "Variant1");
        assert_eq!(enum_instance.args.len(), 2);
        assert_eq!(rest.len(), 0);
    }
}
