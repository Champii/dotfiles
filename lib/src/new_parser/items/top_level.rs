use crate::new_parser::{
    engine::*,
    items::{primitives::indent, utils::empty_lines},
    Ident, TopLevel, TopLevelKind,
};

use super::{enum_decl, function_decl, macro_decl, macro_invoc, struct_decl};

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
            })),
        empty_lines,
    )
        .map(|(_, _, top_level, _)| top_level)
        .process(stream)
}
