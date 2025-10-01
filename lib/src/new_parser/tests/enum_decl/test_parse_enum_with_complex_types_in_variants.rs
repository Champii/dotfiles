use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

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
