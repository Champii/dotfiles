use crate::new_parser::{engine::*, StructDecl, StructDeclField, TokenType};

use super::{expression, ident, indent, parse_type, parse_type_inner};

pub fn struct_decl(stream: Input) -> IResult<StructDecl> {
    (
        TokenType::Keyword("struct".to_string()),
        parse_type_inner,
        TokenType::Eol,
        indented(many(struct_decl_field)),
    )
        .map(|(_, name, _, fields)| StructDecl { name, fields })
        .process(stream)
}

pub fn struct_decl_field(stream: Input) -> IResult<StructDeclField> {
    (
        indent,
        TokenType::Operator("<".to_string()).opt(),
        ident,
        TokenType::Colon,
        parse_type,
        (TokenType::Equal, expression).opt(),
        TokenType::Eol,
    )
        .map(|(_, public, name, _, ty, expr_opt, _)| StructDeclField {
            name,
            ty,
            public: public.is_some(),
            default: expr_opt.map(|(_, expr)| expr),
        })
        .process(stream)
}
