use crate::new_parser::{
    engine::*,
    items::{primitives::indent, utils::empty_lines},
    TopLevel, TopLevelKind,
};

use super::function_decl;

pub fn top_level(stream: Input) -> IResult<TopLevel> {
    (
        empty_lines,
        indent,
        (
            /* seek(TokenType::Keyword("macro".to_string()), parse_macro_decl)
            .map(|macro_decl| TopLevel {
                ident: macro_decl.name.clone(),
                kind: TopLevelKind::MacroDecl(macro_decl),
            }) */
            /* .or(
                seek(TokenType::Keyword("struct".to_string()), parse_struct_decl).map(|struct_decl| {
                    TopLevel {
                        ident: Ident::default(), // FIXME
                        kind: TopLevelKind::StructDecl(struct_decl),
                    }
                }),
            )
            .or(
                seek(TokenType::Keyword("enum".to_string()), parse_enum_decl).map(|enum_decl| {
                    TopLevel {
                        ident: Ident::default(), // FIXME
                        kind: TopLevelKind::EnumDecl(enum_decl),
                    }
                }),
            )
            .or(
                seek(TokenType::Keyword("trait".to_string()), parse_trait_decl).map(|trait_decl| {
                    TopLevel {
                        ident: Ident::default(), // FIXME
                        kind: TopLevelKind::TraitDecl(trait_decl),
                    }
                }),
            )
            .or(
                seek(TokenType::Keyword("impl".to_string()), parse_impl).map(|impl_| {
                    TopLevel {
                        ident: Ident::default(), // FIXME
                        kind: TopLevelKind::Impl(impl_),
                    }
                }),
            )
            .or(seek(
                TokenType::Keyword("infix".to_string()),
                parse_infix_operator,
            )
            .map(|(precedence, op)| {
                TopLevel {
                    ident: Ident::default(), // FIXME
                    kind: TopLevelKind::InfixOperator(precedence, op),
                }
            }))
            .or(
                seek(TokenType::Keyword("mod".to_string()), parse_module).map(|module| TopLevel {
                    ident: module.name.clone().unwrap_or_default(),
                    kind: TopLevelKind::Module(ModuleDecl(module)),
                }),
            )
            .or(
                seek(TokenType::Keyword("extern".to_string()), parse_function_sig).map(
                    |function_sig| TopLevel {
                        ident: function_sig.name.clone(),
                        kind: TopLevelKind::Extern(function_sig),
                    },
                ),
            )
            .or(
                seek(TokenType::Keyword("type".to_string()), parse_new_type).map(
                    |(parse_type_inner, parse_type)| {
                        TopLevel {
                            ident: Ident::default(), // FIXME
                            kind: TopLevelKind::NewType(parse_type_inner, parse_type),
                        }
                    },
                ),
            ) */
            function_decl,
        ),
        empty_lines,
    )
        .map(|(_, _, (function_decl,), _)| TopLevel {
            ident: function_decl.name.clone(),
            kind: TopLevelKind::FunctionDecl(function_decl),
        })
        .process(stream)
}
