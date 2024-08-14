use std::collections::BTreeMap;

use crate::{
    lexer::TokenType,
    new_parser::{engine::*, FunctionDecl, FunctionSig, Impl},
};

use super::{function_decl, function_sig, indent, parse_type_inner};

enum FnDeclOrSig {
    Decl(FunctionDecl),
    Sig(FunctionSig),
}

pub fn r#impl(stream: Input) -> IResult<Impl> {
    (
        TokenType::Keyword("impl".to_string()),
        parse_type_inner,
        TokenType::Eol,
        indented(many(preceded(
            indent,
            function_decl
                .map(FnDeclOrSig::Decl)
                .or(function_sig.map(FnDeclOrSig::Sig)),
        ))),
    )
        .map(|(_, name, _, items)| {
            let mut methods = BTreeMap::new();
            let mut signatures = BTreeMap::new();

            for item in items {
                match item {
                    FnDeclOrSig::Decl(decl) => {
                        methods.insert(decl.name.clone(), decl);
                    }
                    FnDeclOrSig::Sig(sig) => {
                        signatures.insert(sig.name.clone(), sig);
                    }
                }
            }

            Impl {
                name,
                methods,
                signatures,
            }
        })
        .process(stream)
}

#[cfg(test)]
mod parse_impl {
    use crate::{
        new_parser::{lex_test, ParseCtx},
        Config,
    };

    use super::*;

    #[test]
    fn test_parse_impl() {
        let input = "impl Test\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, r#impl) = r#impl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(r#impl.name.to_string(), "Test");
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn test_parse_impl_with_methods() {
        let input = "impl Test\n    new = -> lol\n    @add = -> a\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, r#impl) = r#impl.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(r#impl.name.to_string(), "Test");
        assert_eq!(r#impl.methods.len(), 2);
        assert_eq!(
            r#impl
                .methods
                .iter()
                .find(|(k, _v)| k.name == "new")
                .unwrap()
                .1
                .name
                .name,
            "new"
        );
        assert_eq!(rest.len(), 0);
    }
}
