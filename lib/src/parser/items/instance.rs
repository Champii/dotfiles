use std::collections::BTreeMap;

use crate::{
    ast::{Expression, Ident, IdentifierPath, Instance},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parse_ctx::ParseCtx,
        util::{parse_indented_vec_of, parse_vec_of},
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
            let (fields, new_remaining_tokens, _) = parse_indented_vec_of::<(Ident, Expression)>(
                &remaining_tokens[1..],
                parse_ctx,
                true,
            )?;

            if fields.is_empty() {
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
                    fields: fields.into_iter().collect(),
                },
                new_remaining_tokens,
            ))
        } else {
            let (fields, remaining_tokens, _diags) = parse_vec_of::<(Ident, Expression)>(
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
        let input = "Test\n    a: 1\n    b: 2\n    c: a + 4";
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
