use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::*;
use crate::Config;

#[test]
fn mut_ident_pattern() {
    let input = "mut a";
    let tokens = lex_test(input);
    let config = Config::default();

    let (rest, pattern) = pattern.process(ParseCtx::from(&tokens, &config)).unwrap();

    assert_eq!(
        pattern,
        Pattern {
            binding: None,
            kind: PatternKind::Ident(IdentPattern {
                name: Ident {
                    name: "a".to_string(),
                    span: Span::default()
                },
                mut_: true,
            })
        }
    );

    assert_eq!(rest.len(), 0);
}
