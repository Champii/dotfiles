use std::collections::BTreeMap;

use crate::{
    ast::{
        Expression, Ident, ParseType, ParseTypeInner, StructDecl, StructDeclField, StructInstance,
    },
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parse_ctx::ParseCtx,
        util::{expect_token, ignore_empty_lines, parse_vec_of},
        Parsable,
    },
};

impl Parsable for StructDecl {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = expect_token(tokens, TokenType::Keyword("struct".to_string()))?;

        let (name, remaining_tokens) = ParseTypeInner::parse(remaining_tokens, parse_ctx)?;

        let mut remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        let mut fields = Vec::new();

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

            if let Ok((field, new_remaining_tokens)) =
                <StructDeclField>::parse(remaining_tokens, parse_ctx)
            {
                remaining_tokens = new_remaining_tokens;
                fields.push(field);
                continue;
            }

            break;
        }

        parse_ctx.dedent();

        Ok((StructDecl { name, fields }, remaining_tokens))
    }
}

impl Parsable for StructDeclField {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let mut remaining_tokens = tokens;
        let mut public = false;

        if TokenType::Operator("<".to_string()) == remaining_tokens[0].token_type {
            remaining_tokens = &remaining_tokens[1..];
            public = true;
        }

        let (ident, remaining_tokens) = Ident::parse(remaining_tokens, parse_ctx)?;
        let remaining_tokens = expect_token(remaining_tokens, TokenType::Colon)?;
        let (parse_type, remaining_tokens) = ParseType::parse(remaining_tokens, parse_ctx)?;
        let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        Ok((
            StructDeclField {
                name: ident,
                ty: parse_type,
                public,
            },
            remaining_tokens,
        ))
    }
}

impl Parsable for (Ident, ParseType) {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let remaining_tokens = tokens;

        let (ident, remaining_tokens) = Ident::parse(remaining_tokens, parse_ctx)?;
        let remaining_tokens = expect_token(remaining_tokens, TokenType::Colon)?;
        let (parse_type, remaining_tokens) = ParseType::parse(remaining_tokens, parse_ctx)?;
        let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        Ok(((ident, parse_type), remaining_tokens))
    }
}

impl Parsable for (Ident, Expression) {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let (ident, remaining_tokens) = Ident::parse(tokens, parse_ctx)?;
        let remaining_tokens = expect_token(remaining_tokens, TokenType::Colon)?;
        let (expression, remaining_tokens) = Expression::parse(remaining_tokens, parse_ctx)?;

        Ok(((ident, expression), remaining_tokens))
    }
}

impl Parsable for StructInstance {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let (parse_type, remaining_tokens) = ParseTypeInner::parse(tokens, parse_ctx)?;

        if remaining_tokens.is_empty() {
            return Ok((
                StructInstance {
                    name: parse_type,
                    fields: BTreeMap::new(),
                },
                remaining_tokens,
            ));
        }

        if let TokenType::Eol = remaining_tokens[0].token_type {
            parse_ctx.indent();
            let (fields, remaining_tokens) =
                StructInstanceBlock::parse(&remaining_tokens[1..], parse_ctx)?;
            parse_ctx.dedent();

            Ok((
                StructInstance {
                    name: parse_type,
                    fields: fields.fields.into_iter().collect(),
                },
                remaining_tokens,
            ))
        } else {
            let (fields, remaining_tokens) = parse_vec_of::<(Ident, Expression)>(
                remaining_tokens,
                Some(TokenType::Coma),
                parse_ctx,
            )?;

            Ok((
                StructInstance {
                    name: parse_type,
                    fields: fields.into_iter().collect(),
                },
                remaining_tokens,
            ))
        }
    }
}

struct StructInstanceBlock {
    fields: Vec<(Ident, Expression)>,
}

impl Parsable for StructInstanceBlock {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let mut remaining_tokens = tokens;
        let mut remaining_tokens_after_match = tokens;
        let mut fields = Vec::new();

        loop {
            if remaining_tokens.is_empty() {
                break;
            }

            remaining_tokens = ignore_empty_lines(remaining_tokens);

            let tokens_backup = remaining_tokens;

            if let Ok(new_remaining_tokens) = parse_ctx.consume_indent(remaining_tokens) {
                remaining_tokens = new_remaining_tokens;
            } else {
                remaining_tokens = remaining_tokens_after_match;
                break;
            }

            if let Ok((field, new_remaining_tokens)) =
                <(Ident, Expression)>::parse(remaining_tokens, parse_ctx)
            {
                remaining_tokens = new_remaining_tokens;
                remaining_tokens_after_match = remaining_tokens;
                fields.push(field);
            } else {
                remaining_tokens = tokens_backup;
                break;
            }

            if let Ok(new_remaining_tokens) = expect_token(remaining_tokens, TokenType::Eol) {
                remaining_tokens = new_remaining_tokens;
            } else {
                break;
            }
        }

        Ok((
            StructInstanceBlock {
                fields: fields.into_iter().collect(),
            },
            remaining_tokens,
        ))
    }
}

#[cfg(test)]
mod parse_struct {
    use crate::parser::util::lex_test;
    use crate::Config;

    use super::*;

    #[test]
    fn test_parse_struct() {
        let input = "struct Test\n";
        let tokens = lex_test(input);
        let (struct_decl, rest) =
            StructDecl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(struct_decl.name.name, "Test");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_with_generics() {
        let input = "struct Test T, U\n";
        let tokens = lex_test(input);
        let (struct_decl, rest) =
            StructDecl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        let ty = struct_decl.name;

        assert_eq!(ty.name, "Test");
        assert_eq!(ty.generics.len(), 2);
        assert_eq!(ty.generics[0].to_string(), "T");
        assert_eq!(ty.generics[1].to_string(), "U");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_with_fields() {
        let input = "struct Test\n  field: Type\n  field2: Type2\n";
        let tokens = lex_test(input);
        let (struct_decl, rest) =
            StructDecl::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        let ty = struct_decl.name;

        assert_eq!(ty.name, "Test");
        assert_eq!(struct_decl.fields.len(), 2);
        assert_eq!(
            struct_decl
                .fields
                .iter()
                .find(|field| field.name.name == "field")
                .unwrap()
                .ty
                .to_string(),
            "Type"
        );
        assert_eq!(
            struct_decl
                .fields
                .iter()
                .find(|field| field.name.name == "field2")
                .unwrap()
                .ty
                .to_string(),
            "Type2"
        );
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_instance_inline() {
        let input = "Test a: 1, b: 2, c: a + 4";
        let tokens = lex_test(input);
        let (struct_instance, rest) =
            StructInstance::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(struct_instance.name.name, "Test");
        assert_eq!(struct_instance.fields.len(), 3);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_instance_multiline() {
        let input = "Test\n  a: 1\n  b: 2\n  c: a + 4";
        let tokens = lex_test(input);
        let (struct_instance, rest) =
            StructInstance::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(struct_instance.name.name, "Test");
        assert_eq!(struct_instance.fields.len(), 3);
        assert_eq!(rest.len(), 0);
    }
}
