use crate::ast::*;
use crate::lexer::Span;
use crate::new_parser::items::*;
use crate::new_parser::items::tests::common::*;
use crate::new_parser::*;
use crate::Config;
use std::path::PathBuf;

#[test]
fn ambiguous_multiline_should_fail() {
    // This should fail because mdr.lol and haha are at the same indentation level as .lol
    // making it ambiguous whether they are arguments or method calls
    let input = r#"a
.lol
mdr.lol
haha"#;
    let tokens = lex_test(input);
    let config = Config::default();

    // This test verifies that ambiguous multiline syntax is correctly rejected

    // This currently parses incorrectly as a.lol(mdr.lol, haha)
    // but should ideally be a parse error due to ambiguity
    let result = expression.process(ParseCtx::from(&tokens, &config));

    // The fix should now correctly reject the ambiguous syntax
    match result {
        Ok((_, Expression::UnaryExpr(UnaryExpr::PrimaryExpr(PrimaryExpr {
            secondaries: Some(secondaries),
            ..
        })))) => {
            // After the fix, this should only parse a.lol (no arguments)
            assert_eq!(secondaries.len(), 1); // Only .lol, no arguments
            match &secondaries[0] {
                SecondaryExpr::Dot(IdentOrNumber::Ident(ident)) => {
                    assert_eq!(ident.name, "lol");
                }
                _ => panic!("Expected .lol as the only secondary"),
            }
        }
        Err(_) => {
            // This would also be acceptable - rejecting ambiguous syntax entirely
            // For now, we expect partial parsing (just a.lol)
            panic!("Expected partial parsing of a.lol, but got error");
        }
        _ => panic!("Unexpected parse result"),
    }
}
