use crate::new_parser::items::*;
use crate::new_parser::*;
use crate::Config;

#[test]
fn test_parse_array_type() {
    let input = "[A]";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, parse_type) = parse_type
        .process(ParseCtx::from(&tokens, &config))
        .unwrap();

    assert_eq!(parse_type.to_string(), "[A]");
    assert_eq!(rest.len(), 0);
}
