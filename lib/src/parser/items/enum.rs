use crate::{
    ast::{EnumDecl, EnumVariant, NamedFieldsOrTypesList, ParseTypeInner},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        util::{expect_token, ignore_empty_lines},
        Parsable, ParseCtx,
    },
};

use super::r#struct::parse_struct_fields;

impl Parsable for EnumDecl {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("enum".to_string()))?;

        let (name, remaining_tokens) = ParseTypeInner::parse(remaining_tokens, parse_ctx)?;

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        let (variants, remaining_tokens) = parse_ctx
            .indent_block(|parse_ctx| parse_enum_decl_variant_loop(remaining_tokens, parse_ctx))?;

        Ok((EnumDecl { name, variants }, remaining_tokens))
    }
}

fn parse_enum_decl_variant_loop<'a>(
    tokens: &'a [Token],
    parse_ctx: &mut ParseCtx,
) -> Result<(Vec<EnumVariant>, &'a [Token]), Diagnostics> {
    let mut remaining_tokens = tokens;
    let mut variants = Vec::new();

    loop {
        if remaining_tokens.is_empty() {
            break;
        }

        remaining_tokens = ignore_empty_lines(remaining_tokens);

        let Ok(new_remaining_tokens) = parse_ctx.consume_indent(remaining_tokens) else {
            break;
        };

        if let Ok((variant, new_remaining_tokens)) =
            EnumVariant::parse(new_remaining_tokens, parse_ctx)
        {
            remaining_tokens = new_remaining_tokens;

            variants.push(variant);
        } else {
            break;
        }
    }
    Ok((variants, remaining_tokens))
}

impl Parsable for EnumVariant {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let (name, remaining_tokens) = ParseTypeInner::parse(tokens, parse_ctx)?;

        if name.generics.len() > 0 {
            let types_list = name.generics.clone();
            let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;
            return Ok((
                EnumVariant {
                    name: ParseTypeInner {
                        span: name.span,
                        name: name.name,
                        generics: Vec::new(),
                    },
                    fields: NamedFieldsOrTypesList::TypesList(types_list),
                },
                remaining_tokens,
            ));
        } else if let TokenType::Eol = remaining_tokens[0].token_type {
            if let Ok((fields, remaining_tokens)) =
                NamedFieldsOrTypesList::parse(&remaining_tokens[1..], parse_ctx)
            {
                Ok((EnumVariant { name, fields }, remaining_tokens))
            } else {
                unimplemented!()
            }
        } else {
            Ok((
                EnumVariant {
                    name,
                    fields: NamedFieldsOrTypesList::NamedFields(Vec::new()),
                },
                remaining_tokens,
            ))
        }
    }
}

impl Parsable for NamedFieldsOrTypesList {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        if let Ok((fields, remaining_tokens)) = parse_struct_fields(tokens, parse_ctx) {
            Ok((
                NamedFieldsOrTypesList::NamedFields(fields),
                remaining_tokens,
            ))
        } else {
            unimplemented!()
        }
    }
}

#[cfg(test)]
mod parse_enum {
    use super::*;
    use crate::{parser::util::lex_test, Config};

    #[test]
    fn test_parse_enum() {
        let input = "enum Type\n  Variant1\n  Variant2 T, U\n  StructLike\n    field: Type\n";
        let tokens = lex_test(input);
        let (enum_decl, rest) =
            EnumDecl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(enum_decl.name.to_string(), "Type");
        assert_eq!(enum_decl.variants.len(), 3);
        assert_eq!(rest.len(), 0);
    }
}
