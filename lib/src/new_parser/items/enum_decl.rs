use crate::{
    lexer::TokenType,
    new_parser::{engine::*, EnumDecl, EnumVariant, NamedFieldsOrTypesList, ParseTypeInner},
};

use super::{empty_lines, indent, parse_type_inner, struct_decl_field};

pub fn enum_decl(stream: Input) -> IResult<EnumDecl> {
    (
        TokenType::Keyword("enum".to_string()),
        parse_type_inner,
        TokenType::Eol.followed_by(empty_lines),
        indented(many(enum_variant)),
    )
        .map(|(_, name, _, variants)| EnumDecl { name, variants })
        .process(stream)
}

pub fn enum_variant(stream: Input) -> IResult<EnumVariant> {
    (
        indent,
        parse_type_inner,
        TokenType::Eol.followed_by(empty_lines),
        indented(named_fields_or_types_list).opt(),
    )
        .map(|(_, name, _, fields_opt)| {
            if !name.generics.is_empty() {
                EnumVariant {
                    name: ParseTypeInner {
                        span: name.span,
                        name: name.name,
                        generics: Vec::new(),
                    },
                    fields: NamedFieldsOrTypesList::TypesList(name.generics),
                }
            } else if let Some(fields) = fields_opt {
                EnumVariant { name, fields }
            } else {
                EnumVariant {
                    name,
                    fields: NamedFieldsOrTypesList::NamedFields(Vec::new()),
                }
            }
        })
        .process(stream)
}

pub fn named_fields_or_types_list(stream: Input) -> IResult<NamedFieldsOrTypesList> {
    many(struct_decl_field)
        .map(NamedFieldsOrTypesList::NamedFields)
        .process(stream)
}

#[cfg(test)]
mod parse_enum {
    use super::*;
    use crate::{new_parser::items::utils::lex_test, Config};

    #[test]
    fn test_parse_enum_with_no_variants() {
        let input = "enum EmptyEnum\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, enum_decl) = enum_decl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(enum_decl.name.to_string(), "EmptyEnum");
        assert_eq!(enum_decl.variants.len(), 0);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_enum_with_single_variant() {
        let input = "enum SingleVariant\n    OnlyVariant\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, enum_decl) = enum_decl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(enum_decl.name.to_string(), "SingleVariant");
        assert_eq!(enum_decl.variants.len(), 1);
        assert_eq!(enum_decl.variants[0].name.to_string(), "OnlyVariant");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_enum_with_struct_like_variant() {
        let input =
            "enum StructLikeEnum\n    Variant\n        field1: Type1\n        field2: Type2\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, enum_decl) = enum_decl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(enum_decl.name.to_string(), "StructLikeEnum");
        assert_eq!(enum_decl.variants.len(), 1);
        let variant = &enum_decl.variants[0];
        assert_eq!(variant.name.to_string(), "Variant");

        if let NamedFieldsOrTypesList::NamedFields(fields) = &variant.fields {
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name.to_string(), "field1");
            assert_eq!(fields[1].name.to_string(), "field2");
        } else {
            panic!("Expected NamedFields for struct-like variant");
        }

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_enum_with_tuple_like_variant() {
        let input = "enum TupleLikeEnum\n    Variant Type1, Type2\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, enum_decl) = enum_decl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(enum_decl.name.to_string(), "TupleLikeEnum");
        assert_eq!(enum_decl.variants.len(), 1);
        let variant = &enum_decl.variants[0];
        assert_eq!(variant.name.to_string(), "Variant");

        if let NamedFieldsOrTypesList::TypesList(types) = &variant.fields {
            assert_eq!(types.len(), 2);
            assert_eq!(types[0].to_string(), "Type1");
            assert_eq!(types[1].to_string(), "Type2");
        } else {
            panic!("Expected TypesList for tuple-like variant");
        }

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_enum_with_comments_and_whitespace() {
        let input = "enum CommentedEnum\n    // This is a variant\n    Variant1\n\n    /* Another variant */\n    Variant2\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, enum_decl) = enum_decl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(enum_decl.name.to_string(), "CommentedEnum");
        assert_eq!(enum_decl.variants.len(), 2);
        assert_eq!(enum_decl.variants[0].name.to_string(), "Variant1");
        assert_eq!(enum_decl.variants[1].name.to_string(), "Variant2");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_enum_with_multiple_struct_like_variants() {
        let input = "enum MultiStructEnum\n    Variant1\n        field1: Type1\n    Variant2\n        field2: Type2\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, enum_decl) = enum_decl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(enum_decl.name.to_string(), "MultiStructEnum");
        assert_eq!(enum_decl.variants.len(), 2);

        // Vérification du premier variant
        let variant1 = &enum_decl.variants[0];
        assert_eq!(variant1.name.to_string(), "Variant1");
        if let NamedFieldsOrTypesList::NamedFields(fields) = &variant1.fields {
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].name.to_string(), "field1");
        } else {
            panic!("Expected NamedFields for Variant1");
        }

        // Vérification du deuxième variant
        let variant2 = &enum_decl.variants[1];
        assert_eq!(variant2.name.to_string(), "Variant2");
        if let NamedFieldsOrTypesList::NamedFields(fields) = &variant2.fields {
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].name.to_string(), "field2");
        } else {
            panic!("Expected NamedFields for Variant2");
        }

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_enum_with_empty_lines() {
        let input = "enum SpacedEnum\n\n    Variant1\n\n    Variant2\n\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, enum_decl) = enum_decl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(enum_decl.name.to_string(), "SpacedEnum");
        assert_eq!(enum_decl.variants.len(), 2);
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_enum_with_complex_types_in_variants() {
        let input = "enum ComplexEnum T\n    Variant1 Option Result T, E\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, enum_decl) = enum_decl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(enum_decl.name.to_string(), "ComplexEnum T");
        assert_eq!(enum_decl.variants.len(), 1);
        let variant = &enum_decl.variants[0];

        if let NamedFieldsOrTypesList::TypesList(types) = &variant.fields {
            assert_eq!(types.len(), 1);
            assert_eq!(types[0].to_string(), "Option Result T, E");
        } else {
            panic!("Expected TypesList for complex variant");
        }

        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_enum() {
        let input =
            "enum Type\n    Variant1\n    Variant2 T, U\n    StructLike\n        field: Type\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, enum_decl) = enum_decl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(enum_decl.name.to_string(), "Type");
        assert_eq!(enum_decl.variants.len(), 3);
        assert_eq!(rest.len(), 0);
    }
}
