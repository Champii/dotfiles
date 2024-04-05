use std::collections::BTreeMap;

use crate::{
    ast::{Expression, Ident, IdentifierPath, Instance},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parse_ctx::ParseCtx,
        util::{expect_token, ignore_empty_lines, parse_vec_of},
        Parsable,
    },
};

impl Parsable for Instance {
    fn parse<'a>(
        tokens: &'a [Token],
        parse_ctx: &mut ParseCtx,
    ) -> Result<(Self, &'a [Token]), Diagnostics> {
        let (identifier_path, remaining_tokens) = IdentifierPath::parse(tokens, parse_ctx)?;

        if remaining_tokens.is_empty() {
            return Ok((
                Instance {
                    name: identifier_path,
                    fields: BTreeMap::new(),
                },
                remaining_tokens,
            ));
        }

        if let TokenType::Eol = remaining_tokens[0].token_type {
            let (fields, new_remaining_tokens) = parse_ctx.indent_block(|parse_ctx| {
                InstanceBlock::parse(&remaining_tokens[1..], parse_ctx)
            })?;

            if fields.fields.is_empty() {
                return Ok((
                    Instance {
                        name: identifier_path,
                        fields: BTreeMap::new(),
                    },
                    remaining_tokens,
                ));
            }

            Ok((
                Instance {
                    name: identifier_path,
                    fields: fields.fields.into_iter().collect(),
                },
                new_remaining_tokens,
            ))
        } else {
            let (fields, remaining_tokens) = parse_vec_of::<(Ident, Expression)>(
                remaining_tokens,
                Some(TokenType::Coma),
                parse_ctx,
            )?;

            Ok((
                Instance {
                    name: identifier_path,
                    fields: fields.into_iter().collect(),
                },
                remaining_tokens,
            ))
        }
    }
}

#[derive(Debug)]
struct InstanceBlock {
    fields: Vec<(Ident, Expression)>,
}

impl Parsable for InstanceBlock {
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
            InstanceBlock {
                fields: fields.into_iter().collect(),
            },
            remaining_tokens,
        ))
    }
}

#[cfg(test)]
mod instance {
    use crate::{
        ast::Instance,
        parser::{util::lex_test, Parsable, ParseCtx},
        Config,
    };

    #[test]
    fn test_parse_enum_instance() {
        let input = "Type::Variant1";
        let tokens = lex_test(input);
        let (enum_instance, rest) =
            Instance::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(enum_instance.name.to_string(), "Type::Variant1");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_instance_inline() {
        let input = "Test a: 1, b: 2, c: a + 4";
        let tokens = lex_test(input);
        let (struct_instance, rest) =
            Instance::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(struct_instance.name.to_string(), "Test");
        assert_eq!(struct_instance.fields.len(), 3);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_instance_multiline() {
        let input = "Test\n  a: 1\n  b: 2\n  c: a + 4";
        let tokens = lex_test(input);
        let (struct_instance, rest) =
            Instance::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(struct_instance.name.to_string(), "Test");
        assert_eq!(struct_instance.fields.len(), 3);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_instance_empty_args() {
        let input = "Test";
        let tokens = lex_test(input);
        let (struct_instance, rest) =
            Instance::parse(&tokens, &mut ParseCtx::new(&Config::default())).unwrap();

        assert_eq!(struct_instance.name.to_string(), "Test");
        assert_eq!(struct_instance.fields.len(), 0);
        assert_eq!(rest.len(), 0);
    }
}
