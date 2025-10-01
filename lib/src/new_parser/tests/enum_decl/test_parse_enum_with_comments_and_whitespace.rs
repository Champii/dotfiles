use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

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
