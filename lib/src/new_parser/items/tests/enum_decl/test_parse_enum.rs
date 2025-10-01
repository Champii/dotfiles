use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

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
