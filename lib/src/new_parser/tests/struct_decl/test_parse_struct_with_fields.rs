use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

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
