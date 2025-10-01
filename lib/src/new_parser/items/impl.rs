use std::collections::BTreeMap;

use crate::{
    lexer::TokenType,
    new_parser::{engine::*, FunctionDecl, FunctionSig, Impl},
};

use super::{empty_lines, function_decl, function_sig, indent, parse_type_inner};

enum FnDeclOrSig {
    Decl(FunctionDecl),
    Sig(FunctionSig),
}

pub fn r#impl(stream: Input) -> IResult<Impl> {
    (
        TokenType::Keyword("impl".to_string()),
        parse_type_inner,
        preceded(TokenType::Keyword("for".to_string()), parse_type_inner).opt(),
        TokenType::Eol.followed_by(empty_lines),
        indented(many(
            preceded(
                indent,
                function_decl
                    .map(FnDeclOrSig::Decl)
                    .or(function_sig.map(FnDeclOrSig::Sig)),
            )
            .followed_by(empty_lines),
        )),
    )
        .map(|(_, name, for_, _, items)| {
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
                for_,
                methods,
                signatures,
            }
        })
        .process(stream)
}
