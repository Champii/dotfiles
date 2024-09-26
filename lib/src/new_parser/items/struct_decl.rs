use crate::new_parser::{engine::*, StructDecl, StructDeclField, TokenType};

use super::{empty_lines, expression, ident, indent, parse_type, parse_type_inner};

pub fn struct_decl(stream: Input) -> IResult<StructDecl> {
    (
        TokenType::Keyword("struct".to_string()),
        parse_type_inner,
        TokenType::Eol.followed_by(empty_lines),
        indented(many(struct_decl_field.followed_by(empty_lines))),
    )
        .map(|(_, name, _, fields)| StructDecl { name, fields })
        .process(stream)
}

pub fn struct_decl_field(stream: Input) -> IResult<StructDeclField> {
    (
        indent,
        TokenType::Operator("<".to_string()).opt(),
        ident,
        TokenType::Colon,
        parse_type,
        (TokenType::Equal, expression).opt(),
        TokenType::Eol,
    )
        .map(|(_, public, name, _, ty, expr_opt, _)| StructDeclField {
            name,
            ty,
            public: public.is_some(),
            default: expr_opt.map(|(_, expr)| expr),
        })
        .process(stream)
}

#[cfg(test)]
mod parse_struct {
    use crate::new_parser::items::utils::lex_test;
    use crate::Config;

    use super::*;

    #[test]
    fn test_parse_struct() {
        let input = "struct Test\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, struct_decl) = struct_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(struct_decl.name.name, "Test");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_with_generics() {
        let input = "struct Test T, U\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, struct_decl) = struct_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        let ty = struct_decl.name;

        assert_eq!(ty.name, "Test");
        assert_eq!(ty.generics.len(), 2);
        assert_eq!(ty.generics[0].to_string(), "T");
        assert_eq!(ty.generics[1].to_string(), "U");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_struct_with_fields() {
        let input = "struct Test\n    field: Type\n    field2: Type2\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, struct_decl) = struct_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

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
    fn test_parse_struct_with_fields_empty_lines() {
        let input = "struct Test\n\n    field: Type\n\n    field2: Type2\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, struct_decl) = struct_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        let ty = struct_decl.name;

        assert_eq!(ty.name, "Test");
        assert_eq!(struct_decl.fields.len(), 2);
        assert_eq!(rest.len(), 0);
    }
}
