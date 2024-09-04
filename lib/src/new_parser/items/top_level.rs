use crate::{
    lexer::TokenType,
    new_parser::{
        engine::*,
        items::{primitives::indent, utils::empty_lines},
        Ident, ModuleDecl, TopLevel, TopLevelKind,
    },
};

use super::{
    enum_decl, function_decl, function_sig, ident, macro_decl, macro_invoc, module, operator,
    operator_token, parse_type, parse_type_inner, path, primitives, r#impl, r#trait, struct_decl,
    stuck_operator_token,
};

pub fn top_level(stream: Input) -> IResult<TopLevel> {
    (
        empty_lines,
        indent,
        function_decl
            .map(|fn_decl| TopLevel {
                ident: fn_decl.name.clone(),
                kind: TopLevelKind::FunctionDecl(fn_decl),
            })
            .or(struct_decl.map(|struct_decl| TopLevel {
                ident: Ident::default(), // FIXME
                kind: TopLevelKind::StructDecl(struct_decl),
            }))
            .or(macro_decl.map(|macro_decl| TopLevel {
                ident: macro_decl.name.clone(),
                kind: TopLevelKind::MacroDecl(macro_decl),
            }))
            .or(macro_invoc.map(|macro_invoc| TopLevel {
                ident: macro_invoc.name.clone(),
                kind: TopLevelKind::MacroInvoc(macro_invoc),
            }))
            .or(enum_decl.map(|enum_decl| TopLevel {
                ident: Ident::default(), // FIXME
                kind: TopLevelKind::EnumDecl(enum_decl),
            }))
            .or(r#trait.map(|trait_decl| TopLevel {
                ident: Ident::default(), // FIXME
                kind: TopLevelKind::TraitDecl(trait_decl),
            }))
            .or(r#impl.map(|impl_decl| TopLevel {
                ident: Ident::default(), // FIXME
                kind: TopLevelKind::Impl(impl_decl),
            }))
            .or(infix_operator_decl.map(|(precedence, name)| TopLevel {
                ident: Ident::default(), // FIXME
                kind: TopLevelKind::InfixOperator(precedence, name),
            }))
            .or(
                preceded(TokenType::Keyword("extern".to_string()), function_sig).map(|sig| {
                    TopLevel {
                        ident: Ident::default(), // FIXME
                        kind: TopLevelKind::Extern(sig),
                    }
                }),
            )
            .or(module.map(|mod_decl| TopLevel {
                ident: Ident::default(), // FIXME
                kind: TopLevelKind::Module(ModuleDecl(mod_decl)),
            }))
            .or((
                TokenType::Keyword("type".to_string()),
                parse_type_inner,
                TokenType::Equal,
                parse_type,
                TokenType::Eol,
            )
                .map(|(_, name, _, ty, _)| TopLevel {
                    ident: Ident::default(), // FIXME
                    kind: TopLevelKind::NewType(name, ty),
                }))
            /* .or(comment.map(|comment| TopLevel {
                ident: Ident::default(), // FIXME
                kind: TopLevelKind::Comment(comment),
            })) */
            .or(
                (TokenType::Operator(">".to_string()), path, TokenType::Eol).map(|(_, path, _)| {
                    TopLevel {
                        ident: Ident::default(), // FIXME
                        kind: TopLevelKind::Import(path),
                    }
                }),
            )
            .or(
                (TokenType::Operator("<".to_string()), path, TokenType::Eol).map(|(_, path, _)| {
                    TopLevel {
                        ident: Ident::default(), // FIXME
                        kind: TopLevelKind::Export(path),
                    }
                }),
            )
            .or(macro_invoc.map(|macro_invoc| TopLevel {
                ident: macro_invoc.name.clone(),
                kind: TopLevelKind::MacroInvoc(macro_invoc),
            }))
            .or(function_sig.map(|function_sig| TopLevel {
                ident: function_sig.name.clone(),
                kind: TopLevelKind::FunctionSig(function_sig),
            })),
        empty_lines,
    )
        .map(|(_, _, top_level, _)| top_level)
        .process(stream)
}

pub fn infix_operator_decl(stream: Input) -> IResult<(u8, String)> {
    (
        TokenType::Keyword("infix".to_string()),
        primitives::int.assert(|precedence| *precedence <= 9),
        operator,
        TokenType::Eol,
    )
        .map(|(_, precedence, name, _)| (precedence as u8, name.value))
        .process(stream)
}

#[cfg(test)]
mod parse_top_level {
    use crate::{
        new_parser::{lex_test, lex_test_toplevel},
        Config,
    };

    use super::*;

    #[test]
    fn parse_infix_operator() {
        let input = "infix 5 |>\n";
        let tokens = lex_test(input);
        let config = Config::default();

        let (rest, (precedence, name)) = infix_operator_decl
            .process(ParseCtx::from(&tokens, &config))
            .unwrap();

        assert_eq!(precedence, 5);
        assert_eq!(name, "|>".to_string());
        assert_eq!(rest.len(), 0);
    }

    #[test]
    fn extern_sig() {
        let input = "extern toto : Toto -> Tata\n";
        let tokens = lex_test_toplevel(input);
        let config = Config::default();

        let (rest, top_level) = top_level.process(ParseCtx::from(&tokens, &config)).unwrap();

        assert_eq!(rest.len(), 0);
    }
}
