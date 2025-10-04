use crate::new_parser::items::*;
use crate::new_parser::*;
use crate::Config;

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
