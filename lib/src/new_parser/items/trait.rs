use std::collections::BTreeMap;

use crate::{
    lexer::TokenType,
    new_parser::{engine::*, FunctionDecl, FunctionSig, TraitDecl},
};

use super::{function_decl, function_sig, indent, parse_type_inner};

enum FnDeclOrSig {
    Decl(FunctionDecl),
    Sig(FunctionSig),
}

pub fn r#trait(stream: Input) -> IResult<TraitDecl> {
    (
        TokenType::Keyword("trait".to_string()),
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

            TraitDecl {
                name,
                methods,
                signatures,
            }
        })
        .process(stream)
}

#[cfg(test)]
mod parse_trait {
    use crate::{
        new_parser::{lex_test, TraitDecl},
        Config,
    };

    use super::*;

    #[test]
    fn test_parse_trait() {
        let tokens = lex_test(
            r#"trait Foo
    bar = a -> a
    baz : Int
    @selfinject = a -> a
"#,
        );
        let config = Config::default();

        let (rest, trait_decl) = r#trait.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(trait_decl.name.name, "Foo");
        assert_eq!(trait_decl.methods.len(), 2);
        assert_eq!(trait_decl.signatures.len(), 1);
        assert_eq!(rest.len(), 0);
    }
}
