use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;

#[test]
fn test_parse_macro_entry() {
    let input = "$a:ident =>\n        statement";
    let tokens = lex(input);
    // let tokens = &tokens[1..]; // skip the Indent(0)
    let config = Config::default();

    let mut parse_ctx = ParseCtx::from(&tokens, &config);

    // Hack to force the indent step
    parse_ctx.indent_step = 4;

    let (rest, macro_entry) = macro_entry(parse_ctx).unwrap();

    assert_eq!(macro_entry.defs.len(), 1);
    assert_eq!(rest.len(), 0);
}
