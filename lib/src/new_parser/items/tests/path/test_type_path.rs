use crate::new_parser::items::*;
use crate::new_parser::*;
use crate::Config;

#[test]
fn test_type_path() {
    let input = "ident::Type::Type";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, ident) = type_path.process(ParseCtx::from(&tokens, &config)).unwrap();

    assert_eq!(ident.path.len(), 3);
    assert_eq!(rest.len(), 0);
}
