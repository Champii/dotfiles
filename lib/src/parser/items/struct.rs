use crate::{
    ast::{Expression, Ident, ParseType, ParseTypeInner, StructDecl, StructDeclField},
    diagnostic::Diagnostics,
    lexer::{Token, TokenType},
    parser::{
        parse_ctx::ParseCtx,
        util::{expect_token, parse_indented_vec_of},
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

        let remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;

        let (fields, mut remaining_tokens, _) =
            parse_indented_vec_of(remaining_tokens, parse_ctx, true)?;

        if !fields.is_empty() {
            remaining_tokens = expect_token(remaining_tokens, TokenType::Eol)?;
        }

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

        if remaining_tokens.is_empty() {
            return Ok((
                StructDeclField {
                    name: ident,
                    ty: parse_type,
                    public,
                    default: None,
                },
                remaining_tokens,
            ));
        }

        let (default, remaining_tokens) = if TokenType::Equal == remaining_tokens[0].token_type {
            let (expression, remaining_tokens) =
                Expression::parse(&remaining_tokens[1..], parse_ctx)?;

            (Some(expression), remaining_tokens)
        } else {
            (None, remaining_tokens)
        };

        Ok((
            StructDeclField {
                name: ident,
                ty: parse_type,
                public,
                default,
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
}
