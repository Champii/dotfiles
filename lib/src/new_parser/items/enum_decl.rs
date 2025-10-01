use crate::{
    lexer::TokenType,
    new_parser::{engine::*, EnumDecl, EnumVariant, NamedFieldsOrTypesList, ParseTypeInner},
};

use super::{empty_lines, indent, parse_type_inner, struct_decl_field};

pub fn enum_decl(stream: Input) -> IResult<EnumDecl> {
    (
        TokenType::Keyword("enum".to_string()),
        parse_type_inner,
        TokenType::Eol.followed_by(empty_lines),
        indented(many(enum_variant)),
    )
        .map(|(_, name, _, variants)| EnumDecl { name, variants })
        .process(stream)
}

pub fn enum_variant(stream: Input) -> IResult<EnumVariant> {
    (
        indent,
        parse_type_inner,
        TokenType::Eol.followed_by(empty_lines),
        indented(named_fields_or_types_list).opt(),
    )
        .map(|(_, name, _, fields_opt)| {
            if !name.generics.is_empty() {
                EnumVariant {
                    name: ParseTypeInner {
                        span: name.span,
                        name: name.name,
                        generics: Vec::new(),
                    },
                    fields: NamedFieldsOrTypesList::TypesList(name.generics),
                }
            } else if let Some(fields) = fields_opt {
                EnumVariant { name, fields }
            } else {
                EnumVariant {
                    name,
                    fields: NamedFieldsOrTypesList::NamedFields(Vec::new()),
                }
            }
        })
        .process(stream)
}

pub fn named_fields_or_types_list(stream: Input) -> IResult<NamedFieldsOrTypesList> {
    many(struct_decl_field)
        .map(NamedFieldsOrTypesList::NamedFields)
        .process(stream)
}
