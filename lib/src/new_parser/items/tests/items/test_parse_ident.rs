use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn test_parse_ident() {
    let tokens = lex_test("foo");
    let config = Config::default();

    let (parse_ctx, ident) = ident(ParseCtx::from(&tokens, &config)).unwrap();

    assert_eq!(
        ident,
        Ident {
            name: "foo".to_string(),
            span: Span {
                start: 0,
                end: 3,
                file_path: PathBuf::new(),
            },
        }
    );

    assert_eq!(parse_ctx.len(), 0);
}
